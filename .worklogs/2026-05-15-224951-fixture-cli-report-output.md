Summary

- Added fixture3 coverage for G3RS CLI output and report renderer output.
- Updated the coverage/disposition ledgers so CLI output and renderer output are no longer marked as kept-test-only behavior.
- Kept the report fixture harness inside the existing report assertions package to avoid adding a sibling crate that violates app architecture.

Decisions made

- Hook rows replaced by `g3rs-hooks/managed-g3rs-hook-chain` are now counted as replaced behavior, not uncovered behavior.
- `fixture3.yaml` now declares suite-specific schema versions because `g3rs-code-ingestion` and `g3rs-report-output` do not use the default replay schema.
- The report output binary compiles its fixture harness module directly. This avoids a forbidden assertions-package self-import while keeping the harness out of the runtime renderer crate.

Key files for context

- `.plans/2026-05-15-220723-fixture-after-cli-refactor-implementation.md`
- `.plans/2026-05-15-220723-fixture-after-cli-refactor-implementation.md.manifest.toml`
- `fixture3.yaml`
- `scripts/behavior/verify-rule-coverage.py`
- `scripts/behavior/verify-fixture3-migration.py`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `scripts/behavior/fixture3-g3rs-report-output.py`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/main.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/mod.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/assertions/src/report_fixture_output/fixture_output.rs`
- `behavior/fixtures/g3rs-cli-output`
- `behavior/fixtures/g3rs-report-output`

Verification

- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-fixture3-migration.py`
- `python3 scripts/behavior/verify-rule-coverage.py`
- `bash scripts/behavior/verify-all.sh`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-report-assertions`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-report-assertions --all-targets -- -D warnings`
- `g3rs validate workspace --path apps/guardrail3-rs --family cargo --family apparch --family test --inventory`

Next steps

- Add validate-command output fixtures for the remaining validate-command disposition rows.
- Add family-runner output fixtures where the family-runner behavior is externally observable.
- Continue serializing ingestion facts directly instead of adding fixture adapters.
