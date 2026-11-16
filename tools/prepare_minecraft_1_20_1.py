#!/usr/bin/env python3
"""Prepare and remap the official Minecraft 1.20.1 client/server JARs for local analysis.

All game JARs, mapping files and remapped outputs stay in the ignored work directory.
Only a metadata-only JSON report should ever be exported from CI.
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
import urllib.error
import urllib.request
import zipfile

VERSION = "1.20.1"
MANIFEST_URL = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
MAVEN_COORDINATES = ("net.md-5", "SpecialSource", "1.11.6")
SPECIALSOURCE_MAIN = "net.md_5.specialsource.SpecialSource"
MAVEN_DEPENDENCY_PLUGIN = "org.apache.maven.plugins:maven-dependency-plugin:3.8.1"


class PrepareError(RuntimeError):
    """A version input failed validation or a preparation stage failed."""


def digest_file(path: Path, algorithm: str = "sha1") -> str:
    hasher = hashlib.new(algorithm)
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def download_verified(url: str, destination: Path, expected_sha1: str,
                      expected_size: int | None = None) -> Path:
    """Download atomically and verify against Mojang's version metadata."""
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.is_file():
        valid_hash = digest_file(destination, "sha1") == expected_sha1
        valid_size = expected_size is None or destination.stat().st_size == expected_size
        if valid_hash and valid_size:
            return destination
        destination.unlink()

    temporary = destination.with_name(destination.name + ".partial")
    request = urllib.request.Request(url, headers={"User-Agent": "RustChunk-1.20.1-research/1.0"})
    try:
        hasher = hashlib.sha1()
        total = 0
        with urllib.request.urlopen(request, timeout=90) as response, temporary.open("wb") as out:
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                out.write(chunk)
                hasher.update(chunk)
                total += len(chunk)
        if hasher.hexdigest() != expected_sha1:
            raise PrepareError(
                f"SHA-1 mismatch for {url}: expected {expected_sha1}, got {hasher.hexdigest()}"
            )
        if expected_size is not None and total != expected_size:
            raise PrepareError(
                f"Size mismatch for {url}: expected {expected_size}, got {total}"
            )
        temporary.replace(destination)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise
    return destination


def load_official_version() -> tuple[dict, str]:
    """Resolve 1.20.1 through Mojang's current official version manifest."""
    request = urllib.request.Request(MANIFEST_URL, headers={"User-Agent": "RustChunk-1.20.1-research/1.0"})
    with urllib.request.urlopen(request, timeout=60) as response:
        manifest_bytes = response.read()
    manifest = json.loads(manifest_bytes)
    entry = next((item for item in manifest["versions"] if item.get("id") == VERSION), None)
    if entry is None:
        raise PrepareError(f"Mojang's official version manifest does not list {VERSION}")
    with urllib.request.urlopen(entry["url"], timeout=60) as response:
        version_bytes = response.read()
    if hashlib.sha1(version_bytes).hexdigest() != entry["sha1"]:
        raise PrepareError("The 1.20.1 version JSON does not match Mojang's manifest SHA-1")
    version_json = json.loads(version_bytes)
    if version_json.get("id") != VERSION:
        raise PrepareError(f"Official version JSON says {version_json.get('id')!r}, not {VERSION}")
    return version_json, entry["sha1"]


def parse_server_bundle_entry(listing: str, version: str) -> tuple[str, str]:
    """Return (SHA-256, relative jar path) for exactly one bundled game entry."""
    records = []
    for line in listing.splitlines():
        fields = line.split()
        if len(fields) == 3 and fields[1] == version:
            records.append((fields[0].lower(), fields[2]))
    if len(records) != 1:
        raise PrepareError(
            f"Expected exactly one bundled server entry for {version}; found {len(records)}"
        )
    return records[0]


def extract_server_game(server_bundler: Path, destination: Path, version: str) -> tuple[Path, str]:
    """Extract the one versioned game JAR inside Mojang's bundled server download."""
    destination.parent.mkdir(parents=True, exist_ok=True)
    partial = destination.with_suffix(".partial")
    try:
        with zipfile.ZipFile(server_bundler) as archive:
            if "META-INF/versions.list" not in archive.namelist():
                shutil.copyfile(server_bundler, partial)
                actual = digest_file(partial, "sha256")
                partial.replace(destination)
                return destination, actual
            expected_sha256, relative = parse_server_bundle_entry(
                archive.read("META-INF/versions.list").decode("utf-8"), version
            )
            entry_path = f"META-INF/versions/{relative}"
            hasher = hashlib.sha256()
            with archive.open(entry_path) as source, partial.open("wb") as out:
                for chunk in iter(lambda: source.read(1024 * 1024), b""):
                    out.write(chunk)
                    hasher.update(chunk)
            actual = hasher.hexdigest()
            if actual != expected_sha256:
                partial.unlink(missing_ok=True)
                raise PrepareError(
                    f"Bundled server game JAR SHA-256 mismatch: expected {expected_sha256}, got {actual}"
                )
            partial.replace(destination)
            return destination, actual
    except (zipfile.BadZipFile, KeyError, UnicodeDecodeError) as exc:
        partial.unlink(missing_ok=True)
        raise PrepareError(f"Could not unpack Mojang's {version} server bundle: {exc}") from exc


