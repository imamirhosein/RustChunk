#!/usr/bin/env python3
"""Extract and compare Minecraft 1.20.1 fluid tags from its official client/server JARs.

The full normalized bundle remains private under .local. Only a compact report
is uploaded to CI, including the legacy fluid-tag directory layout.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import sys
import zipfile

try:
    from .prepare_minecraft_1_20_1 import PrepareError, digest_file, load_target_lock
    from .export_minecraft_1_20_1_block_tags import _validate_tag_file
except ImportError:
    from prepare_minecraft_1_20_1 import PrepareError, digest_file, load_target_lock
    from export_minecraft_1_20_1_block_tags import _validate_tag_file


VERSION = "1.20.1"
TAG_PREFIXES = (
    "data/minecraft/tags/fluids/",
    "data/minecraft/tags/fluid/",
)
FLUID_TAGS_READ_BY_STARLING = (
    "minecraft:water",
    "minecraft:lava",
    "minecraft:supports_frogspawn",
    "minecraft:supports_lily_pad",
    "minecraft:supports_sugar_cane_adjacently",
    "minecraft:bubble_column_can_occupy",
)


def read_fluid_tag_bundle(jar_path: Path) -> tuple[str, dict[str, str]]:
    """Read the single target fluid-tag root into {'minecraft:<path>': raw JSON}."""
    try:
        with zipfile.ZipFile(jar_path) as archive:
            files = archive.namelist()
            matching = [
                prefix for prefix in TAG_PREFIXES
                if any(name.startswith(prefix) and name.endswith(".json") for name in files)
            ]
            if not matching:
                raise PrepareError(
                    f"No Minecraft {VERSION} fluid tags found in {jar_path.name}; "
                    f"checked roots {TAG_PREFIXES}"
                )
            if len(matching) > 1:
                raise PrepareError(
                    f"{jar_path.name}: ambiguous Minecraft {VERSION} fluid-tag roots: {matching}"
                )
            prefix = matching[0]
            bundle: dict[str, str] = {}
            for member in sorted(
                name for name in files
                if name.startswith(prefix) and name.endswith(".json")
            ):
                relative = member[len(prefix):-5]
                path = PurePosixPath(relative)
                if not relative or path.is_absolute() or ".." in path.parts or "\\" in relative:
                    raise PrepareError(f"Unsafe fluid-tag path in {jar_path.name}: {member}")
                identifier = f"minecraft:{relative}"
                if identifier in bundle:
                    raise PrepareError(f"Duplicate fluid tag identifier: {identifier}")
                try:
                    content = archive.read(member).decode("utf-8")
                except (KeyError, UnicodeDecodeError) as exc:
                    raise PrepareError(f"Could not read UTF-8 fluid tag {member}: {exc}") from exc
                _validate_tag_file(identifier, content)
                bundle[identifier] = content
    except zipfile.BadZipFile as exc:
        raise PrepareError(f"Not a valid JAR archive: {jar_path}") from exc
    if not bundle:
        raise PrepareError(f"No Minecraft {VERSION} fluid tags found in {jar_path}")
    return prefix, bundle


def summarize_fluid_tags(bundle: dict[str, str], prefix: str) -> dict:
    files = sorted(bundle.items())
    entry_count = 0
    replace_count = 0
    optional_count = 0
    namespaces: dict[str, int] = {}
    for name, text in files:
        value = _validate_tag_file(name, text)
        entry_count += len(value["values"])
        replace_count += int(value.get("replace", False))
        optional_count += sum(
            int(isinstance(item, dict) and item.get("required") is False)
            for item in value["values"]
        )
        namespace = name.split(":", 1)[0]
        namespaces[namespace] = namespaces.get(namespace, 0) + 1
    canonical = json.dumps(dict(files), ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    present = {name: name in bundle for name in FLUID_TAGS_READ_BY_STARLING}
    return {
        "status": "completed",
        "minecraft_version": VERSION,
        "resource_prefix": prefix,
        "tag_file_count": len(files),
        "tag_entry_count": entry_count,
        "replace_tag_count": replace_count,
        "optional_entry_count": optional_count,
        "tag_namespaces": dict(sorted(namespaces.items())),
        "tag_names": [name for name, _ in files],
        "starling_fluid_tags": present,
        "missing_starling_fluid_tags": [name for name, exists in present.items() if not exists],
        "bundle_sha256": hashlib.sha256(canonical).hexdigest(),
    }


def export(args) -> dict:
    prepared = args.prepared_dir.resolve()
    client_jar = prepared / "downloads" / "client.jar"
    server_jar = prepared / "jars" / "server-game.jar"
    pins = load_target_lock()["minecraft"]["downloads"]
    if not client_jar.is_file() or digest_file(client_jar, "sha1") != pins["client"]:
        raise PrepareError("The client JAR is missing or does not match the reviewed 1.20.1 SHA-1 pin")
    if not server_jar.is_file():
        raise PrepareError(f"The extracted 1.20.1 server JAR is missing: {server_jar}")

    try:
        audit = json.loads((prepared / "audit-report.json").read_text(encoding="utf-8"))
        expected_server = audit["inputs"]["server_game_sha256"]
    except (OSError, json.JSONDecodeError, KeyError, TypeError) as exc:
        raise PrepareError(f"Missing verified server-game checksum in preparation report: {exc}") from exc
    actual_server = digest_file(server_jar, "sha256")
    if actual_server != expected_server:
        raise PrepareError(
            f"Extracted server JAR checksum mismatch: expected {expected_server}, got {actual_server}"
        )

    client_prefix, client = read_fluid_tag_bundle(client_jar)
    server_prefix, server = read_fluid_tag_bundle(server_jar)
    if client_prefix != server_prefix:
        raise PrepareError(f"Client/server fluid-tag roots differ: {client_prefix} vs {server_prefix}")
    if client != server:
        client_only = sorted(set(client) - set(server))[:10]
        server_only = sorted(set(server) - set(client))[:10]
        changed = sorted(k for k in set(client) & set(server) if client[k] != server[k])[:10]
        raise PrepareError(
            "Official 1.20.1 client/server fluid-tag resources differ "
            f"(client_only={client_only}, server_only={server_only}, changed={changed})"
        )

    summary = summarize_fluid_tags(server, server_prefix)
    output_dir = args.work_dir.resolve() / "fluid-tags"
    output_dir.mkdir(parents=True, exist_ok=True)
    bundle_path = output_dir / "fluid-tags.json"
    bundle_path.write_text(
        json.dumps(dict(sorted(server.items())), ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    summary.update({
        "bundle_bytes": bundle_path.stat().st_size,
        "bundle_file_sha256": digest_file(bundle_path, "sha256"),
        "client_jar_sha1": pins["client"],
        "server_game_sha256": actual_server,
        "private_bundle_path": "fluid-tags/fluid-tags.json",
        "note": "The complete fluid-tag bundle remains private under .local and is not uploaded.",
    })
    (output_dir / "fluid-tags-report.json").write_text(
        json.dumps(summary, indent=2) + "\n",
        encoding="utf-8",
    )
    return summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prepared-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/1.20.1-decompile"))
    args = parser.parse_args()
    try:
        result = export(args)
    except (OSError, ValueError, KeyError, PrepareError) as exc:
        print(f"Minecraft 1.20.1 fluid-tag export failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
