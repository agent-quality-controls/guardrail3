# Summary

Started the G3TS external-behavior fixture set.

Added `fixture3` suites for G3TS CLI output, repo validation, and family rule fixtures. Added the replay harness and verification scripts, then covered the `g3ts-package` family through CLI-visible fixtures.

# Decisions Made

- Reused the shared behavior replay helper instead of duplicating command execution, hashing, fixture copying, and output normalization.
- Added `g3ts` binary resolution to `scripts/behavior/replay_common.py`.
- Added `scripts/behavior/fixture3-g3ts-replay.py` as the G3TS suite entry point.
- Added G3TS-specific verifiers:
  - `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
  - `scripts/behavior/verify-g3ts-rule-coverage.py`
- Wired the new verifiers into `scripts/behavior/verify-all.sh`.
- Created one clean package fixture and four broken package fixtures.
- Marked only `package` as completed in the G3TS fixture manifest. The other 19 G3TS families remain planned.
- Did not delete any TS tests or assertion crates. Deletion requires every CLI-visible G3TS rule to be covered or explicitly classified.

# Key Files For Context

- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md`
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `fixture3.yaml`
- `scripts/behavior/replay_common.py`
- `scripts/behavior/fixture3-g3ts-replay.py`
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `scripts/behavior/verify-g3ts-rule-coverage.py`
- `behavior/fixtures/g3ts-rules/package`
- `behavior/golden/g3ts-rule-fixtures/approved.normalized.json`

# Verification

- `fixture3 check --suite g3ts-cli-output --json`
- `fixture3 check --suite g3ts-validate-repo --json`
- `fixture3 check --suite g3ts-rule-fixtures --json`
- `fixture3 check --all --json`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-coverage.py`
- `scripts/behavior/verify-all.sh`
- `cargo build --quiet --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts --bin g3ts`
- `git diff --check`

# Next Steps

- Continue family-by-family G3TS rule fixtures.
- Next families should be `npmrc`, `tsconfig`, `eslint`, `jscpd`, `fmt`, `spelling`, `typecov`, and `style`.
- After all families are completed, run `fixture3 reduce` on broken G3TS fixtures.
- Delete G3TS tests and assertion crates only after the G3TS coverage verifier proves every active production rule is covered, inventory-only, or CLI-unreachable.
