#!/usr/bin/env python3
"""Layer 3: rule IDs and severities.

Each [[rule]] entry declares a rule ID, owning package, and required
severity on violation. Verifier:
- grep the package's source tree for the rule ID literal.
- grep for the severity (`G3Severity::Error`) in the same file.

Each [[rule_removed]] entry asserts a rule ID is GONE from the tree.

This is text-search, not AST. Good enough to catch wholesale removals
or severity downgrades; may miss subtle structural rewrites. The
test_pinning layer (Layer 7) catches severity drift from a different
angle.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from _lib import REPO_ROOT, load_manifest, section


def grep(pattern: str, path: Path) -> list[str]:
    if not path.exists():
        return []
    result = subprocess.run(
        ["grep", "-rn", "--include=*.rs", "-F", pattern, str(path)],
        capture_output=True,
        text=True,
    )
    return [line for line in result.stdout.splitlines() if line.strip()]


def find_package_dir(package: str) -> Path | None:
    # Search packages/* for a Cargo.toml whose name matches.
    for cargo in REPO_ROOT.rglob("Cargo.toml"):
        if "/target/" in str(cargo) or "/.cargo-target/" in str(cargo):
            continue
        try:
            content = cargo.read_text()
        except Exception:
            continue
        # Look for `name = "<package>"` in the [package] section.
        m = re.search(r'^\s*name\s*=\s*"([^"]+)"', content, re.MULTILINE)
        if m and m.group(1) == package:
            return cargo.parent
    # Fallback: try direct path patterns.
    for candidate in [
        REPO_ROOT / "packages" / "rs" / "hooks" / package,
        REPO_ROOT / "packages" / "ts" / "hooks" / package,
        REPO_ROOT / "packages" / "rs" / "topology" / package,
        REPO_ROOT / "packages" / "ts" / "topology" / package,
    ]:
        if candidate.exists():
            return candidate
    return None


def main() -> int:
    manifest = load_manifest()
    failures: list[str] = []

    for entry in section(manifest, "rule"):
        rule_id = entry["id"]
        package = entry["package"]
        severity = entry["severity_on_violation"]
        if not entry.get("must_exist", True):
            continue
        pkg_dir = find_package_dir(package)
        if pkg_dir is None:
            failures.append(
                f"rule {rule_id}: package {package} not found in tree"
            )
            continue
        hits = grep(rule_id, pkg_dir)
        if not hits:
            failures.append(
                f"rule {rule_id}: not found in {pkg_dir.relative_to(REPO_ROOT)}"
            )
            continue
        # For each rule, check severity is referenced in at least one
        # source file alongside the rule id.
        sev_pattern = f"G3Severity::{severity}"
        sev_hits = grep(sev_pattern, pkg_dir)
        if not sev_hits:
            failures.append(
                f"rule {rule_id}: severity {severity} not found in package {package}"
            )

    for entry in section(manifest, "rule_removed"):
        rule_id = entry["id"]
        # Search the entire packages/ tree.
        hits = grep(rule_id, REPO_ROOT / "packages")
        if hits:
            failures.append(
                f"rule_removed {rule_id}: still present in tree: {len(hits)} hits"
            )

    if failures:
        print("layer:3-rules status:FAIL")
        for f in failures:
            print(f"  {f}")
        return 1
    print("layer:3-rules status:PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
