import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from tools.compare_starling_baseline import (
    class_diff_summary,
    command_node_count,
    jar_path_diff,
    report_features,
)


class StarlingBaselineComparisonTests(unittest.TestCase):
    def test_command_node_count(self):
        tree = {
            "type": "root",
            "children": {
                "say": {"type": "literal", "executable": True, "children": {}},
                "execute": {
                    "type": "literal",
                    "children": {"as": {"type": "literal", "children": {}}},
                },
            },
        }
        self.assertEqual(command_node_count(tree), 3)

    def test_jar_path_diff_reports_counts_only(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            old = root / "old.jar"
            new = root / "new.jar"
            with zipfile.ZipFile(old, "w") as archive:
                archive.writestr("assets/minecraft/a.png", b"old")
                archive.writestr("assets/minecraft/b.png", b"same")
                archive.writestr("data/minecraft/worldgen/a.json", b"{}")
            with zipfile.ZipFile(new, "w") as archive:
                archive.writestr("assets/minecraft/b.png", b"same")
                archive.writestr("assets/minecraft/c.png", b"new")
                archive.writestr("data/minecraft/worldgen/a.json", b'{"new":true}')
            self.assertEqual(
                jar_path_diff(old, new, "assets/minecraft/"),
                {
                    "files_1_20_1": 2,
                    "files_26_3": 2,
                    "added_in_26_3": 1,
                    "removed_since_1_20_1": 1,
                    "changed_crc": 0,
                },
            )

    def test_report_features_handles_legacy_reports_without_packets_or_components(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "biome_parameters/minecraft").mkdir(parents=True)
            (root / "blocks.json").write_text(json.dumps({
                "minecraft:stone": {"states": [{"id": 1}, {"id": 2}]},
                "minecraft:dirt": {"states": [{"id": 3}]},
            }), encoding="utf-8")
            (root / "registries.json").write_text(json.dumps({
                "minecraft:block": {"entries": {"minecraft:stone": {}, "minecraft:dirt": {}}}
            }), encoding="utf-8")
            (root / "commands.json").write_text(json.dumps({
                "type": "root", "children": {"say": {"children": {}}}
            }), encoding="utf-8")
            (root / "biome_parameters/minecraft/overworld.json").write_text("{}", encoding="utf-8")
            result = report_features(root)
            self.assertEqual(result["blocks"], 2)
            self.assertEqual(result["block_states"], 3)
            self.assertEqual(result["registries"], 1)
            self.assertEqual(result["registry_entries"], 2)
            self.assertEqual(result["command_nodes"], 1)
            self.assertFalse(result["packets_report_present"])
            self.assertEqual(result["item_component_report_files"], 0)
            self.assertEqual(result["biome_parameter_reports"], 1)

    def test_class_diff_summary_is_aggregate(self):
        class FakeIndexer:
            @staticmethod
            def diff(old, new):
                return {
                    "added": ["net/New"],
                    "removed": ["net/Gone"],
                    "moved": {"net/Old": "net/NewName"},
                    "changed": {"net/Changed": {
                        "methods": {"added": ["a"], "removed": [], "changed": ["b"]}
                    }},
                }

            @staticmethod
            def changed_methods(change):
                methods = change.get("methods", {})
                return sum(len(methods.get(key, [])) for key in ("added", "removed", "changed"))

        result = class_diff_summary({"net/Old": {}}, {"net/New": {}}, FakeIndexer)
        self.assertEqual(result["classes_1_20_1"], 1)
        self.assertEqual(result["classes_26_3"], 1)
        self.assertEqual(result["added_in_26_3"], 1)
        self.assertEqual(result["removed_since_1_20_1"], 1)
        self.assertEqual(result["moved"], 1)
        self.assertEqual(result["changed"], 1)
        self.assertEqual(result["changed_methods"], 2)
        self.assertNotIn("net/Changed", json.dumps(result))


if __name__ == "__main__":
    unittest.main()
