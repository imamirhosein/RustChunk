import json
import unittest

from tools.prepare_minecraft_1_20_1 import PrepareError
from tools.export_minecraft_1_20_1_heightmap_facts import (
    resolve_block_tag,
    summarize_heightmap_facts,
    write_resolved_leaves_tag,
)


def valid_facts():
    return {
        "minecraft_version": "1.20.1",
        "schema": "rustchunk-heightmap-facts-v2",
        "predicate_source": "Heightmap.Types.MOTION_BLOCKING.isOpaque",
        "no_leaves_predicate_source": "Heightmap.Types.MOTION_BLOCKING_NO_LEAVES.isOpaque",
        "leaves_tag_probe": "BlockState.is(BlockTags.LEAVES)",
        "states": [
            [0, "minecraft:air", "Block{minecraft:air}", True, False, False, False, False, False],
            [1, "minecraft:stone", "Block{minecraft:stone}", False, True, False, False, True, True],
            [2, "minecraft:oak_leaves", "Block{minecraft:oak_leaves}", False, True, False, True, True, True],
        ],
    }


def valid_registry():
    return [
        ["minecraft:air", True, False, "Block{minecraft:air}", "minecraft:air"],
        ["minecraft:stone", False, False, "Block{minecraft:stone}", "minecraft:stone"],
        ["minecraft:oak_leaves", False, False, "Block{minecraft:oak_leaves}", "minecraft:oak_leaves"],
    ]


def valid_tags():
    return {
        "minecraft:leaves": json.dumps({"values": ["#minecraft:tree_leaves"]}),
        "minecraft:tree_leaves": json.dumps({"values": ["minecraft:oak_leaves"]}),
    }


