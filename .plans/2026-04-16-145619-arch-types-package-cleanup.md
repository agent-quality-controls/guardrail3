Goal
- Clean `packages/rs/arch/g3rs-arch-types` until `validate` returns `No findings.`
- Normalize it to the current single-crate `*-types` workspace shape.

Approach
- Convert the root manifest into the current shared types shape:
  - explicit `publish = false`
  - feature contract
  - local workspace block and workspace lints
  - `[lints] workspace = true`
- Add the standard root policy files and `guardrail3-rs.toml`
- Make `src/lib.rs` a small gated facade over `src/types.rs`
- Keep the documented waiver for `module_name_repetitions`
- Re-run the validator and package tests if any, then write a worklog and commit

Key decisions
- Keep this as a single-crate workspace because it already is the shared family transport boundary.
- Keep the family-name repetition waiver because these shared type names intentionally carry the arch family context.

Files to modify
- `.plans/2026-04-16-145619-arch-types-package-cleanup.md`
- `packages/rs/arch/g3rs-arch-types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-types/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-types/rust-toolchain.toml`
- `packages/rs/arch/g3rs-arch-types/rustfmt.toml`
- `packages/rs/arch/g3rs-arch-types/clippy.toml`
- `packages/rs/arch/g3rs-arch-types/deny.toml`
- `packages/rs/arch/g3rs-arch-types/src/lib.rs`
