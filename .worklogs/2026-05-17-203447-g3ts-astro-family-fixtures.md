Summary

- Added G3TS fixture coverage for the remaining Astro families: setup, content, mdx, i18n, media, and seo.
- Marked those families completed in the fixture manifest.
- Approved the `g3ts-rule` fixture output after every completed family rule was either covered by Error/Warn output or explicitly classified as inventory-only or CLI-unreachable.

Decisions made

- Used one clean fixture per family and grouped broken fixtures by externally visible rule output.
- Classified Astro hook-contract rules as CLI-unreachable because they are exported for hook aggregation and are not emitted by `g3ts validate --family ...`.
- Classified `g3ts-astro-content/live-config-exists` and `g3ts-astro-seo/broad-crawler-generator-absent` as inventory-only because current CLI behavior emits them as Info, not Error or Warn.

Key files for context

- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/astro-setup`
- `behavior/fixtures/g3ts-rule/astro-content`
- `behavior/fixtures/g3ts-rule/astro-mdx`
- `behavior/fixtures/g3ts-rule/astro-i18n`
- `behavior/fixtures/g3ts-rule/astro-media`
- `behavior/fixtures/g3ts-rule/astro-seo`
- `behavior/golden/g3ts-rule/approved.normalized.json`

Verification

- `fixture3 check --suite g3ts-rule`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`

Next steps

- Audit whether any clean Astro fixtures should be upgraded from inactive no-finding roots to active valid Astro app roots.
- Run the full repo verification surface after this checkpoint.
