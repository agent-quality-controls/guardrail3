# Add Toolchain Family Rule Fixtures

## Summary

Added the toolchain family to the minimized family-rule fixture set.

The toolchain family now has one clean golden fixture and two broken fixtures that make all four active toolchain rules emit `Error` or `Warn` through the public `g3rs validate workspace` CLI surface.

## Decisions

- Kept filetree violations separate from policy violations because a missing `rust-toolchain.toml` prevents channel/component and MSRV checks from running.
- Used an unparseable Cargo `rust-version` in the policy fixture to make `g3rs-toolchain/msrv-consistency` emit `Error` while the same fixture also proves missing component detection.
- Did not add internal serialization, ingestion fixtures, or fixture-output crates.

## Key Files

- `behavior/fixtures/g3rs-rule/toolchain/toolchain-R00-clean-golden/fixture.toml`
- `behavior/fixtures/g3rs-rule/toolchain/toolchain-R10-filetree-violations/fixture.toml`
- `behavior/fixtures/g3rs-rule/toolchain/toolchain-R20-policy-violations/fixture.toml`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`
- `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md.manifest.toml`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`

## Verification

- `fixture3 check --suite g3rs-validate`
- `python3 scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3rs-rule-fixture-coverage.py`
- `python3 scripts/behavior/verify-kept-test-dispositions.py`
- `python3 scripts/behavior/verify-test-deletion.py`
- `python3 -m py_compile scripts/behavior/verify-g3rs-family-rule-fixtures.py`
- `bash scripts/behavior/verify-all.sh`
- `g3rs validate repo --path "$PWD"`
- `git diff --check`

## Next Steps

- Build the next planned family fixture set from `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`; the next listed family is `deps`.
