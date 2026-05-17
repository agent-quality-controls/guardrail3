Summary:
- Added the ignored Node module stubs required by the G3TS style clean fixture.

Decisions made:
- Kept the stubs inside the fixture because the ESLint and Stylelint parser helpers intentionally use Node module resolution.
- Used `git add -f` for these fixture files because repo ignore rules exclude `node_modules` globally.

Key files for context:
- behavior/fixtures/g3ts-rule/style/style-R00-clean-golden/repo/node_modules
- behavior/fixtures/g3ts-rule/style/style-R40-incomplete-stylelint/repo/node_modules

Verification:
- fixture3 check --suite g3ts-rule

Next steps:
- Continue with topology, arch, apparch, hooks, and Astro family fixtures.
