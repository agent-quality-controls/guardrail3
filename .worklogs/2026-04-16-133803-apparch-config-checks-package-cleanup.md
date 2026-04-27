Summary
- Cleaned `packages/rs/apparch/g3rs-apparch-config-checks` until `validate` returned `No findings.` and the package workspace tests passed.
- Removed the fake local `types` crate, restored the working `x_tests/mod.rs` sidecar wiring, and moved final proof into the new shared assertions crate.

Decisions made
- Kept the chosen file-module sidecar contract as `#[path = "x_tests/mod.rs"] mod x_tests;` because plain Rust resolution cannot reach sibling `x_tests/` directories without the path redirect.
- Deleted `crates/types` and used `g3rs-apparch-types` directly because the local crate was only a wrapper and created fake boundary noise.
- Added `g3rs-arch/structural-split` waivers for runtime and assertions because both crates intentionally keep one apparch rule or proof module per file.
- Removed the copied `module_name_repetitions = "allow"` because the clean sibling package does not need it and the warning was package slack, not a rule problem.

Key files for context
- `packages/rs/apparch/g3rs-apparch-config-checks/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/guardrail3-rs.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/run.rs`

Next steps
- Continue package cleanup from the next Rust package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for apparch config workspaces with flat rule files, `x_tests` sidecars, and flat shared assertions modules.
