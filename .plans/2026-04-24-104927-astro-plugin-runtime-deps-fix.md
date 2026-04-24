## Goal

Fix the published `eslint-plugin-astro-pipeline` package so it can be installed and loaded from npm without missing runtime-module failures, and document the current `TS-ASTRO-CONFIG-05/07` investigation result against the real landing app.

## Approach

- Reproduce the current package-shape bug from the release surface.
  - Read the plugin `package.json` and built `dist/**` imports.
  - Confirm which runtime imports are missing from publishable dependency fields.
- Add a release-surface test in the plugin package.
  - Assert that every non-relative import used by built runtime files is declared in `dependencies` or `peerDependencies`.
  - Keep the test local to the package so publish regressions fail before release.
- Fix the package manifest.
  - Move runtime parser dependencies out of `devDependencies` into `dependencies`.
  - Keep test-only tools in `devDependencies`.
- Verify package behavior mechanically.
  - `npm test`
  - `npm pack --dry-run`
- Adversarial review.
  - Have a reviewer compare the fix against the plan and try to find remaining packaging gaps or architecture mistakes.

## Key decisions

- Fix the package boundary, not `g3ts`.
  - The install/load failure is a published npm-package contract bug.
  - `g3ts` enforcement is separate and should not mask a broken package.
- Use a release-surface test instead of only moving dependency entries.
  - The real failure was that publish succeeded while runtime imports were undeclared.
  - A test on built output closes that class broadly.
- Do not change Astro-family logic for `TS-ASTRO-CONFIG-05/07` yet.
  - The current landing app does not reproduce those findings.
  - Current evidence says that report was stale state, different target, or different branch.

## Files to modify

- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/tests/*`
