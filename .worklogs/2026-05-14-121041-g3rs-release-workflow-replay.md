## Summary

Removed the obsolete `goldencheck` CLI from the user Cargo install state and continued the fixture coverage plan with release workflow replay coverage.

Release repo-root checks are now reachable through `g3rs validate --family release`, and behavior replay now covers the release workflow warnings plus release profile inventory.

## Decisions

- Wired `g3rs-release-repo-root-checks` into the process family runner instead of duplicating workflow checks in the runner.
- Replaced the release repo-root ingestion stub with the existing collected release repo facts, so workflow detection remains in one ingestion path.
- Added `L70-release-workflow-policy-violated` as a warning-only fixture because these workflow checks emit `Warn`, and `g3rs` exits zero for warnings.
- Added a release workflow to the L80 clean fixture so the clean replay stays free of warning rows after repo-root release checks became active.
- Fixed `validate-repo` marker-pair scanning to ignore archived `legacy/` workspaces. Adding guardrail configs to archived legacy code would make the archive active again, which is the wrong boundary.
- Updated the fixture coverage matrix from 231 covered / 35 planned to 235 covered / 31 planned.

## Key Files

- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs.rs`
- `behavior/fixtures/g3rs/L70-release-workflow-policy-violated/fixture.toml`
- `behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo/.github/workflows/release.yml`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `behavior/golden/g3rs-validate/approved.normalized.json`

## Verification

- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-family-runner-process`
- `cargo test --manifest-path packages/rs/release/g3rs-release-ingestion/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --force`
- `python3 -m py_compile scripts/behavior/*.py`
- `fixture3 check --all --json`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `g3rs validate-repo`
- `git diff --check`

## Next Steps

- Continue the fixture coverage closure plan with the next planned fixture group after release workflow coverage.
