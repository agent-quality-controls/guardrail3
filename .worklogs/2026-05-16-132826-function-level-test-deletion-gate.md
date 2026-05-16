Summary

- Replaced the old file-level deletion proof with a function-level deletion gate over `g3rs-test-fixture-ledger.toml`.
- The verifier now distinguishes fixture-replaceable tests from tests that must remain active.
- The fixture ledger classifier and verifier now permit historical rows only when deletion is explicitly fixture-covered.

Decisions made

- Kept `g3rs-test-fixture-ledger.toml` as the historical behavior ledger instead of treating it as a mirror of active tests only.
- Required the plan manifest to pin the expected row count, so deleting a test and deleting its ledger row cannot silently pass.
- Allowed missing tests only for `covered_hit`, `covered_non_hit`, `covered_by_cli_output`, and `covered_by_renderer_output`.
- Required `needs_*`, `keep_public_api_contract`, `kept_replay_system`, `not_cli_visible`, and `unclassified` rows to remain active.

Key files for context

- `.plans/2026-05-16-131516-function-level-test-deletion-gate.md`
- `.plans/2026-05-16-131516-function-level-test-deletion-gate.md.manifest.toml`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-test-fixture-ledger.py`
- `scripts/behavior/classify-test-fixture-ledger.py`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`

Verification

- `python3 -m py_compile scripts/behavior/verify-test-deletion.py scripts/behavior/verify-test-fixture-ledger.py scripts/behavior/classify-test-fixture-ledger.py`
- `python3 scripts/behavior/classify-test-fixture-ledger.py --check`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate workspace --path apps/guardrail3-rs --family code --family test --inventory`
- `git diff --check`

Negative probes

- Removing one active ledger row failed the deletion verifier.
- Renaming a `needs_*` row to a nonexistent test failed the deletion verifier.
- Adding a synthetic deleted fixture-covered row passed when the temporary manifest expected-row count was updated.

Next steps

- Add validate-command fixture output for rows still classified as `needs_validate_command_output`.
- Add family-runner fixture output for rows still classified as `needs_family_runner_output`.