class HeightmapFactsSummaryTests(unittest.TestCase):
    def summarize(self, facts=None, registry=None, tags=None):
        return summarize_heightmap_facts(
            facts or valid_facts(),
            expected_states=3,
            registry_states=registry or valid_registry(),
            block_tags_bundle=tags or valid_tags(),
        )

    def test_preserves_complete_official_java_oracle_rows(self):
        report = self.summarize()
        self.assertEqual(report["schema"], "rustchunk-heightmap-facts-v2")
        self.assertEqual(report["block_state_count"], 3)
        self.assertEqual(report["air_state_count"], 1)
        self.assertEqual(report["blocks_motion_state_count"], 2)
        self.assertEqual(report["fluid_state_count"], 0)
        self.assertEqual(report["leaves_block_count"], 1)
        self.assertEqual(report["leaves_state_count"], 1)
        self.assertEqual(report["motion_blocking_state_count"], 2)
        self.assertEqual(report["motion_blocking_no_leaves_state_count"], 2)
        self.assertEqual(report["motion_blocking_excluded_by_no_leaves_count"], 0)
        self.assertEqual(
            report["no_leaves_predicate_validation"],
            "retained_per_state_from_official_1_20_1_Java_oracle; leaves membership independently cross-checked",
        )

    def test_writes_resolved_leaves_tag_as_parseable_json(self):
        import tempfile
        from pathlib import Path

        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "leaves-tag-input.json"
            write_resolved_leaves_tag(
                path,
                {"minecraft:oak_leaves", "minecraft:birch_leaves"},
            )
            self.assertEqual(
                json.loads(path.read_text(encoding="utf-8")),
                ["minecraft:birch_leaves", "minecraft:oak_leaves"],
            )
            self.assertTrue(path.read_text(encoding="utf-8").endswith("\n"))

    def test_resolves_nested_block_tags(self):
        self.assertEqual(
            resolve_block_tag("minecraft:leaves", valid_tags(), set(valid_registry()[i][0] for i in range(3))),
            {"minecraft:oak_leaves"},
        )

    def test_rejects_missing_required_tag_and_cycles(self):
        with self.assertRaisesRegex(PrepareError, "Missing required block tag"):
            resolve_block_tag("minecraft:missing", {}, {"minecraft:stone"})

        cyclic = {
            "minecraft:a": json.dumps({"values": ["#minecraft:b"]}),
            "minecraft:b": json.dumps({"values": ["#minecraft:a"]}),
        }
        with self.assertRaisesRegex(PrepareError, "Cyclic block-tag reference"):
            resolve_block_tag("minecraft:a", cyclic, {"minecraft:stone"})

    def test_ignores_optional_missing_tag_and_optional_block(self):
        bundle = {
            "minecraft:leaves": json.dumps({
                "values": [
                    {"id": "#minecraft:not_installed", "required": False},
                    {"id": "minecraft:modded_leaves", "required": False},
                    "minecraft:oak_leaves",
                ]
            })
        }
        self.assertEqual(
            resolve_block_tag("minecraft:leaves", bundle, {"minecraft:oak_leaves"}),
            {"minecraft:oak_leaves"},
        )

    def test_rejects_wrong_version_or_schema(self):
        facts = valid_facts()
        facts["minecraft_version"] = "1.20.2"
        with self.assertRaisesRegex(PrepareError, "do not identify Minecraft 1.20.1"):
            self.summarize(facts)

        facts = valid_facts()
        facts["schema"] = "unknown"
        with self.assertRaisesRegex(PrepareError, "Unsupported heightmap facts schema"):
            self.summarize(facts)

    def test_rejects_wrong_state_count_and_reordered_ids(self):
        with self.assertRaisesRegex(PrepareError, "block-state count"):
            summarize_heightmap_facts(
                valid_facts(), expected_states=4,
                registry_states=valid_registry(), block_tags_bundle=valid_tags()
            )

        facts = valid_facts()
        facts["states"][1][0] = 2
        with self.assertRaisesRegex(PrepareError, "contiguous and ordered"):
            self.summarize(facts)

    def test_rejects_invalid_names_descriptions_and_non_boolean_facts(self):
        facts = valid_facts()
        facts["states"][1][1] = "stone"
        with self.assertRaisesRegex(PrepareError, "invalid block identifier"):
            self.summarize(facts)

        facts = valid_facts()
        facts["states"][1][2] = ""
        with self.assertRaisesRegex(PrepareError, "no state description"):
            self.summarize(facts)

        facts = valid_facts()
        facts["states"][1][4] = 1
        with self.assertRaisesRegex(PrepareError, "must be booleans"):
            self.summarize(facts)

    def test_cross_checks_block_names_descriptions_air_and_fluid_against_registry(self):
        registry = valid_registry()
        self.assertEqual(self.summarize(registry=registry)["block_state_count"], 3)

        registry[1][3] = "Block{minecraft:dirt}"
        with self.assertRaisesRegex(PrepareError, "does not align"):
            self.summarize(registry=registry)

        registry = valid_registry()
        registry[1][1] = True
        with self.assertRaisesRegex(PrepareError, "primitive facts"):
            self.summarize(registry=registry)

    def test_rejects_java_leaves_tag_probe_disagreement(self):
        facts = valid_facts()
        facts["states"][2][6] = False
        with self.assertRaisesRegex(PrepareError, "BlockTags.LEAVES membership disagrees"):
            self.summarize(facts)

    def test_fluid_remains_motion_blocking_even_when_state_is_in_leaves_tag(self):
        facts = valid_facts()
        facts["states"].append([
            3,
            "minecraft:oak_leaves",
            "Block{minecraft:oak_leaves}[waterlogged=true]",
            False,
            True,
            True,
            True,
            True,
            True,
        ])
        registry = valid_registry()
        registry.append([
            "minecraft:oak_leaves",
            False,
            True,
            "Block{minecraft:oak_leaves}[waterlogged=true]",
            {"id": "minecraft:oak_leaves", "properties": {"waterlogged": "true"}},
        ])
        report = summarize_heightmap_facts(
            facts,
            expected_states=4,
            registry_states=registry,
            block_tags_bundle=valid_tags(),
        )
        self.assertEqual(report["motion_blocking_state_count"], 3)
        self.assertEqual(report["motion_blocking_no_leaves_state_count"], 3)

    def test_rejects_motion_blocking_java_predicate_mismatch(self):
        facts = valid_facts()
        facts["states"][1][7] = False
        with self.assertRaisesRegex(PrepareError, "MOTION_BLOCKING Java predicate mismatch"):
            self.summarize(facts)


if __name__ == "__main__":
    unittest.main()
