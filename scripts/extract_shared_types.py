#!/usr/bin/env python3
"""
Extract shared family types from checks/types into standalone g3rs-{family}-types packages.

For each family:
1. Create packages/rs/{family}/g3rs-{family}-types/ with the input struct
2. Make checks/types re-export from the new package
3. Update ingestion to depend on g3rs-{family}-types instead of checks facade
"""

import os
import re
import shutil

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Family configs: (family_dir, checks_pkg_name, types_pkg_name, ingestion_pkg_name_or_None)
FAMILIES = [
    ("cargo", "g3rs-cargo-config-checks", "g3rs-cargo-types", "g3rs-cargo-config-ingestion"),
    ("clippy", "g3rs-clippy-config-checks", "g3rs-clippy-types", "g3rs-clippy-config-ingestion"),
    ("deny", "g3rs-deny-config-checks", "g3rs-deny-types", "g3rs-deny-config-ingestion"),
    ("deps", "g3rs-deps-config-checks", "g3rs-deps-types", None),  # no ingestion yet
    ("fmt", "g3rs-fmt-config-checks", "g3rs-fmt-types", "g3rs-fmt-config-ingestion"),
    ("garde", "g3rs-garde-config-checks", "g3rs-garde-types", "g3rs-garde-config-ingestion"),
    ("release", "g3rs-release-config-checks", "g3rs-release-types", "g3rs-release-config-ingestion"),
    ("toolchain", "g3rs-toolchain-config-checks", "g3rs-toolchain-types", "g3rs-toolchain-config-ingestion"),
]

WORKSPACE_LINTS = '''[workspace.lints.rust]
warnings = "deny"
unsafe_code = "forbid"
dead_code = "deny"
unused_results = "deny"
unused_crate_dependencies = "deny"
missing_debug_implementations = "deny"
unreachable_pub = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
unimplemented = "deny"
todo = "deny"
dbg_macro = "deny"
print_stdout = "deny"
print_stderr = "deny"
disallowed_methods = "deny"
disallowed_macros = "deny"
disallowed_types = "deny"
indexing_slicing = "deny"
string_slice = "deny"
arithmetic_side_effects = "deny"
shadow_unrelated = "deny"
missing_assert_message = "deny"
partial_pub_fields = "deny"
str_to_string = "deny"
implicit_clone = "deny"
as_conversions = "deny"
float_cmp = "deny"
lossy_float_literal = "deny"
wildcard_enum_match_arm = "deny"
rest_pat_in_fully_bound_structs = "deny"
large_stack_arrays = "deny"
needless_pass_by_value = "deny"
redundant_else = "deny"
large_futures = "deny"
semicolon_if_nothing_returned = "deny"
redundant_closure_for_method_calls = "deny"
map_unwrap_or = "deny"
verbose_file_reads = "deny"
missing_docs_in_private_items = "deny"
module_name_repetitions = "deny"
must_use_candidate = "deny"
option_if_let_else = "deny"
empty_line_after_doc_comments = "deny"
single_match_else = "deny"
ref_option_ref = "deny"
trivially_copy_pass_by_ref = "deny"
multiple_crate_versions = "deny"
redundant_pub_crate = "allow"
'''


