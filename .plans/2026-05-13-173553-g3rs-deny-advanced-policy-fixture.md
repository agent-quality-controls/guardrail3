# G3RS Deny Advanced Policy Fixture

## Goal

Implement Stage 3 from `.plans/2026-05-13-160231-g3rs-replay-coverage-matrix.md`.

End state:

- three new fixtures exist:
  - `L60-deny-cargo-valid-policy-invalid`
  - `L60-deny-schema-invalid-policy-invalid`
  - `L60-deny-deprecated-advisories-policy-invalid`
  - `L60-deny-allow-override-policy-invalid`
- the fixture replays the missing advanced deny policy rules
- the fixture is closed by baseline verification
- coverage matrix rows for those deny rules become `covered`

## Target Rules

- `g3rs-deny/allow-override-channel`
- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/license-exceptions-inventory`
- `g3rs-deny/stricter-advisories-inventory`
- `g3rs-deny/unknown-keys`
- `g3rs-deny/wrappers`

## Rule Implementation Facts

- `allow-override-channel` needs parseable `deny.toml`, valid Rust policy state, and non-empty `[bans].allow`.
- `deprecated-advisories` needs deprecated `[advisories].vulnerability`, `[advisories].notice`, or `[advisories].unsound`.
- `extra-feature-bans-inventory` needs a `[[bans.features]]` entry whose crate name is not `tokio`.
- `license-exceptions-inventory` needs `[[licenses.exceptions]]`; it can emit error/warn findings without breaking other deny checks.
- `stricter-advisories-inventory` needs `[advisories].unmaintained` or `[advisories].yanked` stricter than baseline.
- `unknown-keys` needs parser-preserved unknown keys in otherwise parseable `deny.toml`.
- `wrappers` needs parseable `deny.toml`, valid Rust policy state, and changed wrappers on a managed ban.

## Hiding Boundary

Each new fixture must start from the existing L60 valid-unlock shape:

- workspace root found
- guardrail config valid
- required inputs present
- required inputs valid
- delegated tools present

Each fixture must only make one deny policy layer invalid.

The split is required because some deny mutations are rejected by cargo-deny before cargo-deny can evaluate the full policy:

- `L60-deny-cargo-valid-policy-invalid` keeps `cargo deny check` schema-valid and carries G3RS-only deny policy drift:
  - `extra-feature-bans-inventory`
  - `license-exceptions-inventory`
  - `stricter-advisories-inventory`
  - `wrappers`
- `L60-deny-schema-invalid-policy-invalid` carries deny keys that cargo-deny rejects during config deserialization:
  - `unknown-keys`
- `L60-deny-deprecated-advisories-policy-invalid` carries deprecated advisory keys that cargo-deny rejects during config deserialization:
  - `deprecated-advisories`
- `L60-deny-allow-override-policy-invalid` carries the allow/deny overlap that cargo-deny rejects during config validation:
  - `allow-override-channel`

Do not introduce malformed TOML, missing files, missing delegated tools, nested workspaces, bad `Cargo.toml`, bad `clippy.toml`, bad `rustfmt.toml`, or release/source/test policy violations.

## Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L60-deny-cargo-valid-policy-invalid/**`
- `behavior/fixtures/g3rs/L60-deny-schema-invalid-policy-invalid/**`
- `behavior/fixtures/g3rs/L60-deny-deprecated-advisories-policy-invalid/**`
- `behavior/fixtures/g3rs/L60-deny-allow-override-policy-invalid/**`
- `behavior/baselines/g3rs/L60-deny-cargo-valid-policy-invalid/command-00.json`
- `behavior/baselines/g3rs/L60-deny-schema-invalid-policy-invalid/command-00.json`
- `behavior/baselines/g3rs/L60-deny-deprecated-advisories-policy-invalid/command-00.json`
- `behavior/baselines/g3rs/L60-deny-allow-override-policy-invalid/command-00.json`
- `behavior/coverage/g3rs-rule-coverage.toml`

Modify verifier scripts only if existing verifiers cannot represent the fixture correctly.

## Implementation Steps

1. Copy `behavior/fixtures/g3rs/L60-delegated-tools-present-policy-invalid` into the four deny fixtures.
2. Edit the new fixture metadata:
   - `id` matches each fixture directory
   - `expected_exit = "nonzero"`
   - same valid state as L60
   - `intentionally_invalid = ["delegated_policy_invalid"]`
3. Replace only each new fixture `repo/deny.toml` content to trigger the target rules assigned to that fixture.
4. Add the fixtures to the behavior fixture manifest with closed file lists and required result rows for every emitted Error/Warn and intended Info row.
5. Generate the new baselines.
6. Update `behavior/coverage/g3rs-rule-coverage.toml` rows for the target rules:
   - `coverage_status = "covered"`
   - `current_replay` must match emitted severity
   - `target_replay` must match emitted severity or intended inventory
   - `fixture` must point to the exact fixture that emits the target rule
   - reason must state which deny policy layer the fixture mutates
7. Update `scripts/behavior/verify-g3rs-rule-fixture-coverage.py` so every `target_replay = "info_inventory"` covered row must have a matching `Info|rule-id|...` required result in the named fixture manifest.
8. Run verification.
9. Send adversarial review against this plan, the coverage matrix plan, and the implementation.

## Required Verification

- `python3 -m py_compile scripts/behavior/*.py`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path behavior/fixtures/g3rs/L60-deny-cargo-valid-policy-invalid/repo --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L60-deny-schema-invalid-policy-invalid/repo --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L60-deny-deprecated-advisories-policy-invalid/repo --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L60-deny-allow-override-policy-invalid/repo --inventory`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `g3rs validate --path behavior/fixtures/g3rs/L80-project-policy-valid-clean/repo --inventory`
- `git diff --check`
