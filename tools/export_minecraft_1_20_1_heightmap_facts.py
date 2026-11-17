#!/usr/bin/env python3
"""Export and independently verify exact Minecraft 1.20.1 heightmap predicates.

The official Java predicates are checked against primitive BlockState facts
and the recursively resolved vanilla minecraft:leaves block tag. The complete
per-state table remains private under .local; CI uploads only a compact report.
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
SCHEMA = "rustchunk-heightmap-facts-v2"
SOURCE = Path(__file__).resolve().parent / "java" / "Vanilla1201HeightmapFactsExporter.java"
MAIN_CLASS = "dev.rustchunk.minecraft1201.Vanilla1201HeightmapFactsExporter"
LEAVES_TAG = "minecraft:leaves"


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


def resolve_block_tag(
    tag_name: str,
    bundle: dict[str, str],
    known_blocks: set[str],
    visiting: tuple[str, ...] = (),
) -> set[str]:
    """Resolve a vanilla block tag and all nested tag references."""
    if tag_name in visiting:
        chain = " -> ".join((*visiting, tag_name))
        raise PrepareError(f"Cyclic block-tag reference: {chain}")
    raw = bundle.get(tag_name)
    if raw is None:
        raise PrepareError(f"Missing required block tag: {tag_name}")
    try:
        document = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise PrepareError(f"Invalid JSON in block tag {tag_name}: {exc}") from exc
    if not isinstance(document, dict) or not isinstance(document.get("values"), list):
        raise PrepareError(f"Block tag {tag_name} must contain a values array")

    resolved: set[str] = set()
    for index, entry in enumerate(document["values"]):
        required = True
        if isinstance(entry, str):
            identifier = entry
        elif isinstance(entry, dict):
            identifier = entry.get("id")
            required = entry.get("required", True)
            if not isinstance(required, bool):
                raise PrepareError(
                    f"Block tag {tag_name} entry {index} has a non-boolean required flag"
                )
        else:
            raise PrepareError(f"Block tag {tag_name} entry {index} is malformed")

        if not isinstance(identifier, str) or not identifier or ":" not in identifier:
            raise PrepareError(f"Block tag {tag_name} entry {index} has an invalid identifier")

        if identifier.startswith("#"):
            nested_name = identifier[1:]
            if nested_name not in bundle and not required:
                continue
            resolved.update(
                resolve_block_tag(nested_name, bundle, known_blocks, (*visiting, tag_name))
            )
        elif identifier not in known_blocks:
            if required:
                raise PrepareError(
                    f"Block tag {tag_name} references unknown required block {identifier}"
                )
        else:
            resolved.add(identifier)
    return resolved


def summarize_heightmap_facts(
    facts: dict,
    expected_states: int,
    registry_states: list | None = None,
    block_tags_bundle: dict[str, str] | None = None,
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
    if facts.get("leaves_tag_probe") != "BlockState.is(BlockTags.LEAVES)":
        raise PrepareError("Unexpected Java leaves-tag probe provenance")
    if registry_states is None or block_tags_bundle is None:
        raise PrepareError("Both canonical block-registry facts and block tags are required")

    rows = facts.get("states")
    if not isinstance(rows, list):
        raise PrepareError("Heightmap facts must contain a 'states' array")
    if len(rows) != expected_states:
        raise PrepareError(
            f"Unexpected 1.20.1 block-state count: expected {expected_states}, got {len(rows)}"
        )
    if len(registry_states) != expected_states:
        raise PrepareError("Block facts and heightmap facts have different state counts")

    known_blocks = {
        row[0]
        for row in registry_states
        if isinstance(row, list) and row and isinstance(row[0], str)
    }
    if not known_blocks:
        raise PrepareError("Canonical block registry contains no valid block identifiers")
    leaves_blocks = resolve_block_tag(LEAVES_TAG, block_tags_bundle, known_blocks)

    air_count = 0
    blocks_motion_count = 0
    fluid_count = 0
    leaves_state_count = 0
    motion_count = 0
    no_leaves_count = 0
    for expected_id, row in enumerate(rows):
        if not isinstance(row, list) or len(row) != 9:
            raise PrepareError(
                f"Heightmap facts row {expected_id} must contain exactly nine columns"
            )
        (
            state_id,
            block_name,
            state_description,
            is_air,
            blocks_motion,
            has_fluid,
            java_leaves_tag,
            motion,
            no_leaves,
        ) = row
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
        boolean_fields = (is_air, blocks_motion, has_fluid, java_leaves_tag, motion, no_leaves)
        if any(not isinstance(value, bool) for value in boolean_fields):
            raise PrepareError(
                f"Heightmap facts row {expected_id} flags and predicates must be booleans"
            )

        registry_row = registry_states[expected_id]
        if not isinstance(registry_row, list) or len(registry_row) < 4:
            raise PrepareError(f"Block registry facts row {expected_id} is malformed")
        if block_name != registry_row[0] or state_description != registry_row[3]:
            raise PrepareError(
                f"Heightmap state {expected_id} does not align with the block registry facts"
            )
        if is_air != registry_row[1] or has_fluid != registry_row[2]:
            raise PrepareError(
                f"Heightmap primitive facts for state {expected_id} disagree with block facts"
            )

        expected_leaves = block_name in leaves_blocks
        if java_leaves_tag != expected_leaves:
            raise PrepareError(
                f"Java BlockTags.LEAVES membership disagrees with the exported vanilla tag "
                f"for state {expected_id} ({block_name})"
            )

        # Minecraft 1.20.1 Heightmap predicate semantics:
        # MOTION_BLOCKING: blocksMotion || hasFluid
        # MOTION_BLOCKING_NO_LEAVES: (blocksMotion && !LEAVES) || hasFluid
        # Fluids remain heightmap blockers even when a state belongs to LEAVES.
        expected_motion = blocks_motion or has_fluid
        expected_no_leaves = (blocks_motion and not expected_leaves) or has_fluid
        if motion != expected_motion:
            raise PrepareError(
                f"MOTION_BLOCKING Java predicate mismatch at state {expected_id} ({block_name})"
            )
        if no_leaves != expected_no_leaves:
            raise PrepareError(
                f"MOTION_BLOCKING_NO_LEAVES Java predicate mismatch at state "
                f"{expected_id} ({block_name})"
            )

        air_count += int(is_air)
        blocks_motion_count += int(blocks_motion)
        fluid_count += int(has_fluid)
        leaves_state_count += int(expected_leaves)
        motion_count += int(motion)
        no_leaves_count += int(no_leaves)

    return {
        "status": "completed",
        "minecraft_version": VERSION,
        "schema": SCHEMA,
        "block_state_count": len(rows),
        "air_state_count": air_count,
        "blocks_motion_state_count": blocks_motion_count,
        "fluid_state_count": fluid_count,
        "leaves_block_count": len(leaves_blocks),
        "leaves_state_count": leaves_state_count,
        "motion_blocking_state_count": motion_count,
        "motion_blocking_no_leaves_state_count": no_leaves_count,
        "motion_blocking_excluded_by_no_leaves_count": motion_count - no_leaves_count,
        "leaves_tag": LEAVES_TAG,
        "leaves_tag_block_ids": sorted(leaves_blocks),
        "predicate_validation": "passed_against_primitives_and_resolved_vanilla_leaves_tag",
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

    work_dir = args.work_dir.resolve()
    registry_facts_path = work_dir / "block-facts" / "facts.json"
    block_tags_path = work_dir / "block-tags" / "block-tags.json"
    try:
        registry_facts = json.loads(registry_facts_path.read_text(encoding="utf-8"))
        registry_states = registry_facts["states"]
        block_tags_bundle = json.loads(block_tags_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, KeyError, TypeError) as exc:
        raise PrepareError(
            f"Missing or malformed prerequisite block facts/tags in {work_dir}: {exc}"
        ) from exc
    if not isinstance(block_tags_bundle, dict) or not block_tags_bundle:
        raise PrepareError("The official 1.20.1 block-tag bundle is empty or malformed")

    version_info, _ = load_official_version()
    libraries = download_libraries(version_info, work_dir)
    if not libraries:
        raise PrepareError("Minecraft 1.20.1 metadata returned no client libraries")

    output_dir = work_dir / "heightmap-facts"
    output_dir.mkdir(parents=True, exist_ok=True)
    classes_dir = work_dir / "heightmap-exporter-classes"
    if classes_dir.exists():
        shutil.rmtree(classes_dir)
    classes_dir.mkdir(parents=True, exist_ok=True)
    facts_path = output_dir / "heightmap-facts.json"
    leaves_tag_input_path = output_dir / "leaves-tag-input.json"
    known_blocks = {
        row[0]
        for row in registry_states
        if isinstance(row, list) and row and isinstance(row[0], str)
    }
    leaves_blocks = resolve_block_tag(LEAVES_TAG, block_tags_bundle, known_blocks)
    leaves_tag_input_path.write_text(
        json.dumps(sorted(leaves_blocks), indent=2) + "\\n",
        encoding="utf-8",
    )

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
            MAIN_CLASS, str(facts_path), str(leaves_tag_input_path),
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
        registry_states=registry_states,
        block_tags_bundle=block_tags_bundle,
    )
    report.update({
        "facts_sha256": sha256(facts_path),
        "facts_bytes": facts_path.stat().st_size,
        "exporter_sha256": sha256(SOURCE),
        "block_registry_facts_sha256": sha256(registry_facts_path),
        "block_tags_bundle_sha256": sha256(block_tags_path),
        "resolved_leaves_tag_input_sha256": sha256(leaves_tag_input_path),
        "client_jar_sha1": lock["downloads"]["client"],
        "server_jar_sha1": lock["downloads"]["server"],
        "private_facts_path": "heightmap-facts/heightmap-facts.json",
        "note": (
            "The per-state table stays private under .local. Java Heightmap predicates, "
            "BlockTags.LEAVES membership, registry identity, and the independently resolved "
            "official leaves tag must all agree."
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
    except (OSError, ValueError, KeyError, TypeError, PrepareError) as exc:
        print(f"Minecraft 1.20.1 heightmap export failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
