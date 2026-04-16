Summary
- Cleaned `packages/rs/toolchain/g3rs-toolchain-types` to the current shared `*-types` package shape.
- Added the missing workspace policy files, made publish intent explicit, and split the crate into a small facade plus `types.rs`.

Decisions made
- Kept the shared transport structs with public fields because this crate is the shared contract between toolchain packages.
- Added the `module_name_repetitions` waiver because the shared family-prefixed type names are intentional here.
- Used the same root shape as the other clean `*-types` packages so release, cargo, and deps checks all read this package consistently.

Key files for context
- `packages/rs/toolchain/g3rs-toolchain-types/Cargo.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/guardrail3-rs.toml`
- `packages/rs/toolchain/g3rs-toolchain-types/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-types/src/types.rs`

Next steps
- Commit this package cleanup.
- Move to the next package family and continue until the next real rule contradiction shows up.
