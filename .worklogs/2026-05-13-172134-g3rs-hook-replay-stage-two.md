## Summary

Added Stage 2 hook replay fixtures for validate-repo behavior and made replay coverage stricter so fixture rows must be proven by the named fixture, not by unrelated baselines.

## Decisions Made

- Added R16 and R17 validate-repo fixtures for weakened hook steps and invalid modular hook scripts.
- Added hook contract inventory output so loaded family hook contracts are visible as inventory without being counted as violations.
- Changed positive hook tool availability findings from Error inventory to Info inventory because inventory is not a failure path.
- Tightened replay verification so R16 and R17 are closed fixtures and unlisted Error/Warn findings fail baseline verification.
- Fixed executable command context detection so inert text around `g3rs validate-repo` does not create a false `g3rs validate` finding.
- Removed duplicated availability rule construction by moving shared result emission into hook config support.

## Key Files

- `.plans/2026-05-13-162822-g3rs-hook-replay-stage-two.md`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `behavior/fixtures/g3rs-validate-repo/R16-hooks-required-steps-present-but-weakened`
- `behavior/fixtures/g3rs-validate-repo/R17-hooks-modular-scripts-invalid`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `packages/rs/hooks/g3rs-hooks-config-checks`
- `packages/rs/hooks/g3rs-hooks-source-checks`
- `packages/rs/hooks/g3rs-hooks-ingestion`

## Verification

- `scripts/behavior/verify-all.sh`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-baselines.py --manifest .plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `cargo test -p g3rs-hooks-config-checks-runtime`
- `cargo test -p g3rs-hooks-source-checks-runtime`
- `g3rs validate --path packages/rs/hooks/g3rs-hooks-config-checks --inventory`
- `g3rs validate --path packages/rs/hooks/g3rs-hooks-source-checks --inventory`
- `g3rs validate --path packages/rs/hooks/g3rs-hooks-ingestion --inventory`
- `git diff --check`

## Adversarial Review

- First review found R16/R17 closure gaps, fixture-scoped coverage gaps, inventory severity mistakes, an executable-command false positive, and incorrect invalid-state labels.
- Those issues were fixed and mechanically reverified.
- Final adversarial review found no remaining findings.

## Next Steps

- Continue fixture replay coverage above the current hook layer after selecting the next fixture level from the coverage matrix.
