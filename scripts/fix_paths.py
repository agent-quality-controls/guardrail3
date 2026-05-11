#!/usr/bin/env python3
"""Rewrite all path dependencies after package directory reorganization.

Since packages already moved, old relative paths are now broken.
We extract the package directory name from the path and look up its new location.
"""

import os
import re

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Mapping: package directory name → new location (relative to repo root)
NEW_LOCATIONS = {
    "cargo-toml-parser": "packages/parsers/cargo-toml-parser",
    "cargo-config-toml-parser": "packages/parsers/cargo-config-toml-parser",
    "cliff-toml-parser": "packages/parsers/cliff-toml-parser",
    "clippy-toml-parser": "packages/parsers/clippy-toml-parser",
    "deny-toml-parser": "packages/parsers/deny-toml-parser",
    "mutants-toml-parser": "packages/parsers/mutants-toml-parser",
    "nextest-toml-parser": "packages/parsers/nextest-toml-parser",
    "release-plz-toml-parser": "packages/parsers/release-plz-toml-parser",
    "rust-toolchain-toml-parser": "packages/parsers/rust-toolchain-toml-parser",
    "rustfmt-toml-parser": "packages/parsers/rustfmt-toml-parser",
    "g3rs-toml-parser": "packages/parsers/g3rs-toml-parser",
    "guardrail3-check-types": "packages/shared/guardrail3-check-types",
    "reason-policy": "packages/shared/reason-policy",
    "g3rs-workspace-crawl": "packages/rs/g3rs-workspace-crawl",
    "g3rs-cargo-config-checks": "packages/rs/cargo/g3rs-cargo-config-checks",
    "g3rs-cargo-ingestion": "packages/rs/cargo/g3rs-cargo-ingestion",
    "g3rs-clippy-config-checks": "packages/rs/clippy/g3rs-clippy-config-checks",
    "g3rs-clippy-ingestion": "packages/rs/clippy/g3rs-clippy-ingestion",
    "g3rs-deny-config-checks": "packages/rs/deny/g3rs-deny-config-checks",
    "g3rs-deny-ingestion": "packages/rs/deny/g3rs-deny-ingestion",
    "g3rs-deps-config-checks": "packages/rs/deps/g3rs-deps-config-checks",
    "g3rs-fmt-config-checks": "packages/rs/fmt/g3rs-fmt-config-checks",
    "g3rs-fmt-ingestion": "packages/rs/fmt/g3rs-fmt-ingestion",
    "g3rs-garde-ast-checks": "packages/rs/garde/g3rs-garde-ast-checks",
    "g3rs-garde-config-checks": "packages/rs/garde/g3rs-garde-config-checks",
    "g3rs-garde-ingestion": "packages/rs/garde/g3rs-garde-ingestion",
    "g3rs-release-config-checks": "packages/rs/release/g3rs-release-config-checks",
    "g3rs-release-ingestion": "packages/rs/release/g3rs-release-ingestion",
    "g3rs-toolchain-config-checks": "packages/rs/toolchain/g3rs-toolchain-config-checks",
    "g3rs-toolchain-ingestion": "packages/rs/toolchain/g3rs-toolchain-ingestion",
}

def find_cargo_tomls():
    result = []
    for root, dirs, files in os.walk(REPO_ROOT):
        dirs[:] = [d for d in dirs if d != "target" and d != ".git"]
        if "Cargo.toml" in files:
            result.append(os.path.join(root, "Cargo.toml"))
    return result

def extract_package_target(rel_path):
    """Extract the target package name and any suffix from a relative path.

    E.g. "../../../cargo-toml-parser" → ("cargo-toml-parser", "")
    E.g. "../../../guardrail3-check-types" → ("guardrail3-check-types", "/crates/guardrail3-check-types")
    E.g. "../../../reason-policy" → ("reason-policy", "/crates/reason-policy")
    """
    # Normalize
    parts = [p for p in rel_path.split("/") if p != "." and p != ""]
    # Remove all ".." prefixes
    non_dotdot = []
    for p in parts:
        if p == "..":
            continue
        non_dotdot.append(p)

    if not non_dotdot:
        return None, None

    pkg_name = non_dotdot[0]
    suffix = "/" + "/".join(non_dotdot[1:]) if len(non_dotdot) > 1 else ""
    return pkg_name, suffix

def main():
    changes = 0
    for toml_path in find_cargo_tomls():
        toml_dir = os.path.dirname(toml_path)
        toml_rel = os.path.relpath(toml_path, REPO_ROOT)

        with open(toml_path, "r") as f:
            content = f.read()

        new_content = content

        for match in re.finditer(r'path\s*=\s*"([^"]*)"', content):
            old_rel = match.group(1)

            # Skip internal deps
            if not old_rel.startswith(".."):
                continue
            # Skip within-package deps (one level up)
            parts_up = len([p for p in old_rel.split("/") if p == ".."])
            if parts_up <= 1:
                continue

            pkg_name, suffix = extract_package_target(old_rel)
            if not pkg_name or pkg_name not in NEW_LOCATIONS:
                continue

            new_abs = os.path.join(REPO_ROOT, NEW_LOCATIONS[pkg_name] + suffix)
            new_rel = os.path.relpath(new_abs, toml_dir)

            if old_rel != new_rel:
                # Replace only this specific occurrence
                old_pattern = f'path = "{old_rel}"'
                new_pattern = f'path = "{new_rel}"'
                if old_pattern in new_content:
                    new_content = new_content.replace(old_pattern, new_pattern, 1)
                    print(f"  {toml_rel}: {old_rel} → {new_rel}")
                    changes += 1

        if new_content != content:
            with open(toml_path, "w") as f:
                f.write(new_content)

    print(f"\n=== {changes} path references rewritten ===")

if __name__ == "__main__":
    main()
