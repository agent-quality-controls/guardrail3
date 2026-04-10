# Goal

Remove the retired package-only Rust architecture family from live code, tests, docs, and Rust planning files.

# Approach

1. Replace dead package-owner routing with `arch` in placement and topology.
2. Update tests to prove package roots now key off `arch` enablement.
3. Delete obsolete family-specific Rust plan files.
4. Scrub remaining Rust-side references from docs and plans.
5. Verify with grep and targeted tests.

# Key decisions

- Package-root ownership now belongs to `arch`.
- Obsolete family-specific plan files are deleted instead of preserved.
- Scope is Rust-side references only.

# Files to modify

- apps/guardrail3/crates/app/rs/placement/src/classification.rs
- apps/guardrail3/crates/app/rs/families/topology/...
- apps/guardrail3/crates/app/rs/README.md
- relevant Rust-side .plans files
