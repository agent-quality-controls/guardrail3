# G3RS behavior fixtures

## Summary

Created the first G3RS behavior replay fixture stack and copied the clean L80 fixture from `packages/rs/deny/g3rs-deny-config-checks`.

Added fixture ignore handling so repo-wide crawls and marker-pair validation do not treat replay fixtures as real adopted workspaces.

Fixed pre-commit routing so fixture files still get staged-file safety scans, but intentionally broken fixture workspaces do not get routed into live G3RS/G3TS owning-unit validation.

## Decisions

- Used `packages/rs/deny/g3rs-deny-config-checks` as the clean source because it was the largest clean active Rust package already validated by G3RS.
- Kept L00-L70 stripped to minimal files so lower levels expose one unlock layer instead of carrying a full workspace.
- Added a plan manifest plus `scripts/behavior/verify-all.sh` so fixture tree shape is mechanically checked.
- Excluded `behavior/fixtures` from shared workspace crawl recovery and repo-level marker-pair checks, while still allowing direct validation of a fixture repo path.
- Excluded `behavior/fixtures` from pre-commit live validation routing while keeping merge-conflict, secret, and file-size scans on all staged fixture files.
- Moved marker-pair result-shape assertions into the validate-command assertions crate to satisfy the existing test boundary policy.

## Key Files

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs`
- `scripts/behavior/verify-fixtures.py`
- `scripts/behavior/verify-all.sh`
- `packages/shared/g3-workspace-crawl/crates/runtime/src/recovery.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs.rs`
- `.githooks/pre-commit`

## Verification

- `scripts/behavior/verify-all.sh`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `g3rs validate --path packages/shared/g3-workspace-crawl --inventory`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate-repo`
- `cargo test --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml --workspace`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command`

## Next Steps

- Add the baseline generator and baseline verifier after the fixture shape is accepted.
- Start the temporary migration ledger when deleting old behavior tests.
