#!/usr/bin/env python3
"""Layer 7: test pinning.

Each [[test_pinning]] entry says: every test function in the named
package whose name matches `applies_to_tests_matching` must call the
helper named in `test_helper_required`.

Approach: parse Rust source files in the package's tests directories;
find `#[test]`-annotated `fn name_*` definitions whose name matches
the pattern; require the body to contain the helper name.
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


def collect_tests(pkg_dir: Path) -> list[tuple[str, str, Path]]:
    """Returns list of (test_name, test_body, file_path)."""
    out: list[tuple[str, str, Path]] = []
    test_re = re.compile(
        r"#\[test\]\s*(?:#\[[^\]]+\]\s*)*fn\s+(\w+)\s*\([^)]*\)\s*\{",
        re.MULTILINE,
    )
    for rs in pkg_dir.rglob("*.rs"):
        if "/target/" in str(rs):
            continue
        text = rs.read_text(errors="replace")
        for m in test_re.finditer(text):
            name = m.group(1)
            start = m.end()
            depth = 1
            i = start
            while i < len(text) and depth > 0:
                ch = text[i]
                if ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                i += 1
            body = text[start:i]
            out.append((name, body, rs))
    return out


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "test_pinning"):
        package = entry["package"]
        helpers = entry.get("test_helpers_accepted") or [
            entry["test_helper_required"]
        ]
        pat = entry["applies_to_tests_matching"]
        pkg_dir = find_package_dir(package)
        if pkg_dir is None:
            failures.append(f"package not found: {package}")
            continue
        tests = collect_tests(pkg_dir)
        for name, body, file in tests:
            if pat in name and not any(h in body for h in helpers):
                rel = file.relative_to(REPO_ROOT)
                failures.append(
                    f"{rel}::{name} matches pattern {pat!r} but calls none of {helpers}"
                )

    if failures:
        print("layer:7-test-pinning status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:7-test-pinning status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
