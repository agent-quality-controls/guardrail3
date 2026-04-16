Summary
- Cleaned `packages/rs/apparch/g3rs-apparch-types` until `validate` returned `No findings.` and the package workspace tests passed.
- Converted it to the current single-crate `*-types` workspace shape with root policy files, explicit publish intent, and a facade-only `src/lib.rs`.

Decisions made
- Kept this as a single shared crate because it already is the family transport boundary and does not need subcrates.
- Moved all real type definitions into `src/types.rs` so `src/lib.rs` is a pure facade.
- Added the documented `module_name_repetitions` waiver because the long shared type names are intentional family context, not slack.

Key files for context
- `packages/rs/apparch/g3rs-apparch-types/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-types/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-types/src/types.rs`

Next steps
- Continue to the next remaining Rust package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for other single-crate shared `*-types` workspaces.
