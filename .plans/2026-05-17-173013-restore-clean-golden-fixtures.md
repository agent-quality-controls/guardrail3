# Goal

Restore valid `R00-clean-golden` fixtures after the deep fixture reduction pass.

Clean golden fixtures are not minimization targets. Their job is to remain a full valid workspace that detects regressions where valid projects start failing.

# Approach

- Restore every `behavior/fixtures/g3rs-rule/*/*R00-clean-golden` directory from commit `5ab595eb5`, the commit where clean fixtures were still intentionally unchanged.
- Keep the reduced broken fixtures from commit `1a5b09a9e`.
- Change `scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py` so normal operation cannot reduce clean fixtures:
  - remove `--include-clean`
  - remove `--all-cases`
  - always skip directories containing `R00-clean-golden`
- Keep the semantic `rule-set` oracle because it is valid for broken fixture reduction.
- Re-approve and verify the fixture suite after restoring clean fixtures.

# Key Decisions

- Do not reduce clean fixtures.
- Do not preserve the deep-reduced clean fixture state.
- Do not consolidate to one global clean fixture in this change. The current verifier requires one clean fixture per family, so consolidation needs a separate manifest change and verifier change.

# Files To Modify

- `behavior/fixtures/g3rs-rule/*/*R00-clean-golden`
- `scripts/behavior/reduce-g3rs-broken-family-rule-fixtures.py`
- `behavior/golden/g3rs-rule/approved.normalized.json`
- `behavior/golden/g3rs-rule/approved.meta.json`
