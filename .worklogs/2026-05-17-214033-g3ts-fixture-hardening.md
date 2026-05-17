# G3TS Fixture Hardening

## Summary

Added explicit clean-fixture classification for the G3TS family-rule fixtures and made the family fixture verifier reject unclassified clean roots.

Added a single-fixture reducer replay helper so `fixture3 reduce` can be used with the existing `fixture.toml` plus `repo/` fixture layout.

## Decisions Made

- Clean fixtures are classified in the manifest as `active-clean` or `inactive-clean`.
- The verifier rejects missing, duplicated, invalid, or mismatched clean fixture classifications.
- The normal `g3ts-rule` suite remains unchanged because it intentionally compares all `82` G3TS fixtures.
- Per-fixture reduction uses a temporary single-fixture manifest plus `scripts/behavior/fixture3-g3ts-single-fixture-replay.py`.
- No broken fixture content was reduced. The reducer accepted `0` removals while preserving approved output.
- Four dependency-backed fixtures were not reducible through `fixture3 reduce` because reducer trial copies exclude generated-style folders such as `node_modules`.

## Key Files

- `.plans/2026-05-17-212304-g3ts-fixture-hardening.md`
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `scripts/behavior/fixture3-g3ts-single-fixture-replay.py`

## Verification

- `git diff --check`
- `fixture3 check --suite g3ts-rule`
- `fixture3 check --suite g3ts-validate-repo`
- `fixture3 check --suite g3ts-cli-output`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `scripts/behavior/verify-all.sh`

## Next Steps

- Do not delete G3TS tests yet.
- Use the clean-fixture classification to decide which inactive clean roots should become active clean fixtures in a separate change.
