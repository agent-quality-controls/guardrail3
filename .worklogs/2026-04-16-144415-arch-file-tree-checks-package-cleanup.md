Summary
- Cleaned `packages/rs/arch/g3rs-arch-file-tree-checks` until `validate` returned `No findings.` and the package workspace tests passed.
- Removed the fake local `types` crate, rewrote the rule tests into owned sidecar directories with shared assertions files, and updated `arch-ingestion` to stop depending on the deleted path.

Decisions made
- Deleted `crates/types` because it only re-exported `g3rs-arch-types` and created fake arch and apparch coupling.
- Switched the root facade and runtime crate to use `g3rs-arch-types` directly, then updated `packages/rs/arch/g3rs-arch-ingestion/crates/types` so the repo does not keep a broken path dependency.
- Replaced the shared public test helper bag with per-rule assertions modules so the assertions crate matches the owned sidecar contract and no longer trips the public-field rule.
- Removed `test_support.rs` and moved helper setup into each rule's own `helpers.rs` so the sidecars only touch their owned production rule and the shared assertions crate.
- Kept the flat rule-file layout and used `x_tests/mod.rs` sidecars with `#[path = "..."]` plus same-line reasons, because this package already follows the chosen file-module sidecar pattern.

Key files for context
- `packages/rs/arch/g3rs-arch-file-tree-checks/Cargo.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-file-tree-checks/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_01_crate_has_facade.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_03_mod_rs_required_tests/cases.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split_tests/helpers.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/lib.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/rs_arch_01_crate_has_facade.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/assertions/src/rs_arch_07a_structural_split.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/src/lib.rs`

Next steps
- Continue to the next remaining Rust package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for arch file-tree-check workspaces with flat rule files, `x_tests` sidecars, and per-rule shared assertions modules.
