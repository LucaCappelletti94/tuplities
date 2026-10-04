#!/usr/bin/env python3
"""Validate Python files and quoted Python heredocs without executing them."""

import ast
import re
import sys
from pathlib import Path


HEADER = re.compile(
    r"^[^\n]*\bpython(?:\d(?:\.\d+)?)?\b[^\n]*?<<\s*"
    r"(?P<quote>['\"])(?P<delimiter>PY(?:THON)?)(?P=quote)[^\n]*\n",
    re.MULTILINE,
)


def check(path):
    source = path.read_text()
    if path.suffix == ".py":
        ast.parse(source, filename=str(path))
        return
    for header in HEADER.finditer(source):
        if header.group().lstrip().startswith("#"):
            continue
        closing = re.search(
            rf"^{re.escape(header['delimiter'])}[ \t]*$",
            source[header.end():],
            re.MULTILINE,
        )
        if closing is None:
            raise ValueError(f"{path} has an unterminated Python heredoc")
        body = source[header.end():header.end() + closing.start()]
        padding = "\n" * source[:header.end()].count("\n")
        ast.parse(padding + body, filename=str(path))


def main():
    paths = [Path(argument) for argument in sys.argv[1:]]
    if not paths:
        tools = Path(__file__).resolve().parent
        paths = sorted([*tools.rglob("*.py"), *tools.rglob("*.sh")])
    for path in paths:
        check(path)
    print(f"Python syntax verified in {len(paths)} harness files")


if __name__ == "__main__":
    main()
