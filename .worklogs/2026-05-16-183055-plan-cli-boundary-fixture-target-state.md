# Plan CLI Boundary Fixture Target State

## Summary

Reviewed the current fixture suites against the clarified requirement that durable fixtures should target client-facing behavior, not internal ingestion structs. The current client-facing fixture suites are useful; the `g3rs-code-ingestion` suite and fixture-output crate are internal-only and should be removed.

## Decisions Made

- Keep `g3rs-validate`, `g3rs-validate-repo`, `g3rs-cli-output`, and `g3rs-report-output`.
- Remove `g3rs-code-ingestion` as durable fixture infrastructure.
- Stop treating `needs_serialized_ingestion_output` as a valid migration target.
- Reclassify the 421 affected ledger rows toward CLI fixtures, rule fixtures, retained unit tests, or deleted internal assertions.

## Key Files

- `.plans/2026-05-16-182952-cli-boundary-fixture-target-state.md`
- `.plans/2026-05-16-182952-cli-boundary-fixture-target-state.md.manifest.toml`
- `fixture3.yaml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output`
- `scripts/behavior/fixture3-g3rs-code-ingestion.py`
- `behavior/golden/g3rs-code-ingestion`

## Next Steps

- Remove the internal code-ingestion fixture suite and serializer package.
- Update fixture verification scripts if they require the removed suite.
- Reclassify every `needs_serialized_ingestion_output` ledger row.
- Run fixture3, behavior verification, repo validation, and diff checks.
