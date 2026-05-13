## Summary

Expanded the G3RS L60 behavior fixture to cover every additional parseable delegated-policy row I could prove visible without shadowing existing rows.

The fixture now covers rustfmt settings, missing rustfmt toolchain component, Cargo lint/resolver policy, and broader `deny.toml` policy in addition to the previous clippy, deny, and mutants rows.

## Decisions Made

- Added only probe-proven rows where existing L60 Error/Warn rows remained visible.
- Rejected `nextest-timeouts` for L60 because activation either adds unrelated source-policy findings or causes cargo-gate failure.
- Rejected deprecated cargo-deny advisory fields because `cargo deny check` fails on them before this remains a clean policy fixture.
- Rejected release config rows because making release checks active requires publishable crates and pulls in publish dry-run behavior.
- Rejected `licenses.private.ignore = false` because it makes `cargo deny check` fail on the fixture crate license state.

## Key Files

- `.plans/2026-05-13-044201-expand-g3rs-l60-policy-coverage.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid`
- `behavior/baselines/g3rs/L60-delegated-tools-present-policy-invalid/command-00.json`

## Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate-repo`
- `git diff --check`

## Next Steps

- Continue after L60 with the next behavior layer. Do not merge nextest, release, or command-failure cases into L60 unless their activation changes in the implementation.
