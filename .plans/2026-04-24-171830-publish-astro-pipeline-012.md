## Goal

Publish the latest `eslint-plugin-astro-pipeline` hardening changes as `0.1.2` so external Astro apps can install the same plugin behavior that `g3ts` now requires.

## Approach

1. Bump `packages/ts/eslint-plugin-astro-pipeline/package.json` from `0.1.1` to `0.1.2`.
2. Run the package test suite.
3. Pack/publish to npm using the existing package release scripts.
4. Verify the registry reports `0.1.2`.

## Key decisions

- Patch release only.
  - The change fixes enforcement gaps in existing rules and does not introduce a new public rule ID.
- Keep the package name and public release surface unchanged.

## Files to modify

- `packages/ts/eslint-plugin-astro-pipeline/package.json`
