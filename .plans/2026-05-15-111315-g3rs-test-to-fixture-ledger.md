# G3RS Test To Fixture Ledger

## Goal

Prove behavior equivalence between active Rust tests and replay fixtures without proving fixture minimality.

The proof target is:

```text
active Rust test function
  -> ledger row
  -> expected fixture hit or non-hit
  -> normalized fixture replay output
  -> verifier exit 0
```

## Current Problem

The current `behavior/migration/g3rs-test-ledger.toml` is file-level.

It proves only that selected old test files were either deleted, kept, or migrated.

It does not prove:

- every active `#[test]` function has a row
- each row maps to a specific fixture behavior
- false-positive tests are covered by fixture non-hits
- CLI-visible behavior tests are replayed through `fixture3`

## Approach

Add a separate function-level ledger.

Do not replace the existing deletion ledger.

The existing deletion ledger remains useful for deleted file accounting.

## Files To Add

- `scripts/behavior/list-rust-tests.py`
  - scans active Rust files under `packages/rs` and `apps/guardrail3-rs`
  - ignores `target`, `.cargo-target`, and `legacy`
  - extracts `#[test] fn name` records
  - emits deterministic JSON

- `scripts/behavior/verify-test-fixture-ledger.py`
  - reads `behavior/migration/g3rs-test-fixture-ledger.toml`
  - reads fixture manifests
  - reads approved normalized fixture output
  - validates every active test is classified
  - validates covered hit rows are present in replay output
  - validates covered non-hit rows are absent from replay output
  - validates `not_cli_visible` rows carry a reason

- `behavior/migration/g3rs-test-fixture-ledger.toml`
  - starts as a generated inventory of active tests
  - default rows are `status = "unclassified"`
  - this file is intentionally not proof-complete on first creation

## Ledger Row Shape

```toml
[[test]]
test_path = "packages/rs/fmt/.../cases.rs"
test_name = "errors_when_rustfmt_missing"
status = "covered_hit"
fixture = "L30-guardrail-config-valid-required-inputs-missing"
severity = "Error"
rule = "g3rs-fmt/rustfmt-config-exists"
title = "rustfmt config missing"
file = "rustfmt.toml"
```

Valid statuses:

- `covered_hit`
- `covered_non_hit`
- `not_cli_visible`
- `kept_compile_contract`
- `kept_replay_system`
- `unclassified`

`covered_hit` requires:

- `fixture`
- `severity`
- `rule`
- `title`
- `file`

`covered_non_hit` requires:

- `fixture`
- `rule`

Optional non-hit fields:

- `severity`
- `title`
- `file`

If optional non-hit fields are present, absence is checked at that specificity.

`not_cli_visible` requires:

- `reason`

## Verifier Modes

Default mode:

- verifies schema
- verifies every active test has exactly one row
- reports unclassified count
- exits zero while the migration is still open

Strict mode:

```sh
python3 scripts/behavior/verify-test-fixture-ledger.py --strict
```

Strict mode:

- all default checks
- fails if any row is `unclassified`
- fails if any `covered_hit` is missing from fixture replay
- fails if any `covered_non_hit` is present in fixture replay

## Integration

Add default-mode verifier to:

```text
scripts/behavior/verify-all.sh
```

Do not add strict mode to `verify-all.sh` yet.

Reason:

The strict proof is not true until the ledger is classified.

## Done For This Step

- The active test inventory is generated.
- The verifier proves the ledger covers every active test function.
- The verifier can already prove hit/non-hit rows when rows are classified.
- The current output states how many tests remain unclassified.

## Not Done In This Step

- Classifying all active tests.
- Deleting tests based on the new ledger.
- Fixture minimization.
