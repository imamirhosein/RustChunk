#!/usr/bin/env python3
"""Decompile the verified Minecraft 1.20.1 client/server privately with pinned Vineflower.

All sources and dependency JARs stay under the ignored work directory. Only a
metadata-only report is intended for GitHub Actions artifact upload.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys
import tempfile
import urllib.error
import urllib.request
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from tools.prepare_minecraft_1_20_1 import (
    PrepareError,
    digest_file,
    download_verified,
    java_major,
    load_official_version,
    validate_official_downloads,
)


def safe_library_path(value: str) -> Path:
    """Reject paths that could escape the private libraries directory."""
    relative = PurePosixPath(value)
    if not value or relative.is_absolute() or ".." in relative.parts or "\\" in value:
        raise PrepareError(f"Unsafe library artifact path in version metadata: {value!r}")
    if not relative.parts:
        raise PrepareError(f"Empty library artifact path: {value!r}")
    return Path(*relative.parts)


def library_artifacts(version_info: dict) -> list[dict]:
    """Collect unique Maven artifact JARs from Mojang's version metadata."""
    by_path: dict[str, dict] = {}
    for library in version_info.get("libraries", []):
        artifact = library.get("downloads", {}).get("artifact")
        if not artifact:
            # Native classifiers are not required to resolve source types.
            continue
        relative = safe_library_path(artifact.get("path", ""))
        required = ("url", "sha1")
        if any(not artifact.get(key) for key in required):
            raise PrepareError(
                f"Incomplete library artifact metadata for {library.get('name', relative)}"
            )
        record = {
            "path": relative.as_posix(),
            "url": artifact["url"],
            "sha1": artifact["sha1"],
            "size": artifact.get("size"),
        }
        previous = by_path.get(record["path"])
        if previous and previous != record:
            raise PrepareError(f"Conflicting library metadata for {record['path']}")
        by_path[record["path"]] = record
    return [by_path[key] for key in sorted(by_path)]


def download_vineflower(lock: dict, destination: Path) -> tuple[Path, dict]:
    pin = lock.get("vineflower", {})
    version, url, expected = pin.get("version"), pin.get("url"), pin.get("sha256")
    if not version or not url or not expected:
        raise PrepareError("The pinned Starling research lock has incomplete Vineflower metadata")
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.is_file() and digest_file(destination, "sha256") == expected:
        return destination, pin
    destination.unlink(missing_ok=True)
    temporary = destination.with_name(destination.name + ".partial")
    request = urllib.request.Request(url, headers={"User-Agent": "RustChunk-1.20.1-decompiler/1.0"})
    try:
        hasher = hashlib.sha256()
        with urllib.request.urlopen(request, timeout=120) as response, temporary.open("wb") as out:
            for chunk in iter(lambda: response.read(1024 * 1024), b""):
                out.write(chunk)
                hasher.update(chunk)
        actual = hasher.hexdigest()
        if actual != expected:
            raise PrepareError(
                f"Vineflower SHA-256 mismatch: expected {expected}, got {actual}"
            )
        temporary.replace(destination)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise
    return destination, pin


def download_libraries(version_info: dict, work_dir: Path) -> list[Path]:
    root = work_dir / "libraries"
    downloaded = []
    for artifact in library_artifacts(version_info):
        relative = safe_library_path(artifact["path"])
        target = root / relative
        download_verified(
            artifact["url"], target, artifact["sha1"], artifact.get("size")
        )
        downloaded.append(target)
    return downloaded


