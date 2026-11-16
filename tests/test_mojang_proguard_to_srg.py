import unittest

from tools.mojang_proguard_to_srg import MappingError, convert_text


class ProGuardToSrgTests(unittest.TestCase):
    def test_classes_fields_and_line_numbered_methods(self):
        source = """
# Header comment
com.example.Main -> a:
    java.lang.String label -> b
    1:1:void main(java.lang.String[]):2:2 -> c
com.example.Helper -> b:
    void <init>() -> <init>
    com.example.Main run(com.example.Main,int[]) -> a
"""
        result = convert_text(source).splitlines()
        self.assertIn("CL: a com/example/Main", result)
        self.assertIn("CL: b com/example/Helper", result)
        self.assertIn("FD: a/b com/example/Main/label", result)
        self.assertIn(
            "MD: a/c ([Ljava/lang/String;)V com/example/Main/main ([Ljava/lang/String;)V",
            result,
        )
        self.assertIn(
            "MD: b/a (La;[I)La; com/example/Helper/run "
            "(Lcom/example/Main;[I)Lcom/example/Main;",
            result,
        )
        self.assertIn("MD: b/<init> ()V com/example/Helper/<init> ()V", result)

    def test_arrays_and_generic_erasure(self):
        source = """
example.Api -> q:
    java.util.List<java.lang.String> values(java.lang.String[],int[][]) -> a
"""
        result = convert_text(source)
        self.assertIn(
            "MD: q/a ([Ljava/lang/String;[[I)Ljava/util/List; "
            "example/Api/values ([Ljava/lang/String;[[I)Ljava/util/List;",
            result,
        )

    def test_identity_class_is_not_emitted(self):
        self.assertEqual(convert_text("java.lang.String -> java.lang.String:\n"), "\n")

    def test_member_before_class_is_rejected(self):
        with self.assertRaisesRegex(MappingError, "before a class"):
            convert_text("    void run() -> a\n")

    def test_unrecognized_class_line_is_rejected(self):
        with self.assertRaisesRegex(MappingError, "unrecognized class"):
            convert_text("not a valid mapping\n")

    def test_unbalanced_type_is_rejected(self):
        source = """
example.Api -> q:
    java.util.List<java.lang.String values() -> a
"""
        with self.assertRaises(MappingError):
            convert_text(source)


if __name__ == "__main__":
    unittest.main()
