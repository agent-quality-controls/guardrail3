Summary

- Added G3TS `astro-state` family fixtures to `g3ts-rule`.
- Covered the clean Astro state path and the broken state path for legacy parallel state plus configured forbidden state.
- Fixed the `configured-forbidden-state` message to reference the actual `[astro.state].forbidden` TOML schema.

Decisions made

- Used `[astro.state]` in fixtures because `g3ts-toml-parser` exposes Astro policy at the top-level `astro` table.
- Kept the broken fixture small: one legacy `.next` file and one configured forbidden state file.
- Approved the changed `fixture3` output only after the fixture emitted both Astro state rule IDs.

Key files for context

- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/astro-state`
- `behavior/golden/g3ts-rule/approved.normalized.json`
- `packages/ts/astro/state/g3ts-astro-state-file-tree-checks/crates/runtime/src/configured_forbidden_state.rs`

Verification

- `fixture3 check --suite g3ts-rule`
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py`
- `python3 scripts/behavior/verify-g3ts-rule-fixture-coverage.py`
- `g3rs validate workspace --path packages/ts/astro/state/g3ts-astro-state-file-tree-checks`

Next steps

- Continue with the remaining Astro fixture families: setup, content, mdx, i18n, media, and seo.
