Goal
- Bring `packages/rs/hooks/g3rs-hooks-types` to the current single-crate workspace shape and eliminate all findings.

Approach
- Normalize the root package:
  - add `publish = false`, `readme`, `include`, docs.rs metadata, features, and a `[workspace]` table
  - replace ad hoc root lints with the standard workspace lint baseline
  - add root policy files and `guardrail3-rs.toml`
- Reshape the source layout:
  - move the public type inventory into `src/types.rs`
  - keep `src/lib.rs` as a small feature-gated facade
- Keep only the one real manifest-level waiver for `module_name_repetitions`.

Key decisions
- Keep this as a single-crate types package rather than introducing internal crates.
- Keep the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed hook type names are intentional.

Files to modify
- `packages/rs/hooks/g3rs-hooks-types/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/types.rs`
- `packages/rs/hooks/g3rs-hooks-types/README.md`
- new root policy files
- new `guardrail3-rs.toml`
