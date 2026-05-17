# Update Family Rule Fixture Plan

## Summary

Updated the family-rule fixture plan so the new family fixtures are a replacement corpus, not an additive layer over the existing broad composite fixtures. The old `behavior/fixtures/g3rs/*` corpus is now explicitly transitional.

## Decisions Made

- Added target split between global CLI gating fixtures and minimized family rule fixtures.
- Marked `behavior/fixtures/g3rs/*` as transitional broad composite coverage.
- Added target `behavior/fixtures/g3rs-global/*` for global CLI/adoption/gating states.
- Kept `behavior/fixtures/g3rs-rule/<family>/*` as the minimized family rule corpus.
- Added migration rule to remove or reduce broad fixtures after family coverage replaces them.

## Key Files

- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md`
- `.plans/2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml`

## Next Steps

- Start with cargo.
- Build minimized cargo fixtures under `behavior/fixtures/g3rs-rule/cargo`.
- Then reduce old broad fixtures that only existed for cargo rule coverage.
