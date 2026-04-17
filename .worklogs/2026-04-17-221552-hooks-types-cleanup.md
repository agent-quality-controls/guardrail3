Summary
- Cleaned `packages/rs/hooks/g3rs-hooks-types` to the current single-crate workspace shape and brought it to `No findings.`
- Moved the public hook-family type inventory into `src/types.rs`, leaving a small feature-gated facade in `src/lib.rs`.

Decisions made
- Kept this as a single-crate types package rather than introducing internal crates.
- Replaced the ad hoc root lint tables with the standard workspace lint baseline instead of patching missing lints individually.
- Kept the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed hook type names are intentional.

Key files for context
- `packages/rs/hooks/g3rs-hooks-types/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-types/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/types.rs`
- `packages/rs/hooks/g3rs-hooks-types/README.md`

Next steps
- Continue with the remaining dirty package roots in the `test` family, then `topology`.
- Leave the parser warning-only packages for the end unless the user wants those warnings waived away too.
