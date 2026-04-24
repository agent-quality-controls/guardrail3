# Release G3TS CLI And Astro Pipeline Plugin

## Goal

Install the current `g3ts` CLI from this checkout so local agents use the latest committed code, and publish a fresh npm patch release of `eslint-plugin-astro-pipeline`.

## Approach

1. Verify current registry and local versions.
2. Bump `packages/ts/eslint-plugin-astro-pipeline` from `0.1.2` to `0.1.3` because `0.1.2` is already published.
3. Run plugin tests and `npm pack --dry-run` before publishing.
4. Publish the plugin with the npm token from `.env.local`, then verify npm `latest` points at `0.1.3`.
5. Run `cargo test` for `apps/guardrail3-ts`, then `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`.
6. Write a worklog and commit the release metadata changes.

## Key Decisions

- The CLI crate is `publish = false`, so release means local installation, not crates.io publication.
- The plugin publish needs a patch version bump so npm can accept the current local artifact.

## Files To Modify

- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/package-lock.json`
- `.worklogs/<timestamp>-release-g3ts-cli-plugin.md`
