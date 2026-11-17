import unittest

from tools.prepare_minecraft_1_20_1 import PrepareError
from tools.export_minecraft_1_20_1_heightmap_facts import summarize_heightmap_facts


def valid_facts():
    return {
        "minecraft_version": "1.20.1",
        "schema": "rustchunk-heightmap-facts-v1",
        "predicate_source": "Heightmap.Types.MOTION_BLOCKING.isOpaque",
        "no_leaves_predicate_source": "Heightmap.Types.MOTION_BLOCKING_NO_LEAVES.isOpaque",
        "states": [
            [0, "minecraft:air", "Block{minecraft:air}", False, False],
            [1, "minecraft:stone", "Block{minecraft:stone}", True, True],
            [2, "minecraft:oak_leaves", "Block{minecraft:oak_leaves}", True, False],
        ],
    }


class HeightmapFactsSummaryTests(unittest.TestCase):
    def test_accepts_complete_ordered_java_oracle_rows(self):
        report = summarize_heightmap_facts(valid_facts(), expected_states=3)
        self.assertEqual(report["schema"], "rustchunk-heightmap-facts-v1")
        self.assertEqual(report["block_state_count"], 3)
        self.assertEqual(report["motion_blocking_state_count"], 2)
        self.assertEqual(report["motion_blocking_no_leaves_state_count"], 1)

    def test_rejects_wrong_version_or_schema(self):
        facts = valid_facts()
        facts["minecraft_version"] = "1.20.2"
        with self.assertRaisesRegex(PrepareError, "do not identify Minecraft 1.20.1"):
            summarize_heightmap_facts(facts, expected_states=3)

        facts = valid_facts()
        facts["schema"] = "unknown"
        with self.assertRaisesRegex(PrepareError, "Unsupported heightmap facts schema"):
            summarize_heightmap_facts(facts, expected_states=3)

    def test_rejects_wrong_state_count(self):
        with self.assertRaisesRegex(PrepareError, "block-state count"):
            summarize_heightmap_facts(valid_facts(), expected_states=4)

    def test_rejects_gaps_or_reordered_ids(self):
        facts = valid_facts()
        facts["states"][1][0] = 2
        with self.assertRaisesRegex(PrepareError, "contiguous and ordered"):
            summarize_heightmap_facts(facts, expected_states=3)

    def test_rejects_invalid_names_and_descriptions(self):
        facts = valid_facts()
        facts["states"][1][1] = "stone"
        with self.assertRaisesRegex(PrepareError, "invalid block identifier"):
            summarize_heightmap_facts(facts, expected_states=3)

        facts = valid_facts()
        facts["states"][1][2] = ""
        with self.assertRaisesRegex(PrepareError, "no state description"):
            summarize_heightmap_facts(facts, expected_states=3)

    def test_rejects_non_boolean_predicates(self):
        facts = valid_facts()
        facts["states"][1][3] = 1
        with self.assertRaisesRegex(PrepareError, "predicates must be booleans"):
            summarize_heightmap_facts(facts, expected_states=3)

    def test_rejects_malformed_rows(self):
        facts = valid_facts()
        facts["states"][1] = [1, "minecraft:stone"]
        with self.assertRaisesRegex(PrepareError, "exactly five columns"):
            summarize_heightmap_facts(facts, expected_states=3)


if __name__ == "__main__":
    unittest.main()
