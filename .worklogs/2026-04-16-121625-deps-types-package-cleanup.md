Summary
- Cleaned `packages/rs/deps/g3rs-deps-types` into the standard shared `*-types` workspace shape.
- Added the missing root policy files, explicit publish intent, a gated facade, and the documented manifest waiver for repeated shared type names.

Decisions made
- Treated this as ordinary old root-shape debt because it matched the same migration pattern already used on the other shared types crates.
- Kept the public data structs plain and public because this crate is the shared transport boundary for the deps family.

Key files for context
- `packages/rs/deps/g3rs-deps-types/Cargo.toml`
- `packages/rs/deps/g3rs-deps-types/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-types/src/lib.rs`
- `packages/rs/deps/g3rs-deps-types/src/types.rs`

Next steps
- Commit this package cleanup as its own slice.
- Continue to `packages/rs/deps/g3rs-deps-ingestion` and stop only if a rule is clearly wrong or contradictory.
