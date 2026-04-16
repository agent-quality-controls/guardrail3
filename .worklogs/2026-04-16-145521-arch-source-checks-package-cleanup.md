Summary
- Cleaned `packages/rs/arch/g3rs-arch-source-checks` until `validate` returned `No findings.` and the package workspace tests passed.
- Removed the fake local `types` crate, deleted the dead empty `rs_code_ast_*` runtime directories, rewrote the two real test sidecars, and updated `arch-ingestion` to stop depending on the deleted path.

Decisions made
- Deleted `crates/types` because it only re-exported `g3rs-arch-types` and created fake arch and apparch coupling.
- Switched the root facade and runtime crate to use `g3rs-arch-types` directly, then updated `packages/rs/arch/g3rs-arch-ingestion/crates/types` so the repo does not keep a broken path dependency.
- Deleted the empty `rs_code_ast_*` directories because they were dead junk copied into the runtime tree and were the whole reason the structural split rule fired.
- Replaced the shared public test helper bag with per-rule assertions modules so the assertions crate matches the owned sidecar contract and no longer trips the public-field rule.
- Removed `test_support.rs` and moved helper setup into each rule's own `helpers.rs` so the sidecars only touch their owned production rule and the shared assertions crate.

Key files for context
- `packages/rs/arch/g3rs-arch-source-checks/Cargo.toml`
- `packages/rs/arch/g3rs-arch-source-checks/guardrail3-rs.toml`
- `packages/rs/arch/g3rs-arch-source-checks/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/run.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports_tests/cases.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_09_no_path_attr_tests/helpers.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/lib.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/rs_arch_08a_feature_gated_exports.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/rs_arch_09_no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/Cargo.toml`
- `packages/rs/arch/g3rs-arch-ingestion/crates/types/src/lib.rs`

Next steps
- Continue to the next remaining Rust package and stop only on the next real rule bug or contradiction.
- Reuse this package as the reference shape for arch source-check workspaces with flat rule files, `x_tests` sidecars, and per-rule shared assertions modules.
