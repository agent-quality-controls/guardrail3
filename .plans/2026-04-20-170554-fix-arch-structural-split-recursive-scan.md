Goal

Make `RS-ARCH-FILETREE-07` enforce its intended contract: prevent giant file and directory piles anywhere inside a crate's module tree, not only at the crate root or `src/` root.

Approach

- Add failing pipeline tests in `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs` that prove nested hotspots are currently missed.
  - one test for a nested directory with too many sibling `.rs` files
  - one test for a nested directory with too many sibling child directories
- Fix the aggregate in `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace.rs`.
  - replace the current top-level-only sibling counting with recursive max counting across the crate tree, stopping at nested crates the same way the existing traversal does
- Thread the corrected aggregate through the existing arch types and file-tree input path.
  - keep the rule in `packages/rs/arch/g3rs-arch-file-tree-checks/crates/runtime/src/rs_arch_07a_structural_split.rs` pure
  - if the existing field names become misleading enough, rename them to reflect recursive maxima instead of root-level counts
- Re-run the relevant tests and `g3rs validate` for:
  - `packages/rs/arch/g3rs-arch-ingestion`
  - `packages/rs/arch/g3rs-arch-file-tree-checks`
  - `packages/rs/arch/g3rs-arch-types`

Key decisions

- Fix in ingestion, not in the rule.
  - The rule should continue to read precomputed crate facts.
  - Recursive tree walking belongs in the orchestrator/ingestion boundary.
- Test the pipeline, not only the pure rule.
  - The bug is not that the rule compares numbers wrongly.
  - The bug is that the numbers being fed into the rule are incomplete.

Files to modify

- `.plans/2026-04-20-170554-fix-arch-structural-split-recursive-scan.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree_tests/pipeline.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/workspace.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`
- `packages/rs/arch/g3rs-arch-types/src/types.rs`
- `.worklogs/<timestamp>-fix-arch-structural-split-recursive-scan.md`
