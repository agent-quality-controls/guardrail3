# Goal
Retry publishing `eslint-plugin-astro-pipeline@0.1.0` with the replacement npm token.

# Approach
1. Replace `NPM_TOKEN` in root `.env.local`.
2. Verify npm auth with the new token.
3. Publish `packages/ts/eslint-plugin-astro-pipeline`.
4. Verify the live registry version.
5. Write a worklog for the publish attempt.

# Key Decisions
- Reuse the existing package version `0.1.0` unless the registry reports it already exists.
- Keep auth in root `.env.local` because the user explicitly requested that storage location.
- Use a temporary npm user config for publish instead of persisting auth config in repo files.

# Files To Modify
- `.env.local`
- `.plans/2026-04-24-103424-retry-astro-pipeline-plugin-publish.md`
- `.worklogs/<timestamp>-retry-astro-pipeline-plugin-publish.md`
