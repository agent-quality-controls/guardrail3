Summary:
Planned complete Rust family-rule fixture coverage for all active `g3rs-*` rule namespaces. The plan defines the Cargo fixture set as the reference and lays out clean-golden plus broken-fixture groups for the remaining 13 families.

Decisions made:
- Counted active rule IDs directly from `packages/rs` instead of reusing the older partial manifest.
- Included 14 namespaces and 247 active unique rule IDs.
- Kept the target contract at the CLI surface: one clean golden per family and broken fixtures where each targeted rule emits `Error` or `Warn`.

Key files for context:
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `scripts/behavior/verify-family-rule-fixtures.py`
- `behavior/fixtures/g3rs-rules/cargo`

Verification:
- `git diff --check`
- Parsed `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml` with Python `tomllib`.
- Verified manifest family rule counts sum to 247.

Next steps:
- Implement the next planned family fixture set, starting with `fmt`.
- Update `verify-family-rule-fixtures.py` so completed families must break every active family rule ID unless a kept-test ledger row explicitly marks it non-CLI-exposable.
