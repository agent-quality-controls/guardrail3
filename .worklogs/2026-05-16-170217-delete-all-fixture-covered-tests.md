Summary

- Deleted every active Rust rule-sidecar test file that the fixture ledger already classified as replaceable.
- Active Rust tests dropped from 1530 to 740 while the fixture ledger stayed fixed at 1577 historical rows.
- Removed now-unused test-only helper functions and stale runtime dev-dependencies exposed by the deletions.

Decisions made

- Kept deleted test functions in `behavior/migration/g3rs-test-fixture-ledger.toml` instead of removing ledger rows, because the fixture ledger is the proof that deletion is allowed.
- Deleted whole `rule_tests` directories only when every active test in that directory was fixture-covered.
- Removed only the matching `#[cfg(test)]` sidecar module declarations and dead test-only helpers from production rule files.
- Removed runtime dev-dependencies only when the package no longer referenced them after sidecar deletion.

Key files for context

- `.plans/2026-05-16-152040-delete-all-fixture-covered-tests.md`
- `.plans/2026-05-16-152040-delete-all-fixture-covered-tests.md.manifest.toml`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `scripts/behavior/verify-test-deletion.py`
- `scripts/behavior/verify-all.sh`

Verification

- `bash scripts/behavior/verify-all.sh`
- `python3 scripts/behavior/verify-test-deletion.py`
- `g3rs validate repo --path "$PWD"`
- `g3rs validate workspace --path <each touched package> --inventory`
- `cargo test --manifest-path <each touched package>/Cargo.toml --workspace --all-targets --all-features`
- `cargo clippy --manifest-path <each touched package>/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `git diff --check`

Next steps

- Use the remaining 740 active tests as the deletion boundary.
- Add new behavior fixtures before deleting any remaining tests with `needs_*`, `keep_public_api_contract`, `kept_replay_system`, `not_cli_visible`, or `unclassified` dispositions.
