import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from tools.apply_starling_patches import PatchError, apply_series


class StarlingPatchSeriesTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.starling = self.root / "starling"
        self.patch_root = self.root / "patches"
        self.starling.mkdir()
        self.patch_root.mkdir()

        self.git("init", "-q")
        self.git("config", "user.name", "RustChunk CI")
        self.git("config", "user.email", "ci@example.invalid")
        (self.starling / "value.txt").write_text("before\n", encoding="utf-8")
        self.git("add", "value.txt")
        self.git("commit", "-q", "-m", "base")
        self.base = self.git("rev-parse", "HEAD")

    def tearDown(self):
        self.temp.cleanup()

    def git(self, *args):
        result = subprocess.run(
            ["git", "-C", str(self.starling), *args],
            text=True,
            capture_output=True,
            check=True,
        )
        return result.stdout.strip()

    def set_manifest(self, patches):
        (self.patch_root / "manifest.json").write_text(
            json.dumps({"base_commit": self.base, "patches": patches}),
            encoding="utf-8",
        )

    def write_patch(self, name, old, new):
        (self.patch_root / name).write_text(
            "--- a/value.txt\n"
            "+++ b/value.txt\n"
            "@@ -1 +1 @@\n"
            f"-{old}\n"
            f"+{new}\n",
            encoding="utf-8",
        )

    def test_clean_empty_series_only_verifies_pinned_base(self):
        self.set_manifest([])
        result = apply_series(self.starling, self.patch_root)
        self.assertEqual(result["patch_count"], 0)
        self.assertEqual((self.starling / "value.txt").read_text(), "before\n")

    def test_patch_applies_to_exact_base(self):
        self.write_patch("0001-value.patch", "before", "after")
        self.set_manifest(["0001-value.patch"])
        result = apply_series(self.starling, self.patch_root)
        self.assertEqual(result["patch_count"], 1)
        self.assertEqual((self.starling / "value.txt").read_text(), "after\n")

    def test_wrong_base_is_rejected_before_mutation(self):
        self.write_patch("0001-value.patch", "before", "after")
        self.set_manifest(["0001-value.patch"])
        manifest = self.patch_root / "manifest.json"
        value = json.loads(manifest.read_text())
        value["base_commit"] = "0" * 40
        manifest.write_text(json.dumps(value))
        with self.assertRaisesRegex(PatchError, "base commit mismatch"):
            apply_series(self.starling, self.patch_root)
        self.assertEqual((self.starling / "value.txt").read_text(), "before\n")

    def test_all_patches_are_preflighted_before_any_are_applied(self):
        self.write_patch("0001-valid.patch", "before", "after")
        self.write_patch("0002-invalid.patch", "does-not-match", "broken")
        self.set_manifest(["0001-valid.patch", "0002-invalid.patch"])
        with self.assertRaises(PatchError):
            apply_series(self.starling, self.patch_root)
        self.assertEqual((self.starling / "value.txt").read_text(), "before\n")

    def test_dependent_patches_are_preflighted_and_applied_in_order(self):
        self.write_patch("0001-first.patch", "before", "middle")
        self.write_patch("0002-second.patch", "middle", "after")
        self.set_manifest(["0001-first.patch", "0002-second.patch"])

        result = apply_series(self.starling, self.patch_root)

        self.assertEqual(result["patch_count"], 2)
        self.assertEqual((self.starling / "value.txt").read_text(), "after\n")

    def test_dirty_submodule_is_rejected(self):
        self.set_manifest([])
        (self.starling / "value.txt").write_text("local edit\n")
        with self.assertRaisesRegex(PatchError, "dirty Starling checkout"):
            apply_series(self.starling, self.patch_root)

    def test_unsafe_patch_path_is_rejected(self):
        self.set_manifest(["../escape.patch"])
        with self.assertRaisesRegex(PatchError, "Unsafe or invalid patch path"):
            apply_series(self.starling, self.patch_root)


if __name__ == "__main__":
    unittest.main()
