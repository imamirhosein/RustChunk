#!/usr/bin/env python3
"""Convert Mojang's official ProGuard mappings to SRG for SpecialSource.

Input mappings are ProGuard format: readable/name -> obfuscated/name.
Output is oriented from the obfuscated JAR to the readable Mojang names:
  CL: obfuscated/class readable/class
  FD: obfuscated/class/field readable/class/field
  MD: obfuscated/class/method obfuscated/descriptor readable/class/method readable/descriptor

Game JARs and mapping files are private build inputs. This tool emits only a
mapping file; it does not include or redistribute Minecraft code or assets.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

CLASS_LINE = re.compile(r"^(?P<named>\S+)\s+->\s+(?P<obf>\S+):$")
METHOD_LINE = re.compile(
    r"^(?:(?:\d+):(?:\d+):)?"
    r"(?P<return>\S+)\s+(?P<name>[^\s(]+)"
    r"\((?P<arguments>.*)\)"
    r"(?::\d+:\d+)?$"
)
PRIMITIVES = {
    "void": "V",
    "boolean": "Z",
    "byte": "B",
    "char": "C",
    "short": "S",
    "int": "I",
    "long": "J",
    "float": "F",
    "double": "D",
}


class MappingError(ValueError):
    """Raised for a ProGuard mapping line that cannot be translated safely."""


def _erase_generics(java_type: str) -> str:
    result: list[str] = []
    depth = 0
    for char in java_type.strip():
        if char == "<":
            depth += 1
        elif char == ">":
            depth -= 1
            if depth < 0:
                raise MappingError(f"unbalanced generic type: {java_type!r}")
        elif depth == 0:
            result.append(char)
    if depth:
        raise MappingError(f"unbalanced generic type: {java_type!r}")
    return "".join(result).replace("...", "[]").strip()


def _split_arguments(raw: str) -> list[str]:
    if not raw.strip():
        return []
    args: list[str] = []
    start = 0
    depth = 0
    for index, char in enumerate(raw):
        if char == "<":
            depth += 1
        elif char == ">":
            depth -= 1
            if depth < 0:
                raise MappingError(f"unbalanced method arguments: {raw!r}")
        elif char == "," and depth == 0:
            args.append(raw[start:index].strip())
            start = index + 1
    if depth:
        raise MappingError(f"unbalanced method arguments: {raw!r}")
    args.append(raw[start:].strip())
    if any(not arg for arg in args):
        raise MappingError(f"empty method argument in: {raw!r}")
    return args


def _descriptor(java_type: str, class_names: dict[str, str] | None = None) -> str:
    """Return a JVM descriptor for a Java type, optionally mapping class names."""
    value = _erase_generics(java_type).replace("/", ".")
    dimensions = 0
    while value.endswith("[]"):
        dimensions += 1
        value = value[:-2].strip()
    if value in PRIMITIVES:
        if value == "void" and dimensions:
            raise MappingError("void arrays are not valid JVM types")
        base = PRIMITIVES[value]
    else:
        if not value or any(token in value for token in (" ", "?", "&", "|")):
            raise MappingError(f"unsupported Java type in official mapping: {java_type!r}")
        internal = value.replace(".", "/")
        if class_names is not None:
            internal = class_names.get(value, internal)
            internal = internal.replace(".", "/")
        base = f"L{internal};"
    return "[" * dimensions + base


def _method_descriptor(return_type: str, arguments: str,
                       class_names: dict[str, str] | None = None) -> str:
    params = "".join(_descriptor(arg, class_names) for arg in _split_arguments(arguments))
    return f"({params}){_descriptor(return_type, class_names)}"


def convert_text(mapping_text: str) -> str:
    """Translate one official ProGuard mapping file into deterministic SRG text."""
    class_map: dict[str, str] = {}
    member_lines: list[tuple[int, str, str]] = []
    current_named: str | None = None
    current_obf: str | None = None

    for line_number, raw_line in enumerate(mapping_text.splitlines(), start=1):
        if not raw_line.strip() or raw_line.lstrip().startswith("#"):
            continue
        if raw_line[0].isspace():
            if current_named is None or current_obf is None:
                raise MappingError(f"line {line_number}: member appears before a class mapping")
            if "->" not in raw_line:
                raise MappingError(f"line {line_number}: expected '->' in member mapping")
            left, right = (part.strip() for part in raw_line.split("->", 1))
            if not left or not right or any(ch.isspace() for ch in right):
                raise MappingError(f"line {line_number}: malformed member mapping")
            member_lines.append((line_number, current_named, f"{current_obf}\t{left}\t{right}"))
            continue

        match = CLASS_LINE.fullmatch(raw_line.strip())
        if not match:
            raise MappingError(f"line {line_number}: unrecognized class mapping: {raw_line!r}")
        current_named = match.group("named")
        current_obf = match.group("obf")
        if current_named in class_map and class_map[current_named] != current_obf:
            raise MappingError(f"line {line_number}: duplicate class mapping for {current_named}")
        class_map[current_named] = current_obf

    output: set[str] = set()
    for named, obf in class_map.items():
        named_internal = named.replace(".", "/")
        obf_internal = obf.replace(".", "/")
        if named_internal != obf_internal:
            output.add(f"CL: {obf_internal} {named_internal}")

    for line_number, named_owner, payload in member_lines:
        obf_owner, left, obf_name = payload.split("\t", 2)
        named_owner_internal = named_owner.replace(".", "/")
        obf_owner_internal = obf_owner.replace(".", "/")

        if "(" in left:
            match = METHOD_LINE.fullmatch(left)
            if not match:
                raise MappingError(f"line {line_number}: malformed method signature: {left!r}")
            named_name = match.group("name")
            named_desc = _method_descriptor(match.group("return"), match.group("arguments"))
            obf_desc = _method_descriptor(match.group("return"), match.group("arguments"), class_map)
            output.add(
                f"MD: {obf_owner_internal}/{obf_name} {obf_desc} "
                f"{named_owner_internal}/{named_name} {named_desc}"
            )
        else:
            # ProGuard field syntax is: '<type> <readable-field-name> -> <obfuscated-name>'.
            tokens = left.split()
            if len(tokens) < 2:
                raise MappingError(f"line {line_number}: malformed field signature: {left!r}")
            named_name = tokens[-1]
            if any(ch in named_name for ch in "();,"):
                raise MappingError(f"line {line_number}: malformed field name: {named_name!r}")
            output.add(f"FD: {obf_owner_internal}/{obf_name} {named_owner_internal}/{named_name}")

    # SRG parsers expect classes before members; sorting also makes output reproducible.
    kind_order = {"CL": 0, "FD": 1, "MD": 2}
    return "\n".join(sorted(output, key=lambda row: (kind_order[row[:2]], row))) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="official Mojang ProGuard mappings")
    parser.add_argument("output", type=Path, help="output SRG mapping file")
    args = parser.parse_args()

    try:
        converted = convert_text(args.input.read_text(encoding="utf-8"))
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(converted, encoding="utf-8", newline="\n")
    except (OSError, MappingError) as exc:
        print(f"mapping conversion failed: {exc}", file=sys.stderr)
        return 2

    counts = {kind: sum(line.startswith(kind + ":") for line in converted.splitlines())
              for kind in ("CL", "FD", "MD")}
    print(f"Wrote {args.output}: {counts['CL']} classes, {counts['FD']} fields, {counts['MD']} methods")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
