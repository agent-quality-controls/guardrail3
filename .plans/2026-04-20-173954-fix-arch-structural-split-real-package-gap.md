Goal

Make `RS-ARCH-FILETREE-07` fire on the real package shape it currently misses: a package root that is both a Rust crate and a small workspace, with a member runtime crate containing an oversized nested module folder.

Approach

- Add a failing pipeline test in `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`.
  - Build a fixture shaped like `packages/ts/eslint/g3ts-eslint-config-checks`:
    - root package plus workspace members
    - `crates/runtime/src/full_config`
    - more than 10 sibling `.rs` files inside that nested module folder
  - Assert `RS-ARCH-FILETREE-07` fires on `crates/runtime/Cargo.toml`.
- Read the arch ingestion structure root and traversal code again after the test fails.
  - Focus on the interaction between root-package workspaces and member crate traversal.
  - Fix ingestion, not the pure rule, unless the new test proves the facts are correct and the rule is wrong.
- Re-run targeted tests and `g3rs validate` on:
  - `packages/rs/arch/g3rs-arch-ingestion`
  - `packages/rs/arch/g3rs-arch-file-tree-checks`
  - `packages/ts/eslint/g3ts-eslint-config-checks --family arch --inventory`
- Run one adversarial re-check after the fix lands.

Key decisions

- Prove the bug at the pipeline layer first.
  - The user-observed failure is in end-to-end validation of a package root.
  - The correct first proof is a file-tree ingestion pipeline test with the same workspace shape.
- Keep the fix in arch ingestion if possible.
  - `RS-ARCH-FILETREE-07` already compares aggregate facts against thresholds.
  - The likely gap is still in how those facts are gathered for mixed root-package workspaces.

Files to modify

- `.plans/2026-04-20-173954-fix-arch-structural-split-real-package-gap.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/run.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace/structure.rs`
- `.worklogs/2026-04-20-173954-fix-arch-structural-split-real-package-gap.md`
