# Code File-Tree 35 Hardening

## Goal

- Close the coverage and parity gaps found by the multi-agent test-attack pass for RS-CODE-FILETREE-35.

## Approach

- Add failing tests first for fixture exclusion, isolated threshold branches, quiet baseline, stronger pipeline assertions, multi-root exact attribution, member-threshold quiet behavior, glob/exclude member discovery, and zero-rust-file roots.
- Fix code only if a new failing test exposes a real behavior gap.
- Clean up the stale stub comment in code ingestion once tests are green.

## Key Decisions

- Treat the old app scoped-files behavior as non-applicable to the package model; do not add scoped package inputs.
- Strengthen tests in place instead of adding a second assertions crate layer for ingestion.

## Files To Modify

- packages/rs/code/g3rs-code-file-tree-checks/crates/runtime/src/rs_code_filetree_35_root_structural_cap_tests/mod.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/file_tree.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs
