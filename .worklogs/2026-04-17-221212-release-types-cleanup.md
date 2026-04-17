Summary
- Cleaned `packages/rs/release/g3rs-release-types` to the current single-crate workspace shape and brought it to `No findings.`
- Moved the release-family type inventory out of `src/lib.rs` into `src/types.rs`, leaving a small feature-gated facade at the root.

Decisions made
- Kept this as a single-crate types package rather than inventing internal crates. It matches the cleaned shared and family types packages already in the repo.
- Replaced the ad hoc root lint tables with the standard workspace lint baseline instead of patching individual missing lints one by one.
- Kept the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed release type names are intentional.
- Added exact waivers for the two large release inventory structs instead of distorting the normalized release facts to satisfy the field-count threshold.

Key files for context
- `packages/rs/release/g3rs-release-types/Cargo.toml`
- `packages/rs/release/g3rs-release-types/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-types/README.md`

Next steps
- Re-run the full package-root validate sweep to refresh the remaining dirty package list after the release family cleanup.
- Continue with the next non-clean package root from that fresh sweep.
