#!/usr/bin/env python3
"""Extract the exact 1.20.1 vanilla block-tag bundle for Starling's Rust world loader.

The client and server archives must contain the same block-tag resources. The
full bundle stays private under .local; CI uploads only counts and hashes.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import sys
import zipfile

try:
    from .prepare_minecraft_1_20_1 import PrepareError, digest_file, load_target_lock
except ImportError:
    from prepare_minecraft_1_20_1 import PrepareError, digest_file, load_target_lock


VERSION = "1.20.1"
TAG_PREFIXES = (
    "data/minecraft/tags/blocks/",
    "data/minecraft/tags/block/",
)


def _validate_tag_file(name: str, text: str) -> dict:
    try:
        value = json.loads(text)
    except json.JSONDecodeError as exc:
        raise PrepareError(f"Invalid JSON in block tag {name}: {exc}") from exc
    if not isinstance(value, dict):
        raise PrepareError(f"Block tag {name} must have a JSON object at its root")
    if "replace" in value and not isinstance(value["replace"], bool):
        raise PrepareError(f"Block tag {name} has a non-boolean 'replace' field")
    entries = value.get("values")
    if not isinstance(entries, list):
        raise PrepareError(f"Block tag {name} must contain a 'values' array")
    for index, entry in enumerate(entries):
        if isinstance(entry, str):
            identifier = entry
        elif isinstance(entry, dict):
            identifier = entry.get("id")
            if not isinstance(identifier, str):
                raise PrepareError(f"Block tag {name} entry {index} has no string id")
            required = entry.get("required")
            if required is not None and not isinstance(required, bool):
                raise PrepareError(f"Block tag {name} entry {index} has a non-boolean required flag")
        else:
            raise PrepareError(f"Block tag {name} entry {index} must be a string or object")
        if not identifier or ":" not in identifier:
            raise PrepareError(f"Block tag {name} entry {index} has an invalid identifier: {identifier!r}")
    return value


def read_block_tag_bundle(jar_path: Path) -> tuple[str, dict[str, str]]:
    """Read block tags as {'minecraft:<path>': original UTF-8 JSON text}."""
    try:
        with zipfile.ZipFile(jar_path) as archive:
            files = archive.namelist()
            matching_prefixes = [
                prefix for prefix in TAG_PREFIXES
                if any(name.startswith(prefix) and name.endswith(".json") for name in files)
            ]
            if not matching_prefixes:
                raise PrepareError(
                    f"No Minecraft {VERSION} block tags found in {jar_path.name}; "
                    f"checked roots {TAG_PREFIXES}"
                )
            if len(matching_prefixes) > 1:
                raise PrepareError(
                    f"{jar_path.name}: ambiguous 1.20.1 block-tag roots "
                    f"from {TAG_PREFIXES}, found {matching_prefixes}"
                )
            prefix = matching_prefixes[0]
            selected = sorted(
                name for name in files
                if name.startswith(prefix) and name.endswith(".json")
            )
            bundle: dict[str, str] = {}
            for member in selected:
                relative = member[len(prefix):-5]
                path = PurePosixPath(relative)
                if (
                    not relative
                    or path.is_absolute()
                    or ".." in path.parts
                    or "\\" in relative
                ):
                    raise PrepareError(f"Unsafe block tag path in {jar_path.name}: {member}")
                identifier = f"minecraft:{relative}"
                if identifier in bundle:
                    raise PrepareError(f"Duplicate block tag identifier: {identifier}")
                try:
                    content = archive.read(member).decode("utf-8")
                except (KeyError, UnicodeDecodeError) as exc:
                    raise PrepareError(f"Could not read UTF-8 block tag {member}: {exc}") from exc
                _validate_tag_file(identifier, content)
                bundle[identifier] = content
    except zipfile.BadZipFile as exc:
        raise PrepareError(f"Not a valid JAR archive: {jar_path}") from exc

    if not bundle:
        raise PrepareError(f"No Minecraft {VERSION} block tags found in {jar_path}")
    return prefix, bundle


def summarize_block_tags(bundle: dict[str, str], prefix: str) -> dict:
    files = sorted(bundle.items())
    entries = 0
    replace_count = 0
    optional_entries = 0
    namespaces: dict[str, int] = {}
    for tag_name, text in files:
        value = _validate_tag_file(tag_name, text)
        entries += len(value["values"])
        replace_count += int(value.get("replace", False))
        optional_entries += sum(
            int(isinstance(item, dict) and item.get("required") is False)
            for item in value["values"]
        )
        namespace = tag_name.split(":", 1)[0]
        namespaces[namespace] = namespaces.get(namespace, 0) + 1
    canonical = json.dumps(dict(files), ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    return {
        "status": "completed",
        "minecraft_version": VERSION,
        "resource_prefix": prefix,
        "tag_file_count": len(files),
        "tag_entry_count": entries,
        "replace_tag_count": replace_count,
        "optional_entry_count": optional_entries,
        "tag_namespaces": dict(sorted(namespaces.items())),
        "bundle_sha256": hashlib.sha256(canonical).hexdigest(),
    }


def export(args) -> dict:
    prepared_dir = args.prepared_dir.resolve()
    client_jar = prepared_dir / "downloads" / "client.jar"
    server_jar = prepared_dir / "jars" / "server-game.jar"
    lock = load_target_lock()["minecraft"]

    if not client_jar.is_file() or digest_file(client_jar, "sha1") != lock["downloads"]["client"]:
        raise PrepareError("The client JAR is missing or does not match the reviewed 1.20.1 SHA-1 pin")
    if not server_jar.is_file():
        raise PrepareError(f"The extracted 1.20.1 server JAR is missing: {server_jar}")
    audit_path = prepared_dir / "audit-report.json"
    try:
        audit = json.loads(audit_path.read_text(encoding="utf-8"))
        expected_server_sha256 = audit["inputs"]["server_game_sha256"]
    except (OSError, json.JSONDecodeError, KeyError, TypeError) as exc:
        raise PrepareError(f"Missing verified server-game checksum in {audit_path}: {exc}") from exc
    actual_server_sha256 = digest_file(server_jar, "sha256")
    if actual_server_sha256 != expected_server_sha256:
        raise PrepareError(
            "Extracted server JAR no longer matches the checksum recorded by preparation "
            f"(expected {expected_server_sha256}, got {actual_server_sha256})"
        )

    client_prefix, client = read_block_tag_bundle(client_jar)
    server_prefix, server = read_block_tag_bundle(server_jar)
    if client_prefix != server_prefix:
        raise PrepareError(
            f"Client/server block-tag roots differ: {client_prefix!r} vs {server_prefix!r}"
        )
    if client != server:
        client_only = sorted(set(client) - set(server))[:10]
        server_only = sorted(set(server) - set(client))[:10]
        changed = sorted(k for k in set(client) & set(server) if client[k] != server[k])[:10]
        raise PrepareError(
            "Official 1.20.1 client/server block-tag resources differ "
            f"(client_only={client_only}, server_only={server_only}, changed={changed})"
        )

    summary = summarize_block_tags(server, server_prefix)
    work_dir = args.work_dir.resolve()
    output_dir = work_dir / "block-tags"
    output_dir.mkdir(parents=True, exist_ok=True)
    bundle_path = output_dir / "block-tags.json"
    bundle_path.write_text(
        json.dumps(dict(sorted(server.items())), ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    summary.update({
        "bundle_bytes": bundle_path.stat().st_size,
        "bundle_file_sha256": digest_file(bundle_path, "sha256"),
        "client_jar_sha1": lock["downloads"]["client"],
        "server_game_sha256": actual_server_sha256,
        "private_bundle_path": "block-tags/block-tags.json",
        "note": "The full tag bundle remains private under .local and is not uploaded.",
    })
    report_path = output_dir / "block-tags-report.json"
    report_path.write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    return summary


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--prepared-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/1.20.1-decompile"))
    args = parser.parse_args()
    try:
        result = export(args)
    except (OSError, ValueError, KeyError, PrepareError) as exc:
        print(f"Minecraft 1.20.1 block-tag export failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
