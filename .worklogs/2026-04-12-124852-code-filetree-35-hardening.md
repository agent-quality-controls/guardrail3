# Code File-Tree 35 Hardening

## Summary

- Hardened the new `code` file-tree lane after the multi-agent test-attack pass. Added the missing fixture-exclusion, threshold-combination, multi-root attribution, and exact-output coverage, and cleaned up the stale ingestion comment.

## Decisions made

- Kept the package-model boundary and did not reintroduce old app scoped-file semantics.
  - Why: the package lane has no scoped-files input, so that old app behavior is not the right comparison point.
- Treated the root `Cargo.toml` hard-fail report as a rejected attack finding for this lane.
  - Why: without a parsed root manifest the package model cannot safely discover workspace members, so a partial fallback would either miss owned roots or over-broaden discovery.
- Strengthened existing pipeline assertions instead of adding another assertions helper crate layer.
  - Why: the missing coverage was local to one test file and did not justify more structure.

## Key files for context

- .plans/2026-04-12-123849-code-filetree-35-hardening.md
- packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/rs_code_filetree_35_root_structural_cap_tests/mod.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/file_tree.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs

## Next steps

- If we want even deeper coverage later, add a nested-owned-root pipeline case (`root -> crates/api -> crates/api/subcrate`) to pin deepest-root ownership across member-vs-member overlap.
