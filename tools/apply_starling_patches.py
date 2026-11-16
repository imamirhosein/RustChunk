#!/usr/bin/env python3
"""Apply RustChunk's reviewed patch series to the exact pinned Starling submodule.

The submodule must be a clean checkout at the manifest's base commit. All patches
are checked before any is applied, preventing a bad later patch from leaving a
partially modified engine tree.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_STARLING_ROOT = ROOT / "upstream" / "starling"
DEFAULT_PATCH_ROOT = ROOT / "patches" / "starling"
SHA_RE = re.compile(r"^[0-9a-f]{40}$")


class PatchError(RuntimeError):
    """The patch manifest, base checkout or patch application is invalid."""


def run_git(starling_root: Path, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    result = subprocess.run(
        ["git", "-C", str(starling_root), *args],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if check and result.returncode:
        raise PatchError(
            f"git {' '.join(args)} failed ({result.returncode}): "
            f"{(result.stdout + result.stderr)[-3000:]}"
        )
    return result


def load_manifest(patch_root: Path) -> tuple[str, list[Path]]:
    manifest_path = patch_root / "manifest.json"
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise PatchError(f"Cannot read patch manifest {manifest_path}: {exc}") from exc

    base_commit = manifest.get("base_commit")
    if not isinstance(base_commit, str) or not SHA_RE.fullmatch(base_commit):
        raise PatchError("Patch manifest must pin a full 40-character lowercase base_commit")

    series = manifest.get("patches")
    if not isinstance(series, list):
        raise PatchError("Patch manifest 'patches' must be a list")

    root = patch_root.resolve()
    patch_paths: list[Path] = []
    seen: set[str] = set()
    for item in series:
        if not isinstance(item, str):
            raise PatchError("Every patch series entry must be a relative POSIX path")
        relative = PurePosixPath(item)
        if (
            not item
            or relative.is_absolute()
            or ".." in relative.parts
            or "\\" in item
            or relative.suffix != ".patch"
        ):
            raise PatchError(f"Unsafe or invalid patch path: {item!r}")
        if item in seen:
            raise PatchError(f"Patch appears more than once in the series: {item}")
        seen.add(item)
        path = (root.joinpath(*relative.parts)).resolve()
        if root not in path.parents or not path.is_file():
            raise PatchError(f"Patch file is missing or escapes patch root: {item}")
        patch_paths.append(path)
    return base_commit, patch_paths


def apply_series(
    starling_root: Path,
    patch_root: Path,
    *,
    check_only: bool = False,
) -> dict:
    starling_root = starling_root.resolve()
    patch_root = patch_root.resolve()
    if not starling_root.is_dir():
        raise PatchError(f"Starling source directory does not exist: {starling_root}")

    base_commit, patches = load_manifest(patch_root)
    actual_commit = run_git(starling_root, "rev-parse", "HEAD").stdout.strip()
    if actual_commit != base_commit:
        raise PatchError(
            f"Starling base commit mismatch: manifest expects {base_commit}, "
            f"checkout is {actual_commit}"
        )

    status = run_git(starling_root, "status", "--porcelain", "--untracked-files=all").stdout
    if status.strip():
        raise PatchError(
            "Refusing to apply patches to a dirty Starling checkout; start from the pinned clean submodule"
        )

    # Preflight every patch first. A conflict in the last patch cannot partially
    # mutate the submodule before the failure is reported.
    for patch in patches:
        run_git(
            starling_root,
            "apply",
            "--check",
            "--whitespace=error",
            str(patch),
        )

    if not check_only:
        for patch in patches:
            run_git(
                starling_root,
                "apply",
                "--whitespace=error",
                str(patch),
            )

    return {
        "base_commit": actual_commit,
        "patch_count": len(patches),
        "patches": [str(p.relative_to(patch_root)) for p in patches],
        "mode": "check" if check_only else "apply",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--starling-root", type=Path, default=DEFAULT_STARLING_ROOT)
    parser.add_argument("--patch-root", type=Path, default=DEFAULT_PATCH_ROOT)
    parser.add_argument("--check", action="store_true", help="validate the series without applying it")
    args = parser.parse_args()
    try:
        result = apply_series(args.starling_root, args.patch_root, check_only=args.check)
    except (OSError, ValueError, PatchError) as exc:
        print(f"Starling patch preparation failed: {exc}", file=sys.stderr)
        return 2
    if result["patch_count"]:
        print(
            f"Starling patch series {result['mode']}d: "
            f"{result['patch_count']} patch(es) on {result['base_commit']}"
        )
        for patch in result["patches"]:
            print(f"  - {patch}")
    else:
        print(f"Starling base verified; patch series is empty ({result['base_commit']})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
