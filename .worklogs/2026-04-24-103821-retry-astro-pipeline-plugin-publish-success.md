## Summary

Published `eslint-plugin-astro-pipeline@0.1.0` to npm with the replacement token. The earlier auth failures were caused by npm treating `/Users/tartakovsky/.npmrc` as a project config because the repo lives under the home directory, which overrode the intended auth path.

## Decisions made

- Kept the token in root `.env.local`.
  - Reason: the user explicitly requested that location.
- Isolated publish/auth commands from `/Users/tartakovsky/.npmrc`.
  - Ran npm from `/tmp` with a temporary `HOME` and temp `.npmrc`.
  - Reason: npm was treating `/Users/tartakovsky/.npmrc` as a project config when commands ran inside the repo tree.
- Verified the package with the same clean auth path after publish.
  - Reason: plain `npm view` from the repo tree was still polluted by the local npm config surface.

## Key files for context

- `.plans/2026-04-24-103424-retry-astro-pipeline-plugin-publish.md`
- `.env.local`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`

## Verification

- Clean auth check from `/tmp` with temp `HOME` and temp `.npmrc`
  - `npm whoami` -> `tartakovsky`
- Publish from `/tmp`
  - `npm publish --access public /Users/tartakovsky/Projects/websmasher/guardrail3/packages/ts/eslint-plugin-astro-pipeline`
  - result: `+ eslint-plugin-astro-pipeline@0.1.0`
- Clean registry lookup from `/tmp` with temp `HOME` and temp `.npmrc`
  - `npm view eslint-plugin-astro-pipeline name version`
  - result:
    - `name = 'eslint-plugin-astro-pipeline'`
    - `version = '0.1.0'`

## Next steps

- Real Astro apps can now install:
  - `pnpm add -D eslint-plugin-astro-pipeline`
- The next follow-up is updating the real Astro app setup to use the published package instead of a local path or workspace install.
