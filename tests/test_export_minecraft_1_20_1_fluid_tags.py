import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from tools.prepare_minecraft_1_20_1 import PrepareError
from tools.export_minecraft_1_20_1_fluid_tags import (
    read_fluid_tag_bundle,
    summarize_fluid_tags,
)


def make_jar(path: Path, values: dict[str, str], prefix: str = "data/minecraft/tags/fluids/"):
    with zipfile.ZipFile(path, "w") as archive:
        for name, content in values.items():
            archive.writestr(prefix + name + ".json", content)


class FluidTagExportTests(unittest.TestCase):
    def test_reads_plural_1_20_1_fluid_tag_root(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            text = '{"values":["minecraft:water"]}'
            make_jar(jar, {"water": text})
            prefix, bundle = read_fluid_tag_bundle(jar)
            self.assertEqual(prefix, "data/minecraft/tags/fluids/")
            self.assertEqual(bundle, {"minecraft:water": text})

    def test_summary_counts_entries_and_reports_engine_tag_presence(self):
        bundle = {
            "minecraft:water": json.dumps({"values": ["minecraft:water", "minecraft:flowing_water"]}),
            "minecraft:lava": json.dumps({"values": ["minecraft:lava", "minecraft:flowing_lava"]}),
        }
        report = summarize_fluid_tags(bundle, "data/minecraft/tags/fluids/")
        self.assertEqual(report["tag_file_count"], 2)
        self.assertEqual(report["tag_entry_count"], 4)
        self.assertTrue(report["starling_fluid_tags"]["minecraft:water"])
        self.assertTrue(report["starling_fluid_tags"]["minecraft:lava"])
        self.assertIn("minecraft:supports_frogspawn", report["missing_starling_fluid_tags"])

    def test_rejects_ambiguous_singular_and_plural_roots(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            with zipfile.ZipFile(jar, "w") as archive:
                archive.writestr("data/minecraft/tags/fluids/water.json", '{"values":[]}')
                archive.writestr("data/minecraft/tags/fluid/water.json", '{"values":[]}')
            with self.assertRaisesRegex(PrepareError, "ambiguous"):
                read_fluid_tag_bundle(jar)

    def test_rejects_malformed_entries(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            make_jar(jar, {"water": '{"values":[{"id":7}]}'})
            with self.assertRaisesRegex(PrepareError, "no string id"):
                read_fluid_tag_bundle(jar)

    def test_rejects_missing_fluid_tags(self):
        with tempfile.TemporaryDirectory() as temp:
            jar = Path(temp) / "client.jar"
            with zipfile.ZipFile(jar, "w") as archive:
                archive.writestr("data/minecraft/other.json", "{}")
            with self.assertRaisesRegex(PrepareError, "No Minecraft 1.20.1 fluid tags"):
                read_fluid_tag_bundle(jar)


if __name__ == "__main__":
    unittest.main()
