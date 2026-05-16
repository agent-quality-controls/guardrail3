Summary

- Deleted fixture-covered rule-sidecar tests from `g3rs-cargo-config-checks`.
- Removed now-empty `rule_tests` module declarations from the corresponding cargo rule files.
- Regenerated behavior ledgers so deleted test functions stay in the historical fixture ledger.

Decisions made

- Deleted only directories where every active test was already classified as fixture-covered.
- Kept all non-replaceable cargo tests in place.
- Accepted the package-local `Cargo.lock` refresh because `g3rs-cargo-types` already depends on `serde`; cargo test made the stale lockfile accurate.

Key files for context

- `.plans/2026-05-16-151533-delete-fixture-covered-cargo-config-rule-tests.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/disallowed_macros_deny/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/lint_levels/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/priority_order/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/resolver/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/workspace_lints/rule.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/workspace_metadata/rule.rs`
- `behavior/migration/g3rs-test-fixture-ledger.toml`

Verification

- `cargo test --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/Cargo.toml --workspace --all-targets --all-features`
- `cargo clippy --manifest-path packages/rs/cargo/g3rs-cargo-config-checks/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `python3 scripts/behavior/classify-test-fixture-ledger.py --check`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `python3 scripts/behavior/classify-kept-test-dispositions.py --check`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate workspace --path packages/rs/cargo/g3rs-cargo-config-checks --family cargo --family code --family test --inventory`
- `git diff --check`

Next steps

- Continue package-by-package deletion of fully fixture-covered test files.
- Prefer the next bounded package with whole directories of replaceable rule tests.
