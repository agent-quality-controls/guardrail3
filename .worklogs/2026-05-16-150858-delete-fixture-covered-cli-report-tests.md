Summary

- Deleted the first batch of fixture-covered Rust tests: CLI output tests and plain-text report output tests.
- Removed the now-unused test module declarations and dev-only dependencies.
- Regenerated the behavior ledgers so the deleted tests stay recorded as historical fixture-covered rows.

Decisions made

- Started with CLI and report output because their replacement proof is direct: `g3rs-cli-output` and `g3rs-report-output` fixture3 suites compare full stdout, stderr, exit code, and rendered text.
- Deleted whole sidecar modules only where every test in the module was already replaceable.
- Kept the historical ledger rows instead of dropping them, so deletion remains auditable.
- Removed unused dev dependencies rather than adding dummy imports.

Key files for context

- `.plans/2026-05-16-150421-delete-fixture-covered-cli-report-tests.md`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/src/plain_text.rs`
- `behavior/migration/g3rs-test-fixture-ledger.toml`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `scripts/behavior/verify-test-deletion.py`

Verification

- `cargo test --manifest-path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3-rs/crates/io/outbound/report/crates/runtime/Cargo.toml`
- `python3 scripts/behavior/classify-test-fixture-ledger.py --check`
- `python3 scripts/behavior/verify-test-fixture-ledger.py --strict`
- `python3 scripts/behavior/classify-kept-test-dispositions.py --check`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate workspace --path apps/guardrail3-rs --family code --family test --inventory`
- `git diff --check`

Next steps

- Continue deletion by finding files where every active `#[test]` is fixture-covered.
- Do not delete rows with `needs_*`, `keep_public_api_contract`, `kept_replay_system`, `not_cli_visible`, or `unclassified`.
