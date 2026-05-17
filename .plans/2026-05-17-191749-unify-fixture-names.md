# Unify Fixture Names

## Goal

Make G3RS and G3TS fixture naming symmetric.

## Required Naming

Fixture folders:

- `behavior/fixtures/g3rs-rule`
- `behavior/fixtures/g3ts-rule`
- `behavior/fixtures/g3rs-validate-repo`
- `behavior/fixtures/g3ts-validate-repo`
- `behavior/fixtures/g3rs-cli-output`
- `behavior/fixtures/g3ts-cli-output`

Golden folders:

- `behavior/golden/g3rs-rule`
- `behavior/golden/g3ts-rule`
- `behavior/golden/g3rs-validate-repo`
- `behavior/golden/g3ts-validate-repo`
- `behavior/golden/g3rs-cli-output`
- `behavior/golden/g3ts-cli-output`

Scripts:

- `scripts/behavior/fixture3-g3rs-fixture-replay.py`
- `scripts/behavior/fixture3-g3ts-fixture-replay.py`
- `scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py`
- `scripts/behavior/reduce-g3rs-fixture-oracle.py`
- `scripts/behavior/reduce-g3rs-fixture-rule-set-oracle.py`

## Approach

1. Rename active fixture and golden directories.
2. Rename active behavior scripts so every fixture-specific script has `fixture` or `fixtures` in the filename.
3. Update `fixture3.yaml`.
4. Update plan manifests and verifier scripts.
5. Run `fixture3 check --all --json`.
6. Run `scripts/behavior/verify-all.sh`.
7. Commit with a worklog.

## Non-Goals

- Do not change fixture contents.
- Do not change approved behavior.
- Do not delete historical `behavior/golden/g3rs-validate` in this change.
