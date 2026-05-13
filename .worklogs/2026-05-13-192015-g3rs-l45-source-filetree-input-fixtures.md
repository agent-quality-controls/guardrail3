Summary

- Added the L45 behavior replay fixture for malformed readable Rust source/filetree input failures.
- Fixed two ingestion bugs that hid the fixture behavior: test activation no longer aborts on malformed test source, and arch filetree module discovery skips only `ParseFailed` while propagating other read/discovery errors.
- Updated behavior manifests, the L45 baseline, and the rule coverage matrix so four source/filetree input-failure rules are now covered through public CLI replay.

Decisions made

- Kept one minimal L45 fixture with two malformed files: `src/broken_garde.rs` and `crates/runtime/tests/broken_source.rs`.
- Removed the redundant malformed runtime source file after adversarial review proved the remaining two files still cover all four target rule IDs.
- Made baseline Error/Warn closure manifest-driven through `required_results` instead of hardcoded fixture prefixes.
- Rejected outward symlink reuse in the fixture. The fixture now owns its files and has no symlinks.
- Fixed the arch regression test to avoid direct vector indexing after the commit hook's clippy gate rejected `input.crates[0]`.

Key files for context

- `.plans/2026-05-13-183139-g3rs-l45-source-filetree-input-failures.md`
- `.plans/2026-05-13-184037-g3rs-test-activation-lenient-source-parse.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L45-source-and-filetree-input-failures`
- `behavior/baselines/g3rs/L45-source-and-filetree-input-failures/command-00.json`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/activation.rs`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/file_tree.rs`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-fixtures.py`

Verification

- `cargo test --manifest-path packages/rs/test/g3rs-test-ingestion/crates/runtime/Cargo.toml`
- `cargo test --manifest-path packages/rs/arch/g3rs-arch-ingestion/crates/runtime/Cargo.toml`
- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `apps/guardrail3-rs/target/debug/g3rs validate --path apps/guardrail3-rs --inventory`
- `apps/guardrail3-rs/target/debug/g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`

Adversarial review

- First review found L45 Error/Warn closure was not enforced for unlisted findings.
- Second review found the copied fixture symlink was non-hermetic and arch ingestion skipped too many errors.
- Third review found one malformed source file was redundant.
- Final convergence review found no MUST FIX.

Next steps

- Continue the behavior replay migration with the next fixture layer from the coverage matrix.
