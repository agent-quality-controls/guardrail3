## Summary

Expanded the existing `L60-delegated-tools-present-policy-invalid` replay fixture to cover two previously absent rule IDs.

The fixture now emits `g3rs-cargo/approved-allow-inventory` and `g3rs-toolchain/msrv-consistency` without adding a new fixture layer.

## Decisions

- Used the existing L60 fixture instead of adding a new fixture because both new mutations depend on valid required inputs and invalid delegated policy, and neither hides existing L60 findings.
- Triggered `g3rs-toolchain/msrv-consistency` with `rust-toolchain.toml` pinned to `1.85.0` and workspace package `rust-version = "1.86"`.
- Avoided `1.84.0` because it cannot parse edition 2024 Cargo manifests and causes delegated Cargo gate failure.
- Triggered `g3rs-cargo/approved-allow-inventory` with a root package `[lints.clippy] module_name_repetitions = "allow"` and no matching waiver.
- Updated goldencheck output, fixture required results, and the coverage matrix counts.

## Key Files

- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid/repo/Cargo.toml`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid/repo/rust-toolchain.toml`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `behavior/golden/g3rs-validate/approved.normalized.json`

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- `goldencheck check --all`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- local Stage 1 audit script confirmed the two new L60 rows and no delegated Cargo gate failure.

## Next Steps

- Build Stage 2 from `.plans/2026-05-14-110900-g3rs-fixture-coverage-closure.md`: add the release workflow fixture only if existing release fixtures cannot expose the four absent release workflow rows without hiding metadata checks.