def java_major(java: str) -> int:
    result = subprocess.run([java, "-version"], capture_output=True, text=True, check=False)
    output = result.stderr + result.stdout
    import re
    match = re.search(r'version "(\d+)', output)
    if result.returncode != 0 or not match:
        raise PrepareError(f"Could not determine Java version from {java}: {output[-1000:]}")
    return int(match.group(1))


def ensure_specialsource_classpath(work_dir: Path, maven: str) -> str:
    """Resolve SpecialSource and its declared dependencies from Maven Central."""
    group, artifact, version = MAVEN_COORDINATES
    pom_dir = work_dir / "maven"
    pom_dir.mkdir(parents=True, exist_ok=True)
    pom = pom_dir / "pom.xml"
    classpath_file = pom_dir / "specialsource-classpath.txt"
    pom.write_text(
        f"""<project xmlns="http://maven.apache.org/POM/4.0.0"
  xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">
  <modelVersion>4.0.0</modelVersion>
  <groupId>dev.rustchunk</groupId><artifactId>mapping-tool</artifactId><version>1.0</version>
  <dependencies>
    <dependency>
      <groupId>{group}</groupId><artifactId>{artifact}</artifactId><version>{version}</version>
    </dependency>
  </dependencies>
</project>
""",
        encoding="utf-8",
    )
    command = [
        maven, "-B", "-q", "-f", str(pom),
        f"{MAVEN_DEPENDENCY_PLUGIN}:build-classpath",
        f"-Dmdep.outputFile={classpath_file}",
    ]
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    if result.returncode != 0 or not classpath_file.is_file():
        raise PrepareError(
            "Maven could not resolve SpecialSource 1.11.6 and its dependencies:\n"
            + (result.stdout + result.stderr)[-4000:]
        )
    classpath = classpath_file.read_text(encoding="utf-8").strip()
    if not classpath:
        raise PrepareError("Maven returned an empty SpecialSource classpath")
    return classpath


def remap_jar(java: str, classpath: str, source: Path, mappings: Path,
              destination: Path, log_path: Path) -> int:
    """Apply Mojang's original ProGuard map; SpecialSource 1.11.6 parses ProGuard directly."""
    destination.parent.mkdir(parents=True, exist_ok=True)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    command = [
        java, "-cp", classpath, SPECIALSOURCE_MAIN,
        "--in-jar", str(source),
        "--out-jar", str(destination),
        "--srg-in", str(mappings),
        "--kill-lvt",
        "--stable",
    ]
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    log_path.write_text(result.stdout + result.stderr, encoding="utf-8")
    if result.returncode != 0 or not destination.is_file() or destination.stat().st_size == 0:
        raise PrepareError(
            f"SpecialSource failed while remapping {source.name}; see {log_path}:\n"
            + (result.stdout + result.stderr)[-4000:]
        )
    try:
        with zipfile.ZipFile(destination) as archive:
            names = archive.namelist()
            class_count = sum(name.endswith(".class") for name in names)
            if not class_count or archive.testzip() is not None:
                raise PrepareError(f"Remapped JAR is empty or corrupt: {destination}")
    except zipfile.BadZipFile as exc:
        raise PrepareError(f"SpecialSource output is not a valid JAR: {destination}") from exc
    return class_count


def version_facts(game_info: dict, manifest_info: dict) -> dict:
    pack = game_info.get("pack_version", {})
    return {
        "version": game_info.get("id"),
        "protocol": game_info.get("protocol_version"),
        "data_version": game_info.get("world_version"),
        "java_major": manifest_info.get("javaVersion", {}).get("majorVersion"),
        # 1.20.1 stores these as "resource"/"data"; newer 26.x JARs
        # use "resource_major"/"data_major".
        "resource_pack_format": pack.get("resource_major", pack.get("resource")),
        "data_pack_format": pack.get("data_major", pack.get("data")),
    }


