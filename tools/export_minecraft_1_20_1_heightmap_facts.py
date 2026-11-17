#!/usr/bin/env python3
"""Export exact 1.20.1 Heightmap predicates from the verified official game classes.

The full per-state table remains private under .local. CI uploads only counts,
hashes and provenance; the exporter never guesses predicates from block names.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

try:
    from .decompile_minecraft_1_20_1 import download_libraries, java_major, load_official_version
    from .prepare_minecraft_1_20_1 import PrepareError, load_target_lock
except ImportError:
    from decompile_minecraft_1_20_1 import download_libraries, java_major, load_official_version
    from prepare_minecraft_1_20_1 import PrepareError, load_target_lock


VERSION = "1.20.1"
SCHEMA = "rustchunk-heightmap-facts-v1"
SOURCE = Path(__file__).resolve().parent / "java" / "Vanilla1201HeightmapFactsExporter.java"
MAIN_CLASS = "dev.rustchunk.minecraft1201.Vanilla1201HeightmapFactsExporter"


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


def summarize_heightmap_facts(
    facts: dict,
    expected_states: int,
) -> dict:
    if not isinstance(facts, dict):
        raise PrepareError("Heightmap facts root must be a JSON object")
    if facts.get("minecraft_version") != VERSION:
        raise PrepareError("Heightmap facts do not identify Minecraft 1.20.1")
    if facts.get("schema") != SCHEMA:
        raise PrepareError(f"Unsupported heightmap facts schema: {facts.get('schema')!r}")
    if facts.get("predicate_source") != "Heightmap.Types.MOTION_BLOCKING.isOpaque":
        raise PrepareError("Unexpected MOTION_BLOCKING predicate provenance")
    if facts.get("no_leaves_predicate_source") != (
        "Heightmap.Types.MOTION_BLOCKING_NO_LEAVES.isOpaque"
    ):
        raise PrepareError("Unexpected MOTION_BLOCKING_NO_LEAVES predicate provenance")

    rows = facts.get("states")
    if not isinstance(rows, list):
        raise PrepareError("Heightmap facts must contain a 'states' array")
    if len(rows) != expected_states:
        raise PrepareError(
            f"Unexpected 1.20.1 block-state count: expected {expected_states}, got {len(rows)}"
        )

    motion_count = 0
    no_leaves_count = 0
    for expected_id, row in enumerate(rows):
        if not isinstance(row, list) or len(row) != 5:
            raise PrepareError(f"Heightmap facts row {expected_id} must contain exactly five columns")
        state_id, block_name, state_description, motion, no_leaves = row
        if not isinstance(state_id, int) or isinstance(state_id, bool):
            raise PrepareError(f"Heightmap facts row {expected_id} has a non-integer state ID")
        if state_id != expected_id:
            raise PrepareError(
                f"Block-state IDs must be contiguous and ordered: row {expected_id} has ID {state_id}"
            )
        if not isinstance(block_name, str) or not block_name or ":" not in block_name:
            raise PrepareError(f"Heightmap facts row {expected_id} has an invalid block identifier")
        if not isinstance(state_description, str) or not state_description:
            raise PrepareError(f"Heightmap facts row {expected_id} has no state description")
        if not isinstance(motion, bool) or not isinstance(no_leaves, bool):
            raise PrepareError(f"Heightmap facts row {expected_id} predicates must be booleans")
        motion_count += int(motion)
        no_leaves_count += int(no_leaves)

    return {
        "status": "completed",
        "minecraft_version": VERSION,
        "schema": SCHEMA,
        "block_state_count": len(rows),
        "motion_blocking_state_count": motion_count,
        "motion_blocking_no_leaves_state_count": no_leaves_count,
        "predicate_source": facts["predicate_source"],
        "no_leaves_predicate_source": facts["no_leaves_predicate_source"],
    }


def export(args) -> dict:
    if java_major(args.java) != 17:
        raise PrepareError("The Minecraft 1.20.1 heightmap exporter requires Java 17 exactly")
    if not SOURCE.is_file():
        raise PrepareError(f"Missing original RustChunk exporter source: {SOURCE}")

    prepared_dir = args.prepared_dir.resolve()
    client_jar = prepared_dir / "mapped" / "client-named.jar"
    server_jar = prepared_dir / "mapped" / "server-named.jar"
    for path in (client_jar, server_jar):
        if not path.is_file() or path.stat().st_size == 0:
            raise PrepareError(f"Missing verified, remapped 1.20.1 input: {path}")

    version_info, _ = load_official_version()

    libraries = download_libraries(version_info, args.work_dir.resolve())
    if not libraries:
        raise PrepareError("Minecraft 1.20.1 metadata returned no client libraries")

    work_dir = args.work_dir.resolve()
    output_dir = work_dir / "heightmap-facts"
    output_dir.mkdir(parents=True, exist_ok=True)
    classes_dir = work_dir / "heightmap-exporter-classes"
    if classes_dir.exists():
        shutil.rmtree(classes_dir)
    classes_dir.mkdir(parents=True, exist_ok=True)
    facts_path = output_dir / "heightmap-facts.json"

    javac = args.javac
    if not Path(javac).is_file():
        raise PrepareError(f"JDK 17 javac executable not found: {javac}")
    cp = classpath([client_jar, server_jar, *libraries])
    run(
        [
            javac, "--release", "17", "-encoding", "UTF-8",
            "-classpath", cp, "-d", str(classes_dir), str(SOURCE),
        ],
        output_dir / "javac.log",
        "Compiling the 1.20.1 heightmap facts exporter",
    )
    run(
        [
            args.java, "-Xmx2G", "-classpath",
            classpath([classes_dir, client_jar, server_jar, *libraries]),
            MAIN_CLASS, str(facts_path),
        ],
        output_dir / "export.log",
        "Running the 1.20.1 heightmap facts exporter",
    )

    try:
        facts = json.loads(facts_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise PrepareError(f"Heightmap exporter produced invalid JSON: {exc}") from exc

    lock = load_target_lock()["minecraft"]
    report = summarize_heightmap_facts(
        facts,
        expected_states=lock["expected_block_state_count"],
    )
    report.update({
        "facts_sha256": sha256(facts_path),
        "facts_bytes": facts_path.stat().st_size,
        "exporter_sha256": sha256(SOURCE),
        "client_jar_sha1": lock["downloads"]["client"],
        "server_jar_sha1": lock["downloads"]["server"],
        "private_facts_path": "heightmap-facts/heightmap-facts.json",
        "note": (
            "The complete per-state table stays private under .local. "
            "Both predicates were evaluated by the official 1.20.1 Heightmap.Types."
        ),
    })
    (output_dir / "heightmap-facts-report.json").write_text(
        json.dumps(report, indent=2) + "\n",
        encoding="utf-8",
    )
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--java", default=os.environ.get("JAVA_HOME_17_X64", "java"))
    parser.add_argument("--javac", default=os.environ.get("JAVA_HOME_17_X64", ""))
    parser.add_argument(
        "--prepared-dir", type=Path,
        default=Path(".local/minecraft/1.20.1-remap"),
    )
    parser.add_argument(
        "--work-dir", type=Path,
        default=Path(".local/minecraft/1.20.1-export"),
    )
    args = parser.parse_args()
    if args.javac and Path(args.javac).is_dir():
        executable = "javac.exe" if os.name == "nt" else "javac"
        args.javac = str(Path(args.javac) / "bin" / executable)
    try:
        report = export(args)
    except (OSError, ValueError, KeyError, PrepareError) as exc:
        print(f"Minecraft 1.20.1 heightmap export failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
