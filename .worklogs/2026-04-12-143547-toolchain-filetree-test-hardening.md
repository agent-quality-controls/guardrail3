# Toolchain Filetree Test Hardening

## Summary

Ran a multi-agent `test-attack` pass on the new toolchain filetree lane and hardened the package tests around the real gaps it found. The package implementation did not need to change; the missing work was exact aggregate assertions plus filetree-boundary fixtures for malformed, unreadable, and deleted-after-crawl root files.

## Decisions made

- Ignored the old app runtime ownership finding.
  - Why: under the current model the old app is inventory only, not a runtime we preserve.
- Added aggregate runtime tests in `run_tests/` instead of overloading the per-rule sidecars.
  - Why: the gap was interaction between `g3rs-toolchain/exists` and `04`, not the individual rule bodies.
- Treated malformed, unreadable, and deleted-after-crawl root files as valid filetree presence.
  - Why: the filetree lane is path-based and parse-free by design.
- Added a final convergence attack agent after the new tests passed.
  - Why: the first attack found coverage gaps, so completion required a second adversarial pass proving they were closed.

## Key files for context

- `.plans/2026-04-12-143124-toolchain-filetree-test-hardening.md`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_04_legacy_file_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/filetree.rs`

## Next steps

- `toolchain` package work is clean after the re-attack.
- The next small remaining family slice is still `fmt` filetree.
