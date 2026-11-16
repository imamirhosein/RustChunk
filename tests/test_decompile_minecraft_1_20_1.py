import json
from pathlib import Path
import tempfile
import unittest

from tools.decompile_minecraft_1_20_1 import (
    PrepareError,
    library_artifacts,
    safe_library_path,
)


class Minecraft1201DecompileTests(unittest.TestCase):
    def test_safe_library_path_accepts_maven_relative_paths(self):
        path = safe_library_path("com/google/guava/guava/31.1-jre/guava-31.1-jre.jar")
        self.assertEqual(path.as_posix(), "com/google/guava/guava/31.1-jre/guava-31.1-jre.jar")

    def test_safe_library_path_rejects_traversal_and_absolute_paths(self):
        for value in ("../escape.jar", "/tmp/escape.jar", "a/../../escape.jar", ""):
            with self.subTest(value=value):
                with self.assertRaises(PrepareError):
                    safe_library_path(value)

    def test_library_artifacts_are_deduplicated_and_sorted(self):
        version_info = {
            "libraries": [
                {
                    "name": "z:last:1",
                    "downloads": {"artifact": {
                        "path": "z/last/1/last-1.jar",
                        "url": "https://example.invalid/last.jar",
                        "sha1": "a" * 40,
                        "size": 3,
                    }},
                },
                {
                    "name": "native:only:1",
                    "downloads": {"classifiers": {"natives-linux": {"path": "natives.jar"}}},
                },
                {
                    "name": "a:first:1",
                    "downloads": {"artifact": {
                        "path": "a/first/1/first-1.jar",
                        "url": "https://example.invalid/first.jar",
                        "sha1": "b" * 40,
                        "size": 5,
                    }},
                },
                {
                    "name": "a:first:duplicate",
                    "downloads": {"artifact": {
                        "path": "a/first/1/first-1.jar",
                        "url": "https://example.invalid/first.jar",
                        "sha1": "b" * 40,
                        "size": 5,
                    }},
                },
            ]
        }
        artifacts = library_artifacts(version_info)
        self.assertEqual(
            [item["path"] for item in artifacts],
            ["a/first/1/first-1.jar", "z/last/1/last-1.jar"],
        )


if __name__ == "__main__":
    unittest.main()
