Summary
- Normalized `packages/rs/release/g3rs-release-filetree-checks` to the current internal package shape and converted the flat runtime files off inline tests onto owned sidecars with shared assertions.
- Kept the cleanup package-local. No rule changes were needed.

Decisions made
- Switched runtime from the local `crates/types` facade to `g3rs-release-types` directly to remove the internal crate-boundary violation instead of widening allowlists.
- Deleted `crates/runtime/src/test_support.rs` entirely rather than moving it, because these flat filetree checks can construct their typed test inputs directly inside the owned sidecars.
- Used flat assertions files for the flat production files (`rs_release_filetree_*` and `run`) so the test-filetree rule sees the exact matching shape.
- Kept the aggregate `run` assertions as a small hand-written proof surface because this lane needs an aggregate result-id proof, not just one rule ID.

Key files for context
- `packages/rs/release/g3rs-release-filetree-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-filetree-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/lib.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/rs_release_filetree_01_license_file.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/assertions/src/common.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/assertions/src/run.rs`

Next steps
- Continue with the next dirty release package root, likely `packages/rs/release/g3rs-release-ingestion`.
- Stop only if the next package exposes a real rule contradiction rather than more old package debt.
