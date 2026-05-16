# Function-Level Test Deletion Gate

## Goal

Make deleted unit-test behavior auditable at the Rust `#[test]` function level.

The verifier must fail when a test function disappears unless one of these is true:

- the function-level fixture ledger marks it as fixture-covered behavior
- the kept-test disposition ledger marks it as covered by a fixture3 output suite
- the function-level fixture ledger marks it as replay-system behavior that is intentionally not a product guardrail behavior test

## Current Bug

`scripts/behavior/verify-test-deletion.py` reads `behavior/migration/g3rs-test-ledger.toml`.

That file has only 16 file-level rows. The real behavior ledger is `behavior/migration/g3rs-test-fixture-ledger.toml`, which has one row per Rust `#[test]` function.

That means a Rust test function can be deleted without a function-level replacement proof if its file still exists or if it is outside the old 16-row file ledger.

## Required Behavior

Modify `scripts/behavior/verify-test-deletion.py`.

The script must:

- read `behavior/migration/g3rs-test-fixture-ledger.toml`
- read `behavior/migration/g3rs-kept-test-disposition.toml`
- discover active Rust tests through `scripts/behavior/list-rust-tests.py --format json`
- compare active tests against ledger rows by `(test_path, test_name)`
- reject duplicate rows in either ledger
- reject active tests missing from the function-level fixture ledger
- reject deleted tests unless they have explicit replacement or keep-proof

Modify `scripts/behavior/verify-test-fixture-ledger.py`.

The script must:

- keep rejecting active tests missing from the function-level fixture ledger
- allow historical rows for deleted tests only when the same replacement rules allow deletion
- keep rejecting deleted tests with non-replacement statuses

Modify `scripts/behavior/classify-test-fixture-ledger.py`.

The script must:

- keep generating rows for active Rust tests
- preserve existing historical rows for deleted tests only when the same replacement rules allow deletion
- not preserve deleted rows that still need active tests

## Deletion Rules

Allow a missing test only when its function-level row has:

- `status = "covered_hit"`
- `status = "covered_non_hit"`

Also allow a missing test when its function-level row has `status = "kept_compile_contract"` and its kept disposition is:

- `covered_by_cli_output`
- `covered_by_renderer_output`

Require an active test to still exist when its function-level row has `status = "kept_compile_contract"` and its kept disposition is:

- `needs_serialized_ingestion_output`
- `needs_rule_fixture_or_golden_output`
- `needs_family_runner_output`
- `needs_validate_command_output`
- `keep_public_api_contract`

Require an active test to still exist when its function-level row has:

- `status = "not_cli_visible"`
- `status = "kept_replay_system"`

Reject every `unclassified` missing test.

## Output Contract

Success:

```text
behavior-test-deletion: PASS rows:<n> active:<n> replaceable:<n> kept:<n>
```

Failure:

```text
behavior-test-deletion: FAIL
  <path>::<test>: <specific failure>
```

## Files To Modify

- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-test-fixture-ledger.py`
- `scripts/behavior/classify-test-fixture-ledger.py`

## Verification

Run:

```sh
python3 -m py_compile scripts/behavior/verify-test-deletion.py
python3 scripts/behavior/verify-test-deletion.py
python3 scripts/behavior/verify-all.sh
```

Add temporary negative probes during implementation and do not commit them:

- remove one active ledger row and verify failure
- change a kept test to a nonexistent name with a `needs_*` disposition and verify failure
- change a covered test to a nonexistent name with `covered_by_cli_output` disposition and verify pass
