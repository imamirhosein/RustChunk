import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from tools.prepare_minecraft_1_20_1 import (
    PrepareError,
    digest_file,
    extract_server_game,
    parse_server_bundle_entry,
    read_game_facts,
    version_facts,
)


class Minecraft1201PreparationTests(unittest.TestCase):
    def test_parse_server_bundle_entry(self):
        sha = "a" * 64
        listing = f"{sha} 1.20.1 1.20.1/server-1.20.1.jar\n"
        self.assertEqual(
            parse_server_bundle_entry(listing, "1.20.1"),
            (sha, "1.20.1/server-1.20.1.jar"),
        )

    def test_missing_or_duplicate_server_bundle_entry_fails(self):
        with self.assertRaises(PrepareError):
            parse_server_bundle_entry("", "1.20.1")
        listing = "a 1.20.1 a.jar\nb 1.20.1 b.jar\n"
        with self.assertRaises(PrepareError):
            parse_server_bundle_entry(listing, "1.20.1")

    def test_extract_server_game_verifies_sha256(self):
        payload = b"fake-but-valid-test-game-jar-payload"
        sha = hashlib.sha256(payload).hexdigest()
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            bundler = root / "server-bundler.jar"
            with zipfile.ZipFile(bundler, "w") as archive:
                archive.writestr("META-INF/versions.list", f"{sha} 1.20.1 1.20.1/server.jar\n")
                archive.writestr("META-INF/versions/1.20.1/server.jar", payload)
            output, result_sha = extract_server_game(bundler, root / "out/server-game.jar", "1.20.1")
            self.assertEqual(output.read_bytes(), payload)
            self.assertEqual(result_sha, sha)

    def test_bad_server_bundle_hash_is_rejected(self):
        payload = b"tampered-payload"
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            bundler = root / "server-bundler.jar"
            with zipfile.ZipFile(bundler, "w") as archive:
                archive.writestr("META-INF/versions.list", f"{'0' * 64} 1.20.1 1.20.1/server.jar\n")
                archive.writestr("META-INF/versions/1.20.1/server.jar", payload)
            with self.assertRaisesRegex(PrepareError, "SHA-256 mismatch"):
                extract_server_game(bundler, root / "out/server-game.jar", "1.20.1")

    def test_game_facts_match_target(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            client = root / "client.jar"
            game_info = {
                "id": "1.20.1",
                "protocol_version": 763,
                "world_version": 3465,
                "pack_version": {"resource": 15, "data": 15},
            }
            with zipfile.ZipFile(client, "w") as archive:
                archive.writestr("version.json", json.dumps(game_info))
            facts = read_game_facts(client, {"javaVersion": {"majorVersion": 17}})
            self.assertEqual(facts["protocol"], 763)
            self.assertEqual(facts["data_version"], 3465)
            self.assertEqual(facts["java_major"], 17)
            self.assertEqual(facts["resource_pack_format"], 15)
            self.assertEqual(facts["data_pack_format"], 15)


    def test_newer_pack_version_field_names_remain_supported(self):
        facts = version_facts(
            {
                "id": "26.3",
                "protocol_version": 777,
                "world_version": 5023,
                "pack_version": {"resource_major": 97, "data_major": 121},
            },
            {"javaVersion": {"majorVersion": 25}},
        )
        self.assertEqual(facts["resource_pack_format"], 97)
        self.assertEqual(facts["data_pack_format"], 121)

    def test_wrong_version_facts_are_rejected(self):
        with tempfile.TemporaryDirectory() as temp:
            client = Path(temp) / "client.jar"
            with zipfile.ZipFile(client, "w") as archive:
                archive.writestr(
                    "version.json",
                    json.dumps({
                        "id": "26.3", "protocol_version": 777, "world_version": 5023,
                        "pack_version": {"resource_major": 97, "data_major": 121},
                    }),
                )
            with self.assertRaisesRegex(PrepareError, "Client JAR identifies itself"):
                read_game_facts(client, {"javaVersion": {"majorVersion": 17}})

    def test_summarize_data_reports_keeps_metadata_not_game_data(self):
        from tools.prepare_minecraft_1_20_1 import summarize_data_reports

        with tempfile.TemporaryDirectory() as temp:
            reports = Path(temp)
            (reports / "minecraft").mkdir()
            (reports / "blocks.json").write_text(
                json.dumps({"minecraft:stone": {}, "minecraft:dirt": {}}),
                encoding="utf-8",
            )
            (reports / "minecraft" / "nested.json").write_text(
                json.dumps({"secret_game_data": "not copied into report"}),
                encoding="utf-8",
            )
            summary = summarize_data_reports(reports)
            self.assertEqual(summary["json_file_count"], 2)
            self.assertEqual(summary["top_level_reports"]["blocks.json"]["entry_count"], 2)
            self.assertNotIn("minecraft", summary["top_level_reports"])
            self.assertNotIn("secret_game_data", json.dumps(summary))


    def test_digest_file(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "x.bin"
            path.write_bytes(b"abc")
            self.assertEqual(
                digest_file(path),
                "a9993e364706816aba3e25717850c26c9cd0d89d",
            )


if __name__ == "__main__":
    unittest.main()
