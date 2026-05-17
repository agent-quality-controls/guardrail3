# Add Deps Family Rule Fixtures

## Summary

Added the deps family to the minimized family-rule fixture set.

The deps family now has one clean golden fixture and three broken fixtures that make all 11 active deps rules emit `Error` or `Warn` through the public `g3rs validate workspace` CLI surface.

## Decisions

- Kept service-profile lockfile/tool failures separate from library-profile allowlist failures because `g3rs-deps/cargo-lock-present` only errors outside library profile, while `g3rs-deps/library-allowlist-present` only warns inside library profile.
- Used `runner_mode = "path_without_delegated_tools"` to make delegated tool checks deterministic instead of relying on the host PATH.
- Used fixture-local fake executables plus `path_prepend = ["bin"]` for clean and dependency-policy fixtures so installed-tool success rows are deterministic.
- Put `g3rs-deps/direct-dependency-cap` in `deps-R21-library-allowlist-missing` because the missing allowlist gate suppresses per-dependency allowlist rows, which keeps the approved output small while still proving both rules.

## Key Files

- `behavior/fixtures/g3rs-rule/deps/deps-R00-clean-golden/fixture.toml`
- `behavior/fixtures/g3rs-rule/deps/deps-R10-required-files-and-tools/fixture.toml`
- `behavior/fixtures/g3rs-rule/deps/deps-R20-allowlist-and-count-violations/fixture.toml`
- `behavior/fixtures/g3rs-rule/deps/deps-R21-library-allowlist-missing/fixture.toml`
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

- Build the next planned family fixture set from `.plans/2026-05-16-200957-all-rs-family-rule-fixtures.md`; the next listed family is `clippy`.
