# Goal
Save the npm token in root `.env.local` and publish `eslint-plugin-astro-pipeline@0.1.0` to npm.

# Approach
1. Append `NPM_TOKEN` to root `.env.local` without exposing the token in command output.
2. Publish from `packages/ts/eslint-plugin-astro-pipeline` using a temporary npm user config that reads `${NPM_TOKEN}` from the environment.
3. Verify publish success with `npm view eslint-plugin-astro-pipeline version`.
4. Write a worklog and commit only the plan/worklog if code does not change.

# Key Decisions
- Keep the npm token in `.env.local` because the user explicitly requested that location.
- Use a temporary `NPM_CONFIG_USERCONFIG` file instead of writing auth into repo files.
- Do not change the package version unless publish fails because `0.1.0` already exists.

# Files To Modify
- `.env.local`
- `.plans/2026-04-24-102939-publish-astro-pipeline-plugin.md`
- `.worklogs/<timestamp>-publish-astro-pipeline-plugin.md`
