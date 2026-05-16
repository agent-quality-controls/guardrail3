# Delete Fixture-Covered Cargo Config Rule Tests

## Goal

Delete the next bounded group of Rust unit tests already replaced by fixture replay.

This batch covers `packages/rs/cargo/g3rs-cargo-config-checks` rule-sidecar tests where every active test in the targeted `rule_tests` directory is classified as fixture-covered.

## Files Deleted

- `disallowed_macros_deny/rule_tests`
- `lint_levels/rule_tests`
- `priority_order/rule_tests`
- `resolver/rule_tests`
- `workspace_lints/rule_tests`
- `workspace_metadata/rule_tests`

## Files Updated

- remove the `#[cfg(test)]` `rule_tests` module declaration from each corresponding `rule.rs`
- regenerate `behavior/migration/g3rs-test-fixture-ledger.toml`
- regenerate `behavior/migration/g3rs-kept-test-disposition.toml`

## Replacement Proof

Before deletion, every active `#[test]` in the targeted files was classified as replaceable by the function-level behavior ledger.

After deletion:

```text
behavior-test-deletion: PASS rows:1577 active:1530 replaceable:837 kept:740
```

The ledger keeps the historical rows, so deleting the test files does not erase the behavior record.

## Verification

Run:

```sh
cargo test --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/Cargo.toml --workspace --all-targets --all-features
python3 scripts/behavior/classify-test-fixture-ledger.py --check
python3 scripts/behavior/verify-test-fixture-ledger.py --strict
python3 scripts/behavior/classify-kept-test-dispositions.py --check
python3 scripts/behavior/verify-kept-test-dispositions.py
python3 scripts/behavior/verify-test-deletion.py
bash scripts/behavior/verify-all.sh
g3rs validate workspace --path packages/rs/cargo/g3rs-cargo-config-checks --family cargo --family code --family test --inventory
git diff --check
```

