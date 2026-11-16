#!/usr/bin/env python3
"""Compare the prepared Minecraft 1.20.1 inputs against Starling's pinned 26.3 baseline.

Game JARs, mappings, generated reports and full bytecode indexes stay under .local/.
Only aggregate, metadata-only counts are written to comparison-report.json.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import sys
import urllib.request
import zipfile

from tools.prepare_minecraft_1_20_1 import (
    MANIFEST_URL,
    PrepareError,
    download_verified,
    extract_server_game,
    java_major,
    run_data_generator,
    summarize_data_reports,
)


def fetch_json(url: str) -> tuple[dict, bytes]:
    request = urllib.request.Request(url, headers={"User-Agent": "RustChunk-version-diff/1.0"})
    with urllib.request.urlopen(request, timeout=90) as response:
        payload = response.read()
    try:
        return json.loads(payload), payload
    except json.JSONDecodeError as exc:
        raise PrepareError(f"Invalid JSON returned by {url}: {exc}") from exc


def load_pinned_26_3(starling_root: Path) -> tuple[dict, str]:
    lock_path = starling_root / "research-lock.json"
    if not lock_path.is_file():
        raise PrepareError(f"Starling's research lock was not found: {lock_path}")
    lock = json.loads(lock_path.read_text(encoding="utf-8"))
    pinned = lock.get("minecraft", {})
    if pinned.get("version") != "26.3":
        raise PrepareError(
            f"The checked-out Starling baseline pins {pinned.get('version')!r}; expected 26.3"
        )

    manifest, _ = fetch_json(MANIFEST_URL)
    entry = next((item for item in manifest.get("versions", []) if item.get("id") == "26.3"), None)
    if entry is None:
        raise PrepareError("Mojang's official version manifest no longer lists 26.3")
    allowed_manifest_hashes = {
        pinned.get("manifest_sha1", ""),
        *pinned.get("accepted_manifest_sha1", []),
    }
    if entry.get("sha1") not in allowed_manifest_hashes:
        raise PrepareError(
            "The current 26.3 version JSON is not one of the exact upstream-pinned "
            "or explicitly accepted manifests; stop rather than silently changing the baseline."
        )

    info, version_bytes = fetch_json(entry["url"])
    actual_manifest_sha1 = hashlib.sha1(version_bytes).hexdigest()
    if actual_manifest_sha1 != entry["sha1"]:
        raise PrepareError("The 26.3 version JSON does not match Mojang's manifest SHA-1")
    if info.get("id") != "26.3":
        raise PrepareError(f"Official version JSON says {info.get('id')!r}, not 26.3")
    if info.get("javaVersion", {}).get("majorVersion") != 25:
        raise PrepareError("The pinned 26.3 baseline no longer identifies Java 25")
    downloads = info.get("downloads", {})
    if "client_mappings" in downloads or "server_mappings" in downloads:
        raise PrepareError("The pinned 26.3 baseline unexpectedly became obfuscated")
    if downloads.get("client", {}).get("sha1") != pinned.get("client_sha1"):
        raise PrepareError("Mojang's 26.3 client JAR SHA-1 differs from Starling's research lock")
    if downloads.get("server", {}).get("sha1") != pinned.get("server_sha1"):
        raise PrepareError("Mojang's 26.3 server JAR SHA-1 differs from Starling's research lock")
    return info, actual_manifest_sha1


def load_class_indexer(starling_root: Path):
    tools_dir = str((starling_root / "tools").resolve())
    if not (Path(tools_dir) / "class_index.py").is_file():
        raise PrepareError(f"Starling's class indexer is missing from {tools_dir}")
    if tools_dir not in sys.path:
        sys.path.insert(0, tools_dir)
    try:
        import class_index
    except ImportError as exc:
        raise PrepareError(f"Could not import Starling's class indexer: {exc}") from exc
    return class_index


def class_diff_summary(old_index: dict, new_index: dict, class_index) -> dict:
    changes = class_index.diff(old_index, new_index)
    changed_methods = sum(
        class_index.changed_methods(change) for change in changes["changed"].values()
    )
    return {
        "classes_1_20_1": len(old_index),
        "classes_26_3": len(new_index),
        "added_in_26_3": len(changes["added"]),
        "removed_since_1_20_1": len(changes["removed"]),
        "moved": len(changes["moved"]),
        "changed": len(changes["changed"]),
        "changed_methods": changed_methods,
        "note": "Full names and bytecode indexes remain private; only aggregate counts are exported.",
    }


def jar_member_map(jar: Path, prefix: str) -> dict[str, int]:
    with zipfile.ZipFile(jar) as archive:
        return {
            item.filename: item.CRC
            for item in archive.infolist()
            if item.filename.startswith(prefix) and not item.is_dir()
        }


def jar_path_diff(old_jar: Path, new_jar: Path, prefix: str) -> dict:
    old, new = jar_member_map(old_jar, prefix), jar_member_map(new_jar, prefix)
    common = old.keys() & new.keys()
    return {
        "files_1_20_1": len(old),
        "files_26_3": len(new),
        "added_in_26_3": len(new.keys() - old.keys()),
        "removed_since_1_20_1": len(old.keys() - new.keys()),
        "changed_crc": sum(old[name] != new[name] for name in common),
    }


def command_node_count(node: object) -> int:
    if not isinstance(node, dict):
        return 0
    children = node.get("children", {})
    if not isinstance(children, dict):
        return 0
    return sum(1 + command_node_count(child) for child in children.values())


def report_features(reports_dir: Path) -> dict:
    summary = summarize_data_reports(reports_dir)

    def read_root(name: str):
        path = reports_dir / name
        return json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None

    blocks = read_root("blocks.json")
    registries = read_root("registries.json")
    commands = read_root("commands.json")
    block_states = (
        sum(len(value.get("states", [])) for value in blocks.values() if isinstance(value, dict))
        if isinstance(blocks, dict) else None
    )
    registry_entries = (
        sum(len(value.get("entries", {})) for value in registries.values() if isinstance(value, dict))
        if isinstance(registries, dict) else None
    )
    return {
        "json_file_count": summary["json_file_count"],
        "top_level_report_names": sorted(summary["top_level_reports"]),
        "blocks": len(blocks) if isinstance(blocks, dict) else None,
        "block_states": block_states,
        "registries": len(registries) if isinstance(registries, dict) else None,
        "registry_entries": registry_entries,
        "command_nodes": command_node_count(commands) if isinstance(commands, dict) else None,
        "packets_report_present": (reports_dir / "packets.json").is_file(),
        "item_component_report_files": len(list((reports_dir / "minecraft" / "components" / "item").glob("*.json"))),
        "biome_parameter_reports": len(list((reports_dir / "biome_parameters").rglob("*.json"))),
    }


def compare(args) -> dict:
    starling_root = args.starling_root.resolve()
    target_dir = args.target_work_dir.resolve()
    output_dir = args.work_dir.resolve()
    old_report_path = target_dir / "audit-report.json"
    old_index_path = target_dir / "private-index" / "1.20.1-classes.json"
    old_client_jar = target_dir / "mapped" / "client-named.jar"
    old_reports_dir = target_dir / "datagen" / "out" / "reports"
    for required in (old_report_path, old_index_path, old_client_jar, old_reports_dir):
        if not required.exists():
            raise PrepareError(f"Required prepared 1.20.1 input is missing: {required}")

    output_dir.mkdir(parents=True, exist_ok=True)
    baseline_dir = output_dir / "26.3"
    downloads_dir = baseline_dir / "downloads"
    jars_dir = baseline_dir / "jars"
    downloads_dir.mkdir(parents=True, exist_ok=True)
    jars_dir.mkdir(parents=True, exist_ok=True)

    info, manifest_sha1 = load_pinned_26_3(starling_root)
    downloads = info["downloads"]
    client_jar = download_verified(
        downloads["client"]["url"], downloads_dir / "client.jar",
        downloads["client"]["sha1"], downloads["client"].get("size"),
    )
    server_bundle = download_verified(
        downloads["server"]["url"], downloads_dir / "server-bundler.jar",
        downloads["server"]["sha1"], downloads["server"].get("size"),
    )
    server_game, server_game_sha256 = extract_server_game(
        server_bundle, jars_dir / "server-game.jar", "26.3"
    )

    java = args.java_25
    if java_major(java) != 25:
        raise PrepareError("The Starling 26.3 baseline requires Java 25 exactly")
    run_data_generator(java, server_bundle, baseline_dir, version="26.3")
    new_reports_dir = baseline_dir / "datagen" / "out" / "reports"

    class_index = load_class_indexer(starling_root)
    old_index = json.loads(old_index_path.read_text(encoding="utf-8"))
    new_index = class_index.index_jars([client_jar, server_game], jobs=args.jobs)
    private_index = baseline_dir / "private-index"
    private_index.mkdir(parents=True, exist_ok=True)
    (private_index / "26.3-classes.json").write_text(
        json.dumps(new_index, separators=(",", ":")) + "\n", encoding="utf-8"
    )

    comparison = {
        "versions": {
            "target": "1.20.1",
            "starling_baseline": "26.3",
            "starling_source_commit": "5432370d573642041585c69b40fb045b27133488",
        },
        "baseline_inputs": {
            "version_metadata_sha1": manifest_sha1,
            "client_sha1": downloads["client"]["sha1"],
            "server_sha1": downloads["server"]["sha1"],
            "server_game_sha256": server_game_sha256,
            "java_major": 25,
        },
        "code": class_diff_summary(old_index, new_index, class_index),
        "data_generator": {
            "1.20.1": report_features(old_reports_dir),
            "26.3": report_features(new_reports_dir),
        },
        "client_resource_files": {
            "assets": jar_path_diff(old_client_jar, client_jar, "assets/minecraft/"),
            "data": jar_path_diff(old_client_jar, client_jar, "data/minecraft/"),
            "worldgen_data": jar_path_diff(old_client_jar, client_jar, "data/minecraft/worldgen/"),
        },
        "limitations": [
            "This is a version-difference inventory, not a gameplay port.",
            "Minecraft 1.20.1 has no packets.json or item-component report from its data generator; packet registration needs a version-specific exporter, and 1.20.1 item semantics need their legacy representation.",
            "The full JARs, generated JSON, source files and class/method indexes stay under .local and are never uploaded.",
        ],
        "private_paths": {
            "26.3_class_index": "26.3/private-index/26.3-classes.json",
            "26.3_reports": "26.3/datagen/out/reports",
        },
    }
    (output_dir / "comparison-report.json").write_text(
        json.dumps(comparison, indent=2) + "\n", encoding="utf-8"
    )
    return comparison


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--starling-root", type=Path, default=Path("upstream/starling"))
    parser.add_argument("--target-work-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/version-compare"))
    java_home_25 = os.environ.get("JAVA_HOME_25_X64")
    default_java_25 = str(Path(java_home_25) / "bin" / "java") if java_home_25 else "java"
    parser.add_argument("--java-25", default=default_java_25)
    parser.add_argument("--jobs", type=int, default=2)
    args = parser.parse_args()

    try:
        report = compare(args)
    except (OSError, ValueError, KeyError, urllib.error.URLError, PrepareError, zipfile.BadZipFile) as exc:
        print(f"Version comparison failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
