# Summary

Released `g3ts-eslint-plugin-astro-pipeline@0.1.6` to npm and reinstalled the local `g3ts` CLI from the current workspace. Updated the plugin README so future agents use the token-safe temporary npm userconfig flow instead of assuming npm reads `.env.local`.

# Decisions

- Published `0.1.6` because npm latest was `0.1.5` while the committed Astro guardrails require `0.1.6`.
- Used a temporary `.npmrc` populated from root `.env.local`.
  - Why: default `npm publish` was unauthenticated because npm does not source `.env.local`.
  - Rejected: writing the token into a persistent repo or user npm config.
- Kept the CLI local.
  - Why: the accepted workflow here is `cargo install --path ... --force`.

# Key Files

- `packages/ts/g3ts-eslint-plugin-astro-pipeline/package.json`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/README.md`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime`

# Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `npm --userconfig "$TMP_NPMRC" whoami`
- `npm --userconfig "$TMP_NPMRC" publish --access public`
- `npm view g3ts-eslint-plugin-astro-pipeline version dist-tags.latest --json`
  - `version = 0.1.6`
  - `dist-tags.latest = 0.1.6`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`

# Next Steps

- Landing agents can now install `g3ts-eslint-plugin-astro-pipeline@0.1.6`.
- Landing should rerun `g3ts validate --path apps/landing --family astro --inventory` and fix the reported Astro findings from the messages.
