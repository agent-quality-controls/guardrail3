# Delete Fixture-Covered CLI And Report Tests

## Goal

Start deleting Rust unit tests only where fixture3 output already replaces the behavior.

This batch removes the first small, high-confidence group:

- CLI parse/output tests covered by `g3rs-cli-output`
- plain-text report renderer tests covered by `g3rs-report-output`

## Replacement Proof

The rows are already classified as replaceable by `scripts/behavior/verify-test-deletion.py`.

Current verifier output:

```text
behavior-test-deletion: PASS rows:1577 active:1577 replaceable:837 kept:740
```

The targeted tests are listed in `behavior/migration/g3rs-kept-test-disposition.toml` as:

- `covered_by_cli_output`
- `covered_by_renderer_output`

## Files To Delete

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli_tests/mod.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run_tests/mod.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text_tests/cases.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text_tests/mod.rs`

## Files To Update

- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs`

Remove the `#[cfg(test)]` sidecar module declarations for the deleted modules.

## Verification

Run:

```sh
cargo test --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml
cargo test --manifest-path apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/Cargo.toml
python3 scripts/behavior/classify-test-fixture-ledger.py --check
python3 scripts/behavior/verify-test-fixture-ledger.py --strict
python3 scripts/behavior/verify-test-deletion.py
bash scripts/behavior/verify-all.sh
g3rs validate workspace --path apps/guardrail3-rs --family code --family test --inventory
git diff --check
```

Expected deletion verifier after this batch:

- active tests decrease from `1577` to `1562`
- rows remain `1577`
- replaceable remains `837`
- kept remains `740`

