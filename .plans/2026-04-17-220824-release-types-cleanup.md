Goal
- Bring `packages/rs/release/g3rs-release-types` to the current single-crate workspace shape and eliminate all findings.

Approach
- Normalize the root package:
  - add `publish = false`, `readme`, `include`, docs.rs metadata, features, and a `[workspace]` table
  - replace ad hoc root lints with the standard workspace lint baseline
  - add root policy files and `guardrail3-rs.toml`
- Reshape the source layout:
  - move the large public type inventory out of `src/lib.rs` into `src/types.rs`
  - keep `src/lib.rs` as a small feature-gated facade
- Add narrow waivers for the two intentionally large release inventory structs.

Key decisions
- Keep this as a single-crate types package rather than inventing internal crates. It matches the cleaned shared and family types packages.
- Keep the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed type names are intentional here.
- Waive the large inventory structs exactly instead of distorting the release facts into less truthful shapes.

Files to modify
- `packages/rs/release/g3rs-release-types/Cargo.toml`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-types/README.md`
- new root policy files
- new `guardrail3-rs.toml`
