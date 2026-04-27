Summary

The recursive `g3rs-arch/structural-split` logic was already correct in source. The miss on `packages/ts/eslint/g3ts-eslint-config-checks` came from using a stale installed `g3rs` binary from before the arch fixes. I added a mixed root-package workspace regression to arch ingestion and reinstalled `g3rs` from current source so the installed CLI now matches the repo state.

Decisions made

- Kept the new regression test.
  - It proves the exact package shape that caused confusion: a root package that is also a workspace, with a member runtime crate containing a dense nested module folder.
  - This is worth keeping even though the source logic did not need another code change.
- Did not change arch logic again.
  - The current source already reports `g3rs-arch/structural-split` on the TS ESLint runtime crate.
  - The actual bug was environmental drift between installed CLI and current source.
- Reinstalled the `g3rs` binary from the current repo path.
  - Reason: the user explicitly uses the installed command, so source-only correctness was not enough.

Key files for context

- `.plans/2026-04-20-173954-fix-arch-structural-split-real-package-gap.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/structure.rs`
- `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs`

Verification

- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml -p g3rs-arch-ingestion-runtime file_tree::file_tree_tests::pipeline::file_tree_pipeline_reports_dense_nested_member_module_in_mixed_root_workspace -- --exact`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/rs/arch/g3rs-arch-file-tree-checks/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-config-checks --family arch --inventory`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --force`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks --family arch --inventory`

Next steps

- Stop treating the installed `g3rs` as authoritative after source changes unless it has just been rebuilt or reinstalled.
- If the user wants the TS ESLint package clean again, the next work is to restructure `crates/runtime/src/full_config` rather than touching arch.