def read_game_facts(client_jar: Path, manifest_info: dict) -> dict:
    try:
        with zipfile.ZipFile(client_jar) as archive:
            game_info = json.loads(archive.read("version.json"))
    except (zipfile.BadZipFile, KeyError, json.JSONDecodeError) as exc:
        raise PrepareError(f"Could not read version.json from the official client JAR: {exc}") from exc
    facts = version_facts(game_info, manifest_info)
    if facts["version"] != VERSION:
        raise PrepareError(f"Client JAR identifies itself as {facts['version']!r}, not {VERSION}")
    expected = {
        "protocol": 763,
        "data_version": 3465,
        "java_major": 17,
        "resource_pack_format": 15,
        "data_pack_format": 15,
    }
    differences = {key: [value, facts.get(key)] for key, value in expected.items() if facts.get(key) != value}
    if differences:
        raise PrepareError(f"Unexpected {VERSION} version facts in the verified client JAR: {differences}")
    return facts


def run_data_generator(java: str, server_bundler: Path, work_dir: Path,
                       version: str = VERSION) -> dict:
    """Run the selected version's own data generator and retain its JSON reports privately."""
    datagen_dir = work_dir / "datagen"
    output_dir = datagen_dir / "out"
    log_path = datagen_dir / "datagen.log"
    reports_dir = output_dir / "reports"
    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    command = [
        java,
        "-DbundlerMainClass=net.minecraft.data.Main",
        "-jar",
        str(server_bundler),
        "--reports",
        "--output",
        str(output_dir),
    ]
    result = subprocess.run(command, capture_output=True, text=True, check=False)
    log_path.write_text(result.stdout + result.stderr, encoding="utf-8")
    if result.returncode != 0:
        raise PrepareError(
            f"Minecraft {version} data generator failed; see {log_path}:\n"
            + (result.stdout + result.stderr)[-5000:]
        )
    if not reports_dir.is_dir():
        raise PrepareError(
            f"Minecraft {version} data generator returned success but did not create {reports_dir}"
        )

    return summarize_data_reports(reports_dir)


def summarize_data_reports(reports_dir: Path) -> dict:
    """Return metadata only; never put generated Minecraft JSON data in a public artifact."""
    files = sorted(path for path in reports_dir.rglob("*.json") if path.is_file())
    if not files:
        raise PrepareError(f"No JSON reports were generated under {reports_dir}")

    summaries = {}
    for path in files:
        relative = path.relative_to(reports_dir).as_posix()
        # Keep detailed summaries for high-value root reports but not their proprietary game data.
        if "/" not in relative:
            summary = {"bytes": path.stat().st_size}
            try:
                value = json.loads(path.read_text(encoding="utf-8"))
                summary["type"] = type(value).__name__
                if isinstance(value, dict):
                    summary["entry_count"] = len(value)
                    summary["sample_keys"] = sorted(map(str, value.keys()))[:12]
                elif isinstance(value, list):
                    summary["entry_count"] = len(value)
            except (OSError, UnicodeDecodeError, json.JSONDecodeError):
                summary["parseable_json"] = False
            summaries[relative] = summary

    return {
        "status": "completed",
        "json_file_count": len(files),
        "total_bytes": sum(path.stat().st_size for path in files),
        "report_paths": [path.relative_to(reports_dir).as_posix() for path in files],
        "top_level_reports": summaries,
        "reports_path": "datagen/out/reports",
    }


def index_remapped_game(client_jar: Path, server_jar: Path, starling_root: Path,
                         work_dir: Path, jobs: int | None = None) -> dict:
    """Use the pinned upstream class indexer; keep the full bytecode inventory private."""
    indexer = starling_root / "tools" / "class_index.py"
    if not indexer.is_file():
        raise PrepareError(
            f"Starling's class indexer was not found at {indexer}; "
            "check out the pinned upstream submodule before preparing."
        )

    tools_dir = str(indexer.parent.resolve())
    if tools_dir not in sys.path:
        sys.path.insert(0, tools_dir)
    try:
        import class_index
        inventory = class_index.index_jars([client_jar, server_jar], jobs=jobs)
    except Exception as exc:
        raise PrepareError(f"Starling's class indexer failed on the remapped 1.20.1 JARs: {exc}") from exc

    report_dir = work_dir / "private-index"
    report_dir.mkdir(parents=True, exist_ok=True)
    inventory_path = report_dir / "1.20.1-classes.json"
    inventory_path.write_text(
        json.dumps(inventory, separators=(",", ":")) + "\n", encoding="utf-8"
    )
    methods = sum(len(value.get("methods", {})) for value in inventory.values())
    prefixes = {}
    for name in inventory:
        prefix = name.split("/", 1)[0]
        prefixes[prefix] = prefixes.get(prefix, 0) + 1
    return {
        "status": "completed",
        "class_count": len(inventory),
        "method_count": methods,
        "class_prefix_counts": dict(sorted(prefixes.items())),
        "private_inventory_path": "private-index/1.20.1-classes.json",
    }