def extract_family(family_dir, checks_pkg, types_pkg, ingestion_pkg):
    """Extract shared types for one family."""
    family_path = os.path.join(REPO, "packages", "rs", family_dir)
    checks_path = os.path.join(family_path, checks_pkg)
    types_path = os.path.join(family_path, types_pkg)

    # Read current checks types
    checks_types_dir = os.path.join(checks_path, "crates", "types")
    checks_types_cargo = os.path.join(checks_types_dir, "Cargo.toml")

    if not os.path.exists(checks_types_cargo):
        print(f"  SKIP: {checks_types_cargo} not found")
        return

    with open(checks_types_cargo) as f:
        checks_types_cargo_content = f.read()

    # Read the source files
    types_src_dir = os.path.join(checks_types_dir, "src")
    src_files = {}
    for fname in os.listdir(types_src_dir):
        if fname.endswith(".rs"):
            with open(os.path.join(types_src_dir, fname)) as f:
                src_files[fname] = f.read()

    # Extract dependency lines from checks types Cargo.toml
    dep_lines = []
    in_deps = False
    for line in checks_types_cargo_content.splitlines():
        if line.strip() == "[dependencies]":
            in_deps = True
            continue
        if line.strip().startswith("[") and in_deps:
            break
        if in_deps and line.strip():
            dep_lines.append(line)

    # Compute the crate name for the types package
    types_crate_name = types_pkg

    # Create the new types package
    os.makedirs(os.path.join(types_path, "src"), exist_ok=True)

    # Build Cargo.toml — standalone package (no workspace, no subcrates)
    deps_section = "\n".join(dep_lines) if dep_lines else ""
    cargo_toml = f'''[package]
name = "{types_crate_name}"
version = "0.1.0"
edition = "2024"
description = "Shared types for g3rs {family_dir} family"
license = "MIT OR Apache-2.0"
rust-version = "1.85"
repository = "https://github.com/ArcticLens/websmasher"
keywords = ["guardrail3", "{family_dir}", "types", "rust"]
categories = ["development-tools"]

[package.metadata.guardrail3]
shared = true

[lints.rust]
warnings = "deny"
unsafe_code = "forbid"
dead_code = "deny"
unused_results = "deny"
unused_crate_dependencies = "deny"
missing_debug_implementations = "deny"
unreachable_pub = "deny"

[lints.clippy]
all = {{ level = "deny", priority = -1 }}
pedantic = {{ level = "deny", priority = -1 }}
cargo = {{ level = "deny", priority = -1 }}
nursery = {{ level = "deny", priority = -1 }}
redundant_pub_crate = "allow"
module_name_repetitions = "allow"

[dependencies]
{deps_section}
'''

    with open(os.path.join(types_path, "Cargo.toml"), "w") as f:
        f.write(cargo_toml)

    # Copy source files
    for fname, content in src_files.items():
        with open(os.path.join(types_path, "src", fname), "w") as f:
            f.write(content)

    print(f"  Created {types_pkg} with {len(src_files)} source files")

    # Now update checks types to re-export from the new package
    # The checks types Cargo.toml gets a dep on the new types package
    # and its lib.rs becomes a re-export facade

    # Figure out what the checks types lib.rs currently exports
    lib_rs = src_files.get("lib.rs", "")
    # Find all pub use / pub struct names
    exports = re.findall(r'pub (?:use \w+::)?(\w+)', lib_rs)

    # Rewrite checks types Cargo.toml to depend on new types package
    # Remove old parser deps, add new types dep
    new_checks_cargo = []
    in_deps_section = False
    deps_written = False
    for line in checks_types_cargo_content.splitlines():
        if line.strip() == "[dependencies]":
            in_deps_section = True
            new_checks_cargo.append(line)
            # Add dependency on new types package
            new_checks_cargo.append(f'{types_crate_name} = {{ path = "../../../{types_pkg}" }}')
            deps_written = True
            continue
        if in_deps_section:
            if line.strip().startswith("["):
                in_deps_section = False
                new_checks_cargo.append(line)
            # Skip old dep lines
            continue
        new_checks_cargo.append(line)

    if not deps_written:
        new_checks_cargo.append("")
        new_checks_cargo.append("[dependencies]")
        new_checks_cargo.append(f'{types_crate_name} = {{ path = "../../../{types_pkg}" }}')

    with open(checks_types_cargo, "w") as f:
        f.write("\n".join(new_checks_cargo) + "\n")

    # Rewrite checks types lib.rs to re-export from new package
    types_crate_ident = types_crate_name.replace("-", "_")
    checks_types_crate_name = checks_pkg.replace("-", "_") + "_types"

    # Find all public items to re-export
    public_items = []
    for fname, content in src_files.items():
        for m in re.finditer(r'pub (?:struct|enum|type) (\w+)', content):
            public_items.append(m.group(1))

    if public_items:
        reexport = f"pub use {types_crate_ident}::{{{', '.join(public_items)}}};\n"
    else:
        reexport = f"pub use {types_crate_ident}::*;\n"

    with open(os.path.join(types_src_dir, "lib.rs"), "w") as f:
        f.write(reexport)

    # Remove non-lib.rs source files from checks types (they're now in the new package)
    for fname in src_files:
        if fname != "lib.rs":
            fpath = os.path.join(types_src_dir, fname)
            if os.path.exists(fpath):
                os.remove(fpath)
                print(f"  Removed {checks_pkg}/crates/types/src/{fname}")

    print(f"  Updated {checks_pkg}/crates/types/ to re-export from {types_pkg}")

    # Update ingestion to depend on new types package instead of checks facade
    if ingestion_pkg:
        ingestion_path = os.path.join(family_path, ingestion_pkg)
        ingestion_runtime_cargo = os.path.join(ingestion_path, "crates", "runtime", "Cargo.toml")

        if os.path.exists(ingestion_runtime_cargo):
            with open(ingestion_runtime_cargo) as f:
                content = f.read()

            # Replace checks dep with types dep
            checks_dep_pattern = re.escape(checks_pkg)
            # Find the line with the checks dep
            new_content = content
            for line in content.splitlines():
                if checks_pkg in line and "path" in line:
                    # Replace with types package dep
                    new_line = f'{types_crate_name} = {{ path = "../../../{types_pkg}", version = "0.1.0" }}'
                    new_content = new_content.replace(line, new_line)
                    break

            with open(ingestion_runtime_cargo, "w") as f:
                f.write(new_content)

            # Update import in ingest.rs and run.rs
            checks_crate_ident = checks_pkg.replace("-", "_")
            for src_file in ["ingest.rs", "run.rs"]:
                src_path = os.path.join(ingestion_path, "crates", "runtime", "src", src_file)
                if os.path.exists(src_path):
                    with open(src_path) as f:
                        src = f.read()
                    new_src = src.replace(f"use {checks_crate_ident}::", f"use {types_crate_ident}::")
                    if new_src != src:
                        with open(src_path, "w") as f:
                            f.write(new_src)
                        print(f"  Updated {ingestion_pkg}/{src_file} imports")

            print(f"  Updated {ingestion_pkg} to depend on {types_pkg}")

    # Update checks runtime to also depend on new types (it may import from types crate)
    # Actually the checks runtime already depends on checks-types which re-exports, so no change needed.


def main():
    for family_dir, checks_pkg, types_pkg, ingestion_pkg in FAMILIES:
        print(f"\n=== {family_dir} ===")
        extract_family(family_dir, checks_pkg, types_pkg, ingestion_pkg)

    # Handle garde specially — garde-ast-checks also has types
    print("\n=== garde-ast (additional) ===")
    garde_path = os.path.join(REPO, "packages", "rs", "garde")
    ast_types_dir = os.path.join(garde_path, "g3rs-garde-ast-checks", "crates", "types")
    # garde-ast types stay in place — they're different types (G3RsAstFile, G3RsGardeAstChecksInput)
    # not shared with garde-config. No extraction needed.
    print("  SKIP: garde-ast types are specific to ast-checks, not shared")


if __name__ == "__main__":
    main()
