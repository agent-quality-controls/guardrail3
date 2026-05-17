## Summary

Closed the remaining G3RS replay coverage rows by adding targeted fixtures for config-branch and release-tool behavior, then tightened coverage verification so approved replay fixtures cannot hide unpinned Error/Warn rows.

## Decisions

- Added `L61-cargo-clippy-code-config-branches` as a config-policy fixture instead of mixing those branches into delegated-tool fixtures.
- Added `L50-release-semver-checks-missing` so `cargo-semver-checks` availability is covered in the delegated-tool-missing layer without polluting invalid semver release policy coverage.
- Kept `g3rs-clippy/unknown-keys` in `L44-clippy-typed-config-invalid` because the strict clippy parser treats the near-miss managed key as a typed config parse error.
- Strengthened `verify-g3rs-rule-fixture-coverage.py` to require every approved fixture Error/Warn row to be pinned in manifest `required_results`, not only the rows selected by coverage entries.

## Key Files

- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `scripts/behavior/verify-fixtures.py`
- `scripts/behavior/replay_common.py`
- `behavior/fixtures/g3rs/L50-release-semver-checks-missing/`
- `behavior/fixtures/g3rs/L61-cargo-clippy-code-config-branches/`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/coverage/g3rs-rule-coverage.toml`

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `fixture3 check --all --json`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `g3rs validate-repo`
- `git diff --check`

## Next Steps

- Continue fixture-layer migration above the current coverage closure only after the next fixture layer is planned with explicit shadowing rules.
