## Summary

Added Stage 3 G3RS behavior replay coverage for advanced deny policy rules.

The initial combined fixture hid cargo-deny behavior, so it was split into four fixtures by delegated failure layer: cargo-deny-valid policy drift, unknown-key schema invalid, deprecated-advisory schema invalid, and allow/deny override validation invalid.

## Decisions

- Split deny replay fixtures by cargo-deny failure layer.
- Kept `L60-deny-cargo-valid-policy-invalid` free of `cargo gate failed`, so G3RS-only deny policy drift is not hidden by delegated tool schema errors.
- Put unknown top-level deny keys in `L60-deny-schema-invalid-policy-invalid`.
- Put deprecated advisory keys in `L60-deny-deprecated-advisories-policy-invalid`.
- Put allow/deny overlaps in `L60-deny-allow-override-policy-invalid`.
- Tightened `verify-g3rs-rule-fixture-coverage.py` so `target_replay = "info_inventory"` requires exact counted `Info|rule|title|path` rows in the named fixture manifest and baseline.

## Key Files

- `.plans/2026-05-13-173553-g3rs-deny-advanced-policy-fixture.md`
- `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L60-deny-cargo-valid-policy-invalid`
- `behavior/fixtures/g3rs/L60-deny-schema-invalid-policy-invalid`
- `behavior/fixtures/g3rs/L60-deny-deprecated-advisories-policy-invalid`
- `behavior/fixtures/g3rs/L60-deny-allow-override-policy-invalid`
- `behavior/coverage/g3rs-rule-coverage.toml`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`

## Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`
- Final adversarial review: no findings.

## Next Steps

- Continue the replay fixture migration from the 41 remaining planned rule IDs in `behavior/coverage/g3rs-rule-coverage.toml`.
