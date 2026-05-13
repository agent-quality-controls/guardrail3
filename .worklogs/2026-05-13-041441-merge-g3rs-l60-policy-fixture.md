## Summary

Merged all currently visible L60 delegated-policy failures into `L60-delegated-tools-present-policy-invalid` without introducing lower-layer missing-tool findings.

Fixed the cargo-mutants installation probe to use the supported `cargo mutants --version` command shape, then tightened the behavior runner so the L50 mutation fixture can block that cargo subcommand without breaking other delegated-tool fixtures.

## Decisions Made

- Kept clippy threshold failures, clippy test relaxation failures, deny policy failures, and mutants config sanity failures in one L60 fixture because they all remain visible together.
- Did not merge release publishability into L60 because publishable crates trigger `cargo publish --dry-run`, which changes the fixture from policy wiring into registry/package validation.
- Did not merge hooks into L60 because hook shell policy belongs to the validate-repo behavior fixture set.
- Added `blocked_cargo_subcommands = ["mutants"]` to the L50 mutation fixture instead of isolating `CARGO_HOME`, because broad Cargo-home isolation made cargo delegated gates fail before G3RS emitted missing-tool findings.

## Key Files

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-13-033126-clean-g3rs-l00-l50-and-plan-l60.md`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid`
- `behavior/fixtures/g3rs/L50-test-cargo-mutants-missing/fixture.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run.rs`
- `scripts/behavior/baseline_common.py`

## Verification

- `cargo test --manifest-path packages/rs/test/g3rs-test-ingestion/Cargo.toml -p g3rs-test-ingestion-runtime`
- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path packages/rs/test/g3rs-test-ingestion --inventory`
- `g3rs validate-repo`
- `git diff --check`

## Next Steps

- Continue with the next behavior layer after L60 once the fixture set is stable.
