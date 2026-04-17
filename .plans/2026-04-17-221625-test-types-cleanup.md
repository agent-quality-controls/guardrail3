Goal
- Bring `packages/rs/test/g3rs-test-types` to the current single-crate workspace shape and eliminate all findings.

Approach
- Normalize the root package:
  - add `publish = false`, `readme`, `include`, docs.rs metadata, features, and a `[workspace]` table
  - replace ad hoc root lints with the standard workspace lint baseline
  - add root policy files and `guardrail3-rs.toml`
- Keep the existing `src/types.rs` inventory and turn `src/lib.rs` into a fully feature-gated facade.
- Keep only the one real manifest-level waiver for `module_name_repetitions`.

Key decisions
- Keep this as a single-crate types package rather than introducing internal crates.
- Keep the `module_name_repetitions` allowance only with a documented waiver because the family-prefixed test type names are intentional.

Files to modify
- `packages/rs/test/g3rs-test-types/Cargo.toml`
- `packages/rs/test/g3rs-test-types/src/lib.rs`
- `packages/rs/test/g3rs-test-types/README.md`
- new root policy files
- new `guardrail3-rs.toml`
