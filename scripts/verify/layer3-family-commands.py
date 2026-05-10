#!/usr/bin/env python3
"""Layer 3b: family hook contract command set.

Each [[family_command]] entry says that a family's `hook_contract()`
function must reference a specific command. This is the single source
of truth for what `g3rs validate --staged` executes.

Verifier: for each family-package, grep its source for the literal
command string. If the command is missing, the family contract has
drifted away from what the manifest declares.
"""

from __future__ import annotations

import re
import subprocess
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


def grep_for_substrings(needle: str, root: Path) -> int:
    if not root.exists():
        return 0
    result = subprocess.run(
        ["grep", "-rn", "--include=*.rs", "-F", needle, str(root)],
        capture_output=True,
        text=True,
    )
    return len(
        [line for line in result.stdout.splitlines() if line.strip()]
    )


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "family_command"):
        package = entry["package"]
        cmd = entry["command"]
        pkg_dir = find_package_dir(package)
        if pkg_dir is None:
            failures.append(f"family package not found: {package}")
            continue
        # Look for the full command as a literal substring.
        hits = grep_for_substrings(cmd, pkg_dir)
        if hits == 0:
            failures.append(
                f"family {entry['family']} ({package}): command literal absent: {cmd!r}"
            )

    if failures:
        print("layer:3b-family-commands status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:3b-family-commands status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
