# Classify G3RS Test Fixture Ledger

## Goal

Classify every row in `behavior/migration/g3rs-test-fixture-ledger.toml` without pretending that fixture replay proves behavior it does not prove.

The output must separate:

- replay-covered fixture hits
- replay-covered fixture non-hits
- internal compile/API contracts
- replay-system tests
- tests that still need manual classification

## Approach

Do not hand-edit 1735 rows.

Add a deterministic classifier script that:

- reads active Rust tests from `scripts/behavior/list-rust-tests.py`
- reads fixture replay output from `behavior/golden/**/approved.normalized.json`
- rewrites `behavior/migration/g3rs-test-fixture-ledger.toml`
- preserves one row per active test
- classifies only rows supported by stable path/name/rule evidence
- leaves uncertain rows as `unclassified`

## Classification Rules

### Replay-covered hit

Use `covered_hit` only when:

- the test belongs to a rule-specific check package path
- a semantic rule id can be derived from the rule module path
- that rule id appears in approved fixture replay output
- the test name indicates a violating case

The classifier records the exact fixture finding tuple:

- `fixture`
- `severity`
- `rule`
- `title`
- `file`

### Replay-covered non-hit

Use `covered_non_hit` only when:

- a semantic rule id can be derived from the rule module path
- a clean fixture has no finding for that rule
- the test name indicates a clean, allowed, ignored, or non-reporting case

The classifier records:

- `fixture = "L80-project-policy-valid-clean"` for workspace validation rules
- `rule`

### Hook replay rows

Use validate-repo fixtures for hook rules:

- `g3rs-hooks/*`
- `g3rs-*/hook-contract`

### Kept compile contract

Use `kept_compile_contract` for tests that prove public API, parser, renderer, orchestrator, or type contracts that cannot be observed as a fixture finding without losing precision.

Every row must carry `reason`.

### Kept replay system

Use `kept_replay_system` for tests that test fixture replay, golden normalization, migration ledgers, or verifier scripts themselves.

Every row must carry `reason`.

### Unclassified

Leave `unclassified` when the classifier cannot prove one of the above from code path, test name, and replay output.

## Files To Modify

- Add `scripts/behavior/classify-test-fixture-ledger.py`
- Rewrite `behavior/migration/g3rs-test-fixture-ledger.toml`
- Optionally tighten `scripts/behavior/verify-test-fixture-ledger.py` so kept rows require `reason`

## Verification

Run:

```sh
python3 scripts/behavior/classify-test-fixture-ledger.py --check
python3 scripts/behavior/verify-test-fixture-ledger.py
python3 scripts/behavior/verify-test-fixture-ledger.py --strict
scripts/behavior/verify-all.sh
git diff --check
```

Strict mode is expected to fail until every row is classified.

## Done

- The ledger has fewer `unclassified` rows than before.
- Every classified row passes verifier checks.
- No row is classified as fixture-covered unless the fixture replay output supports it.
