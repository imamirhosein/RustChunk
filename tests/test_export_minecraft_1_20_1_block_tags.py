import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from tools.prepare_minecraft_1_20_1 import PrepareError
from tools.export_minecraft_1_20_1_block_tags import (
    read_block_tag_bundle,
    summarize_block_tags,
)


def make_jar(path: Path, values: dict[str, str], prefix: str = "data/minecraft/tags/blocks/"):
    with zipfile.ZipFile(path, "w") as archive:
        for name, content in values.items():
            archive.writestr(prefix + name + ".json", content)


class BlockTagExportTests(unittest.TestCase):
    def test_reads_plural_1_20_1_block_tag_root_and_preserves_vanilla_text(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            text = '{"replace":false,"values":["minecraft:stone",{"id":"minecraft:diamond_ore","required":false}]}'
            make_jar(jar, {"mineable/pickaxe": text})
            prefix, bundle = read_block_tag_bundle(jar)
            self.assertEqual(prefix, "data/minecraft/tags/blocks/")
            self.assertEqual(bundle, {"minecraft:mineable/pickaxe": text})

    def test_summary_counts_tag_entries_replace_flags_and_optional_entries(self):
        bundle = {
            "minecraft:mineable/pickaxe": json.dumps({
                "replace": True,
                "values": ["minecraft:stone", {"id": "minecraft:diamond_ore", "required": False}],
            }),
            "minecraft:logs": json.dumps({"values": ["#minecraft:logs_that_burn"]}),
        }
        report = summarize_block_tags(bundle, "data/minecraft/tags/blocks/")
        self.assertEqual(report["tag_file_count"], 2)
        self.assertEqual(report["tag_entry_count"], 3)
        self.assertEqual(report["replace_tag_count"], 1)
        self.assertEqual(report["optional_entry_count"], 1)
        self.assertEqual(report["tag_namespaces"], {"minecraft": 2})

    def test_rejects_ambiguous_singular_and_plural_roots(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            with zipfile.ZipFile(jar, "w") as archive:
                archive.writestr("data/minecraft/tags/blocks/stone.json", '{"values":[]}')
                archive.writestr("data/minecraft/tags/block/stone.json", '{"values":[]}')
            with self.assertRaisesRegex(PrepareError, "ambiguous"):
                read_block_tag_bundle(jar)

    def test_rejects_malformed_tag_entries(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            make_jar(jar, {"mineable/pickaxe": '{"values":[{"id":3}]}'})
            with self.assertRaisesRegex(PrepareError, "no string id"):
                read_block_tag_bundle(jar)

    def test_rejects_missing_block_tags(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            with zipfile.ZipFile(jar, "w") as archive:
                archive.writestr("data/minecraft/other.json", "{}")
            with self.assertRaisesRegex(PrepareError, "No Minecraft 1.20.1 block tags"):
                read_block_tag_bundle(jar)


if __name__ == "__main__":
    unittest.main()
