#!/usr/bin/env python3
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path


class GlobalFreeFunctionChecker:
    COMMANDS = [
        ["ast-outline", "crates/app-cli/src", "crates/app-cli/tests", "--glob", "*.rs", "--no-docs"],
        ["ast-outline", "research/scripts", "--glob", "*.py", "--no-docs"],
        ["ast-outline", "apps/player/src", "apps/viewer/src", "docs/prototype.tsx", "--glob", "*.tsx", "--no-docs"],
        ["ast-outline", "apps/player/src", "apps/viewer/src", "--glob", "*.ts", "--no-docs"],
    ]

    SYMBOL_RE = re.compile(
        r"^(?:pub\s+fn|fn|def|function|export\s+function|const\s+[A-Za-z_]\w*\s*=.*=>)\b"
    )

    @classmethod
    def main(cls) -> int:
        repo = Path(__file__).resolve().parents[1]
        failures: list[str] = []
        for command in cls.COMMANDS:
            outline = subprocess.run(
                command,
                cwd=repo,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )
            if outline.returncode != 0:
                print(outline.stderr, file=sys.stderr)
                return outline.returncode
            failures.extend(cls.find_violations(outline.stdout))

        if failures:
            print("global free function policy failed:", file=sys.stderr)
            for failure in failures:
                print(f"  - {failure}", file=sys.stderr)
            return 1
        print("global free function policy passed")
        return 0

    @classmethod
    def find_violations(cls, outline: str) -> list[str]:
        failures: list[str] = []
        current_file = ""
        for line in outline.splitlines():
            if line.startswith("# "):
                current_file = line[2:].split(" (", 1)[0]
                continue
            if not current_file or not cls.SYMBOL_RE.match(line):
                continue
            if cls.is_allowed(current_file, line):
                continue
            failures.append(f"{current_file}: {line}")
        return failures

    @classmethod
    def is_allowed(cls, path: str, line: str) -> bool:
        if path == "crates/app-cli/src/main.rs" and (
            line.startswith("fn main() -> ExitCode")
            or line.startswith("fn run() -> Result<(), AppError>")
        ):
            return True
        if path.startswith("crates/app-cli/tests/") and line.startswith("#[test] fn "):
            return True
        if "/src/features/" in path:
            return True
        if path == "apps/viewer/src/mdxCoursePreview.tsx":
            return True
        if path == "docs/prototype.tsx":
            return True
        return False


if __name__ == "__main__":
    raise SystemExit(GlobalFreeFunctionChecker.main())