def decompile_side(
    *,
    side: str,
    java: str,
    vineflower: Path,
    libraries: list[Path],
    input_jar: Path,
    sources_root: Path,
    threads: int,
    heap: str,
) -> dict:
    destination = sources_root / side
    source_sha256 = digest_file(input_jar, "sha256")
    marker = destination / ".rustchunk-decompile.json"
    if marker.is_file():
        cached = json.loads(marker.read_text(encoding="utf-8"))
        if (
            cached.get("input_sha256") == source_sha256
            and cached.get("vineflower_sha256") == digest_file(vineflower, "sha256")
        ):
            return {key: value for key, value in cached.items() if key != "private_source_path"}
        raise PrepareError(
            f"Existing {side} source directory does not match the current input; "
            f"move it aside before retrying: {destination}"
        )
    if destination.exists():
        raise PrepareError(
            f"Refusing to overwrite existing {side} source directory without a valid marker: {destination}"
        )

    sources_root.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=f"{side}-incomplete-", dir=sources_root))
    log_path = sources_root / f"{side}-vineflower.log"
    command = [
        java,
        f"-Xmx{heap}",
        "-jar",
        str(vineflower),
        "--folder",
        "--skip-extra-files=1",
        "--log-level=warn",
        f"--thread-count={threads}",
    ]
    command.extend(f"--add-external={library}" for library in libraries)
    command.extend([str(input_jar), str(stage)])

    with log_path.open("w", encoding="utf-8") as log:
        result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=False)
    if result.returncode:
        raise PrepareError(
            f"Vineflower failed for {side} with exit code {result.returncode}; "
            f"partial sources: {stage}; log: {log_path}"
        )

    java_files = [path for path in stage.rglob("*.java") if path.is_file()]
    if not java_files:
        raise PrepareError(
            f"Vineflower returned success for {side} but produced no .java files; see {log_path}"
        )
    total_bytes = sum(path.stat().st_size for path in java_files)
    summary = {
        "side": side,
        "status": "completed",
        "input_sha256": source_sha256,
        "vineflower_version": "pinned-by-upstream-lock",
        "vineflower_sha256": digest_file(vineflower, "sha256"),
        "java_source_file_count": len(java_files),
        "java_source_bytes": total_bytes,
        "private_source_path": f"sources/{side}",
    }
    (stage / ".rustchunk-decompile.json").write_text(
        json.dumps(summary, indent=2) + "\n", encoding="utf-8"
    )
    stage.rename(destination)
    return {key: value for key, value in summary.items() if key != "private_source_path"}


def decompile(args) -> dict:
    java = args.java
    if java_major(java) != 17:
        raise PrepareError("Minecraft 1.20.1 source preparation requires Java 17 exactly")

    version_info, metadata_sha1 = load_official_version()
    if version_info.get("id") != "1.20.1":
        raise PrepareError("Official version metadata did not resolve to 1.20.1")

    validate_official_downloads(version_info.get("downloads", {}))

    starling_root = args.starling_root.resolve()
    lock_path = starling_root / "research-lock.json"
    if not lock_path.is_file():
        raise PrepareError(f"Starling research lock not found: {lock_path}")
    lock = json.loads(lock_path.read_text(encoding="utf-8"))

    work_dir = args.work_dir.resolve()
    work_dir.mkdir(parents=True, exist_ok=True)
    vineflower, vineflower_pin = download_vineflower(
        lock, work_dir / "tools" / f"vineflower-{lock['vineflower']['version']}.jar"
    )
    libraries = download_libraries(version_info, work_dir)

    target_dir = args.prepared_dir.resolve()
    client_jar = target_dir / "mapped" / "client-named.jar"
    server_jar = target_dir / "mapped" / "server-named.jar"
    for required in (client_jar, server_jar):
        if not required.is_file():
            raise PrepareError(f"Missing remapped 1.20.1 input JAR: {required}")
        with zipfile.ZipFile(required) as archive:
            if archive.testzip() is not None:
                raise PrepareError(f"Remapped input JAR failed ZIP integrity check: {required}")

    selected = ["client", "server"] if args.side == "both" else [args.side]
    outputs = []
    for side in selected:
        outputs.append(
            decompile_side(
                side=side,
                java=java,
                vineflower=vineflower,
                libraries=libraries,
                input_jar=client_jar if side == "client" else server_jar,
                sources_root=work_dir / "sources",
                threads=args.threads,
                heap=args.heap,
            )
        )

    report = {
        "status": "completed",
        "minecraft_version": "1.20.1",
        "version_metadata_sha1": metadata_sha1,
        "java_major": 17,
        "decompiler": {
            "name": "Vineflower",
            "version": vineflower_pin["version"],
            "sha256": vineflower_pin["sha256"],
        },
        "external_library_artifacts": len(libraries),
        "outputs": outputs,
        "private_workspace": str(work_dir),
        "note": "Decompiled sources and library JARs remain private under .local and are not uploaded.",
    }
    (work_dir / "decompile-report.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8"
    )
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--java", default=os.environ.get("JAVA_HOME_17_X64", "java"))
    parser.add_argument("--starling-root", type=Path, default=Path("upstream/starling"))
    parser.add_argument("--prepared-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/1.20.1-decompile"))
    parser.add_argument("--side", choices=("client", "server", "both"), default="both")
    parser.add_argument("--threads", type=int, default=4)
    parser.add_argument("--heap", default="4G")
    args = parser.parse_args()
    if args.threads < 1:
        parser.error("--threads must be at least 1")
    if not args.heap or not args.heap[-1:] in "MmGg" or not args.heap[:-1].isdigit() or int(args.heap[:-1]) < 1:
        parser.error("--heap must look like 4G or 512M")
    try:
        report = decompile(args)
    except (OSError, ValueError, KeyError, urllib.error.URLError, PrepareError, zipfile.BadZipFile) as exc:
        print(f"1.20.1 decompilation failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
