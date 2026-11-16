import json
import unittest

from tools.prepare_minecraft_1_20_1 import PrepareError
from tools.export_minecraft_1_20_1_block_facts import summarize_facts


def valid_facts():
    return {
        "states": [
            ["minecraft:air", True, False, "Block{minecraft:air}", "minecraft:air"],
            ["minecraft:stone", False, False, "Block{minecraft:stone}", "minecraft:stone"],
            ["minecraft:oak_slab", False, False, "Block{minecraft:oak_slab}[type=bottom]",
             {"id": "minecraft:oak_slab", "properties": {"type": "bottom"}}],
            ["minecraft:oak_slab", False, False, "Block{minecraft:oak_slab}[type=top]",
             {"id": "minecraft:oak_slab", "properties": {"type": "top"}}],
            ["minecraft:water", False, True, "Block{minecraft:water}[level=0]",
             {"id": "minecraft:water", "properties": {"level": "0"}}],
        ],
        "defaults": {
            "minecraft:air": 0,
            "minecraft:stone": 1,
            "minecraft:oak_slab": 2,
            "minecraft:water": 4,
        },
    }


class BlockFactsSummaryTests(unittest.TestCase):
    def test_accepts_fact_shape_consumed_by_horizon_block_registry(self):
        report = summarize_facts(valid_facts(), expected_blocks=4, expected_states=5)
        self.assertEqual(report["schema"], "horizon-block-facts-v1")
        self.assertEqual(report["block_count"], 4)
        self.assertEqual(report["block_state_count"], 5)
        self.assertEqual(report["air_state_count"], 1)
        self.assertEqual(report["fluid_state_count"], 1)
        self.assertEqual(report["states_with_properties"], 3)

    def test_rejects_duplicate_state_descriptors(self):
        facts = valid_facts()
        facts["states"][3] = list(facts["states"][2])
        with self.assertRaisesRegex(PrepareError, "Duplicate block state descriptor"):
            summarize_facts(facts, expected_blocks=4, expected_states=5)

    def test_rejects_default_state_that_points_to_another_block(self):
        facts = valid_facts()
        facts["defaults"]["minecraft:oak_slab"] = 1
        with self.assertRaisesRegex(PrepareError, "does not belong"):
            summarize_facts(facts, expected_blocks=4, expected_states=5)

    def test_rejects_block_registry_names_missing_defaults(self):
        facts = valid_facts()
        facts["defaults"].pop("minecraft:water")
        with self.assertRaisesRegex(PrepareError, "Default-state registry mismatch"):
            summarize_facts(facts, expected_blocks=3, expected_states=5)

    def test_rejects_wrong_property_type(self):
        facts = valid_facts()
        facts["states"][2][4]["properties"]["type"] = 1
        with self.assertRaisesRegex(PrepareError, "string pairs"):
            summarize_facts(facts, expected_blocks=4, expected_states=5)


if __name__ == "__main__":
    unittest.main()
