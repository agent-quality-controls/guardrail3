## Summary

Retried publishing `eslint-plugin-astro-pipeline@0.1.0` with the replacement npm token. The publish did not proceed because npm rejected the token at the auth step.

## Decisions made

- Replaced `NPM_TOKEN` in root `.env.local`.
  - Reason: the user explicitly requested that storage location.
- Rechecked auth with `npm whoami` using a temporary npm user config.
  - First attempt used npmrc env substitution.
  - Second attempt wrote the literal token into the temp npm config to remove any ambiguity.
- Stopped before another publish attempt once both auth checks returned `401 Unauthorized`.
  - Reason: the failure is credential validity, not package shape.

## Key files for context

- `.plans/2026-04-24-103424-retry-astro-pipeline-plugin-publish.md`
- `.env.local`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`

## Verification

- `npm whoami` with temp npm config using `NPM_TOKEN`
  - result: `E401 Unauthorized`
- `npm whoami` with temp npm config containing the literal token
  - result: `E401 Unauthorized`

## Next steps

- Provide a working npm publish token for the account or org that should own `eslint-plugin-astro-pipeline`.
- After that, rerun:
  - `npm whoami`
  - `npm publish --access public`
  - `npm view eslint-plugin-astro-pipeline version`
