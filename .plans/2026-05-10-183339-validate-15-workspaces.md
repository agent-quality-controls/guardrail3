# Plan: Fix `g3rs validate` errors across 15 workspaces

## Goal

Make `apps/guardrail3-rs/target/release/g3rs validate --path <ws>` exit 0 for each of the 15 workspaces in scope. Then `scripts/verify/all.sh` must pass all 8 layers.

## Workspaces (with current error counts)

- 304 packages/rs/deny/g3rs-deny-config-checks
- 145 packages/rs/cargo/g3rs-cargo-config-checks
- 100 packages/rs/apparch/g3rs-apparch-ingestion
- 44 packages/rs/cargo/g3rs-cargo-ingestion
- 43 packages/rs/clippy/g3rs-clippy-ingestion
- 19 packages/rs/arch/g3rs-arch-config-checks
- 17 packages/rs/arch/g3rs-arch-source-checks
- 16 packages/rs/arch/g3rs-arch-file-tree-checks
- 5 packages/rs/apparch/g3rs-apparch-hook-contract
- 5 packages/rs/arch/g3rs-arch-types
- 5 packages/rs/cargo/g3rs-cargo-hook-contract
- 5 packages/rs/code/g3rs-code-hook-contract
- 5 packages/rs/clippy/g3rs-clippy-hook-contract
- 4 packages/parsers/astro-config-parser
- 4 packages/parsers/clippy-toml-parser

Total: ~721 errors

## Approach

For each workspace (ascending by error count):
1. Run `cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged --manifest-path <ws>/Cargo.toml`
2. Run `cargo fmt --manifest-path <ws>/Cargo.toml --all`
3. Read `apps/guardrail3-rs/target/release/g3rs validate --path <ws>` errors
4. Fix manually (no `#[allow]`/`#[expect]` without specific reason)
5. Re-validate; loop until exit 0

## Common fix patterns

- missing_docs_in_private_items: add `///` per item
- missing_errors_doc: add `# Errors` section
- missing_panics_doc: add `# Panics`
- arithmetic_side_effects: use saturating/checked
- indexing_slicing: use `.get()`
- excessive_nesting: extract helpers
- must_use_candidate: add `#[must_use]`
- missing_const_for_fn: add `const`
- wildcard_enum_match_arm: enumerate variants
- unnecessary_wraps: remove `Result`/`Option`
- large_enum_variant: Box the large variant
- too_many_lines: split into smaller functions
- too_many_arguments: collapse to struct
- module_name_repetitions: rename type
- case_sensitive_file_extension_comparisons: `eq_ignore_ascii_case`
- expect_used: `unwrap_or_else(|| panic!(...))`
- disallowed_methods: route through `crate::fs` port
- derive_partial_eq_without_eq: add `Eq` derive
