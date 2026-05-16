# Add clippy rule fixtures

## Summary

Split family-rule fixtures into their own fixture3 suite and added minimized clippy family fixtures. Clippy now has one clean golden fixture and six broken fixtures that cover every CLI-exposable clippy rule.

## Decisions made

- Split `g3rs-rule-fixtures` out of `g3rs-validate` because the broad suite golden was already within 3710 bytes of the repository file-size limit.
- Kept broad layered fixtures in `g3rs-validate`; moved only `behavior/fixtures/g3rs-rules/**` into the new suite.
- Marked `g3rs-clippy/policy-context-parseable` as CLI-unreachable for its Error branch because invalid `guardrail3-rs.toml` is rejected before clippy checks run.
- Kept `g3rs-clippy/policy-context-parseable` visible through clean fixture inventory.
- Added classifier support for the two clippy policy-context test dispositions so the kept-test ledger remains generated from current sources.

## Key files for context

- `fixture3.yaml`
- `behavior/fixtures/g3rs-rules/clippy`
- `behavior/golden/g3rs-rule-fixtures`
- `scripts/behavior/verify-family-rule-fixtures.py`
- `scripts/behavior/classify-kept-test-dispositions.py`
- `.plans/2026-05-16-213753-split-family-rule-fixture-suite.md`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`

## Verification

- `fixture3 check --suite g3rs-rule-fixtures`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

## Next steps

- Continue the family-rule fixture sequence with `deny`.
- Keep new family fixtures in `g3rs-rule-fixtures`; do not add them back to the broad `g3rs-validate` suite.