def prepare(work_dir: Path, java: str, maven: str) -> dict:
    work_dir.mkdir(parents=True, exist_ok=True)
    info, manifest_sha1 = load_official_version()
    facts_manifest = info.get("javaVersion", {}).get("majorVersion")
    if facts_manifest != 17:
        raise PrepareError(f"Expected Java 17 metadata for {VERSION}, got {facts_manifest!r}")

    downloads = info.get("downloads", {})
    required = ("client", "client_mappings", "server", "server_mappings")
    if any(name not in downloads for name in required):
        raise PrepareError("Official 1.20.1 metadata is missing one or more required client/server files")

    download_dir = work_dir / "downloads"
    mapping_dir = work_dir / "mappings"
    jar_dir = work_dir / "jars"
    mapped_dir = work_dir / "mapped"
    client = download_verified(
        downloads["client"]["url"], download_dir / "client.jar",
        downloads["client"]["sha1"], downloads["client"]["size"],
    )
    client_map = download_verified(
        downloads["client_mappings"]["url"], mapping_dir / "client.txt",
        downloads["client_mappings"]["sha1"], downloads["client_mappings"]["size"],
    )
    server_bundle = download_verified(
        downloads["server"]["url"], download_dir / "server-bundler.jar",
        downloads["server"]["sha1"], downloads["server"]["size"],
    )
    server_map = download_verified(
        downloads["server_mappings"]["url"], mapping_dir / "server.txt",
        downloads["server_mappings"]["sha1"], downloads["server_mappings"]["size"],
    )
    facts = read_game_facts(client, info)
    server_game, server_game_sha256 = extract_server_game(
        server_bundle, jar_dir / "server-game.jar", VERSION
    )

    data_generator_report = run_data_generator(java, server_bundle, work_dir)
    classpath = ensure_specialsource_classpath(work_dir, maven)
    client_mapped_classes = remap_jar(
        java, classpath, client, client_map, mapped_dir / "client-named.jar",
        work_dir / "logs" / "client-remap.log",
    )
    server_mapped_classes = remap_jar(
        java, classpath, server_game, server_map, mapped_dir / "server-named.jar",
        work_dir / "logs" / "server-remap.log",
    )

    with zipfile.ZipFile(mapped_dir / "client-named.jar") as archive:
        client_has_minecraft = "net/minecraft/client/Minecraft.class" in archive.namelist()
    with zipfile.ZipFile(mapped_dir / "server-named.jar") as archive:
        server_has_minecraft = "net/minecraft/server/MinecraftServer.class" in archive.namelist()
    if not client_has_minecraft or not server_has_minecraft:
        raise PrepareError(
            "Remapped JARs did not expose the expected named entry classes "
            f"(client={client_has_minecraft}, server={server_has_minecraft})"
        )

    starling_root = Path(os.environ.get("RUSTCHUNK_STARLING_ROOT", "upstream/starling"))
    code_inventory_report = index_remapped_game(
        mapped_dir / "client-named.jar",
        mapped_dir / "server-named.jar",
        starling_root,
        work_dir,
        jobs=int(os.environ.get("RUSTCHUNK_INDEX_JOBS", "2")),
    )

    report = {
        "status": "remapped",
        "version_metadata_sha1": manifest_sha1,
        "facts": facts,
        "inputs": {
            "client_sha1": downloads["client"]["sha1"],
            "client_mappings_sha1": downloads["client_mappings"]["sha1"],
            "server_bundler_sha1": downloads["server"]["sha1"],
            "server_mappings_sha1": downloads["server_mappings"]["sha1"],
            "server_game_sha256": server_game_sha256,
        },
        "data_generator": data_generator_report,
        "code_inventory": code_inventory_report,
        "remapper": {
            "name": "SpecialSource",
            "version": MAVEN_COORDINATES[2],
            "format": "Mojang ProGuard mappings (parsed directly; no intermediate conversion)",
            "license": "BSD-3-Clause",
        },
        "outputs": {
            "client_named": "mapped/client-named.jar",
            "server_named": "mapped/server-named.jar",
            "client_class_count": client_mapped_classes,
            "server_class_count": server_mapped_classes,
            "client_entry_check": client_has_minecraft,
            "server_entry_check": server_has_minecraft,
        },
        "note": "Metadata report only. JARs and mappings remain private in the ignored work directory.",
    }
    (work_dir / "audit-report.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8"
    )
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", type=Path, default=Path(".local/minecraft/1.20.1-remap"))
    parser.add_argument("--java", default="java")
    parser.add_argument("--maven", default=os.environ.get("MAVEN_EXECUTABLE", "mvn"))
    args = parser.parse_args()

    try:
        if java_major(args.java) != 17:
            raise PrepareError("The 1.20.1 preparation pipeline requires Java 17 exactly")
        report = prepare(args.work_dir.resolve(), args.java, args.maven)
    except (OSError, ValueError, KeyError, urllib.error.URLError, PrepareError) as exc:
        print(f"1.20.1 preparation failed: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
