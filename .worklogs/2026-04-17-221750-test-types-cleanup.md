Summary
- Cleaned `packages/rs/test/g3rs-test-types` to the current single-crate workspace shape and brought it to `No findings.`
- Kept the existing type inventory in place and turned `src/lib.rs` into a fully feature-gated facade.

Decisions made
- Kept this as a single-crate types package rather than introducing internal crates.
- Replaced the ad hoc root lint tables with the standard workspace lint baseline instead of patching missing lints individually.
- Kept the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed test type names are intentional.

Key files for context
- `packages/rs/test/g3rs-test-types/Cargo.toml`
- `packages/rs/test/g3rs-test-types/guardrail3-rs.toml`
- `packages/rs/test/g3rs-test-types/src/lib.rs`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-types/README.md`

Next steps
- Continue with the remaining dirty `test` family roots: `g3rs-test-config-checks`, `g3rs-test-file-tree-checks`, and `g3rs-test-ingestion`.
- After the `test` family is clean, move on to the `topology` roots.
