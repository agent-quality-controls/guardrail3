Summary
- Cleaned `packages/rs/cargo/g3rs-cargo-types` to `No findings.` The package now matches the one-crate workspace shape used by the other shared `*-types` crates and its `src/lib.rs` is facade-only.

Decisions made
- Turned the package into a one-crate workspace. Rejected keeping the old bare-crate shape because the active validators work at workspace roots and the other cleaned `*-types` packages already use this shape.
- Marked the crate unpublished with explicit `publish = false`.
- Moved all public type definitions out of `src/lib.rs` into `src/types.rs` so the facade stays declarations-and-reexports only.
- Kept the `module_name_repetitions` allow but documented it with a waiver because shared cargo type names intentionally repeat the family and rule context.

Key files for context
- `packages/rs/cargo/g3rs-cargo-types/Cargo.toml`
- `packages/rs/cargo/g3rs-cargo-types/guardrail3-rs.toml`
- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`

Next steps
- Move to the next package outside cargo.
- Stop only on the next real outdated or contradictory rule.
