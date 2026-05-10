#!/usr/bin/env python3
"""Layer 8: real-artifact tests.

Each [[real_artifact_test]] entry says: a test matching the pattern
must exist in the package, and that test must `include_str!` the
named artifact.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, load_manifest, section


def find_package_dir(package: str) -> Path | None:
    for cargo in REPO_ROOT.rglob("Cargo.toml"):
        if "/target/" in str(cargo) or "/.cargo-target/" in str(cargo):
            continue
        try:
            content = cargo.read_text()
        except Exception:
            continue
        m = re.search(r'^\s*name\s*=\s*"([^"]+)"', content, re.MULTILINE)
        if m and m.group(1) == package:
            return cargo.parent
    return None


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "real_artifact_test"):
        package = entry["package"]
        artifact = entry["artifact"]
        pattern = entry["test_pattern"]
        pkg_dir = find_package_dir(package)
        if pkg_dir is None:
            failures.append(f"package not found: {package}")
            continue
        # Compile the regex.
        name_re = re.compile(pattern)
        found = False
        for rs in pkg_dir.rglob("*.rs"):
            if "/target/" in str(rs):
                continue
            text = rs.read_text(errors="replace")
            # Find tests matching pattern.
            for m in re.finditer(
                r"#\[test\]\s*(?:#\[[^\]]+\]\s*)*fn\s+(\w+)",
                text,
            ):
                name = m.group(1)
                if name_re.search(name):
                    # The artifact must be referenced via include_str! somewhere
                    # in the same file.
                    if (
                        f'include_str!' in text
                        and artifact.split("/")[-1] in text
                    ):
                        found = True
                        break
            if found:
                break
        if not found:
            failures.append(
                f"package {package}: no test matches {pattern} with include_str! of {artifact}"
            )

    if failures:
        print("layer:8-real-artifact-tests status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:8-real-artifact-tests status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
