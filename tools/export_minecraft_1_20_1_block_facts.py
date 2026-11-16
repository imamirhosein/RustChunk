#!/usr/bin/env python3
"""Compile and run the 1.20.1 block-registry exporter against verified private game inputs.

Only a metadata summary is emitted to CI artifacts. The complete facts.json remains under .local/.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys

try:
    from .decompile_minecraft_1_20_1 import (
        download_libraries,
        java_major,
        load_official_version,
        validate_official_downloads,
    )
    from .prepare_minecraft_1_20_1 import PrepareError, load_target_lock
except ImportError:
    from decompile_minecraft_1_20_1 import (
        download_libraries,
        java_major,
        load_official_version,
        validate_official_downloads,
    )
    from prepare_minecraft_1_20_1 import PrepareError, load_target_lock


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "tools" / "java" / "Vanilla1201BlockFactsExporter.java"
MAIN_CLASS = "dev.rustchunk.minecraft1201.Vanilla1201BlockFactsExporter"
_SHA256_RE = re.compile(r"^[0-9a-f]{64}$")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def classpath(paths: list[Path]) -> str:
    return os.pathsep.join(str(path.resolve()) for path in paths)


def run(command: list[str], log_path: Path, what: str) -> None:
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    log_path.write_text(result.stdout + result.stderr, encoding="utf-8")
    if result.returncode:
        raise PrepareError(
            f"{what} failed with exit code {result.returncode}; see {log_path}:\n"
            + (result.stdout + result.stderr)[-5000:]
        )


def _state_descriptor(row: list) -> tuple[str, tuple[tuple[str, str], ...]]:
    if len(row) < 5 or not isinstance(row[0], str):
        raise PrepareError("Malformed block facts row")
    descriptor = row[4]
    if isinstance(descriptor, str):
        block_name = descriptor
        properties: dict = {}
    elif isinstance(descriptor, dict):
        block_name = descriptor.get("id")
        properties = descriptor.get("properties", {})
        if not isinstance(block_name, str) or not isinstance(properties, dict):
            raise PrepareError("Malformed block-state descriptor")
        if any(not isinstance(k, str) or not isinstance(v, str) for k, v in properties.items()):
            raise PrepareError("Block-state properties must be string pairs")
    else:
        raise PrepareError("Block-state descriptor must be a string or object")
    if block_name != row[0]:
        raise PrepareError("Block-state descriptor name differs from its registry row")
    return block_name, tuple(sorted(properties.items()))


def summarize_facts(facts: dict, expected_blocks: int, expected_states: int) -> dict:
    if not isinstance(facts, dict):
        raise PrepareError("Block facts root must be a JSON object")
    rows = facts.get("states")
    defaults = facts.get("defaults")
    if not isinstance(rows, list) or not isinstance(defaults, dict):
        raise PrepareError("Block facts must contain 'states' array and 'defaults' object")
    if len(rows) != expected_states:
        raise PrepareError(
            f"Unexpected 1.20.1 block-state count: expected {expected_states}, got {len(rows)}"
        )
    if len(defaults) != expected_blocks:
        raise PrepareError(
            f"Unexpected 1.20.1 block count: expected {expected_blocks}, got {len(defaults)}"
        )

    names: set[str] = set()
    identities: set[tuple[str, tuple[tuple[str, str], ...]]] = set()
    air_count = 0
    fluid_count = 0
    property_count = 0
    for row in rows:
        if not isinstance(row, list) or len(row) < 5:
            raise PrepareError("Every block facts row must have at least five columns")
        if not isinstance(row[1], bool) or not isinstance(row[2], bool):
            raise PrepareError("Block facts air/fluid columns must be booleans")
        block_name, properties = _state_descriptor(row)
        if not block_name or ":" not in block_name:
            raise PrepareError(f"Invalid namespaced block identifier: {block_name!r}")
        identity = (block_name, properties)
        if identity in identities:
            raise PrepareError(f"Duplicate block state descriptor: {block_name} {properties}")
        identities.add(identity)
        names.add(block_name)
        air_count += int(row[1])
        fluid_count += int(row[2])
        property_count += int(bool(properties))

    if names != set(defaults):
        missing = sorted(names - set(defaults))[:8]
        extra = sorted(set(defaults) - names)[:8]
        raise PrepareError(f"Default-state registry mismatch (missing={missing}, extra={extra})")

    for name, state_id in defaults.items():
        if not isinstance(name, str) or not isinstance(state_id, int) or isinstance(state_id, bool):
            raise PrepareError("Default block-state IDs must be integer values")
        if state_id < 0 or state_id >= len(rows):
            raise PrepareError(f"Default block-state ID out of range for {name}: {state_id}")
        default_name, _ = _state_descriptor(rows[state_id])
        if default_name != name:
            raise PrepareError(f"Default state ID {state_id} does not belong to block {name}")

    return {
        "status": "completed",
        "minecraft_version": "1.20.1",
        "schema": "horizon-block-facts-v1",
        "block_count": len(defaults),
        "block_state_count": len(rows),
        "air_state_count": air_count,
        "fluid_state_count": fluid_count,
        "states_with_properties": property_count,
    }


def export(args) -> dict:
    if java_major(args.java) != 17:
        raise PrepareError("The Minecraft 1.20.1 block exporter requires Java 17 exactly")
    if not SOURCE.is_file():
        raise PrepareError(f"Missing original RustChunk exporter source: {SOURCE}")

    version_info, _ = load_official_version()
    validate_official_downloads(version_info.get("downloads", {}))

    prepared_dir = args.prepared_dir.resolve()
    client_jar = prepared_dir / "mapped" / "client-named.jar"
    server_jar = prepared_dir / "mapped" / "server-named.jar"
    for path in (client_jar, server_jar):
        if not path.is_file() or path.stat().st_size == 0:
            raise PrepareError(f"Missing verified, remapped 1.20.1 input: {path}")

    work_dir = args.work_dir.resolve()
    libraries = download_libraries(version_info, work_dir)
    if not libraries:
        raise PrepareError("Minecraft 1.20.1 metadata returned no client libraries")

    cp = classpath([client_jar, server_jar, *libraries])
    classes_dir = work_dir / "exporter-classes"
    if classes_dir.exists():
        shutil.rmtree(classes_dir)
    classes_dir.mkdir(parents=True, exist_ok=True)

    javac = args.javac
    if not Path(javac).is_file():
        raise PrepareError(f"JDK 17 javac executable not found: {javac}")
    output_dir = work_dir / "block-facts"
    output_dir.mkdir(parents=True, exist_ok=True)
    facts_path = output_dir / "facts.json"

    run(
        [
            javac, "--release", "17", "-encoding", "UTF-8",
            "-classpath", cp, "-d", str(classes_dir), str(SOURCE),
        ],
        output_dir / "javac.log",
        "Compiling the 1.20.1 block facts exporter",
    )
    run(
        [
            args.java, "-Xmx2G", "-classpath",
            classpath([classes_dir, client_jar, server_jar, *libraries]),
            MAIN_CLASS, str(facts_path),
        ],
        output_dir / "export.log",
        "Running the 1.20.1 block facts exporter",
    )

    try:
        facts = json.loads(facts_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise PrepareError(f"Block facts exporter produced invalid JSON: {exc}") from exc

    lock = load_target_lock()["minecraft"]
    report = summarize_facts(
        facts,
        expected_blocks=lock["expected_block_count"],
        expected_states=lock["expected_block_state_count"],
    )
    report.update({
        "facts_sha256": sha256(facts_path),
        "facts_bytes": facts_path.stat().st_size,
        "exporter_sha256": sha256(SOURCE),
        "inputs": {
            "client_jar_sha1": lock["downloads"]["client"],
            "server_jar_sha1": lock["downloads"]["server"],
        },
        "private_facts_path": "block-facts/facts.json",
        "note": "The complete block facts file stays private under .local and is not uploaded.",
    })
    report_path = output_dir / "block-facts-report.json"
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--java", default=os.environ.get("JAVA_HOME_17_X64", "java"))
    parser.add_argument("--javac", default=os.environ.get("JAVA_HOME_17_X64", ""))
    parser.add_argument("--prepared-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/1.20.1-decompile"))
    args = parser.parse_args()
    if args.javac and Path(args.javac).is_dir():
        executable = "javac.exe" if os.name == "nt" else "javac"
        args.javac = str(Path(args.javac) / "bin" / executable)
    try:
        report = export(args)
    except (OSError, ValueError, KeyError, PrepareError) as exc:
        print(f"Minecraft 1.20.1 block facts export failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
