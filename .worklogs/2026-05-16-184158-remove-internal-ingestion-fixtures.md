# Remove Internal Ingestion Fixtures

## Summary

Removed the durable `g3rs-code-ingestion` serialized-ingestion fixture suite and its fixture-output crate. Reclassified the 421 kept ingestion rows from serialized-output migration work to retained internal unit tests until each row is either exposed through CLI fixtures or deleted as internal-shape-only.

## Decisions Made

- Kept client-facing fixture suites only: `g3rs-validate`, `g3rs-validate-repo`, `g3rs-cli-output`, and `g3rs-report-output`.
- Removed `g3rs-code-ingestion` from `fixture3.yaml` because it tested internal ingestion structs, not user-visible behavior.
- Removed `packages/rs/code/g3rs-code-ingestion/crates/fixture-output`.
- Updated deletion-gate dispositions so `needs_serialized_ingestion_output` is no longer an active target.
- Marked the serialized ingestion plans as superseded instead of deleting historical plans.

## Key Files

- `fixture3.yaml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `scripts/behavior/verify-test-deletion.py`
- `packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/guardrail3-rs.toml`
- `.plans/2026-05-16-182952-cli-boundary-fixture-target-state.md`

## Verification

- `fixture3 check --all`
- `python3 scripts/behavior/verify-fixture3-migration.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-fixture-contract-language.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `cargo test --manifest-path packages/rs/code/g3rs-code-ingestion/Cargo.toml --workspace`
- `cargo clippy --manifest-path packages/rs/code/g3rs-code-ingestion/Cargo.toml --workspace --all-targets --all-features`
- `g3rs validate repo --path "$PWD"`
- `g3rs validate workspace --path packages/rs/code/g3rs-code-ingestion --inventory`
- `git diff --check`

## Next Steps

- Audit `keep_internal_unit_test` rows family by family.
- Move rows to CLI fixtures when the behavior is product-visible.
- Delete rows that only assert internal shape and do not protect product behavior.
