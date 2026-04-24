## Summary

Published `eslint-plugin-astro-pipeline@0.1.2` so external Astro apps can install the latest plugin behavior that the local `g3ts` Astro checks now require.

## Decisions made

- Used a patch release.
  - The release fixes enforcement behavior in existing rules and does not add a new public rule ID.
- Published from the package directory using the existing release scripts.
  - `prepublishOnly` ran the full plugin test suite.
  - `prepack` rebuilt the distribution before upload.

## Key files for context

- `.plans/2026-04-24-171830-publish-astro-pipeline-012.md`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `npm pack --dry-run`
- `npm publish --access public`
- `npm view eslint-plugin-astro-pipeline name version dist-tags.latest`
  - `version = '0.1.2'`
  - `dist-tags.latest = '0.1.2'`

## Next steps

- Landing agents should install `eslint-plugin-astro-pipeline@0.1.2` before trying to satisfy the current Astro guardrails.
