# Release G3TS CLI and Astro plugin

## Summary

Published `eslint-plugin-astro-pipeline@0.1.3` to npm and installed the local `g3ts` CLI from `apps/guardrail3-ts`.

## Decisions made

- Released the plugin from the local workspace package at `packages/ts/eslint-plugin-astro-pipeline`.
- Bumped the plugin package metadata to `0.1.3` before publishing.
- Treated the CLI release as a local install because the `g3ts` runtime crate has `publish = false`.
- Used a temporary npm user config generated from `.env.local` because the default npm user config was not using the project token for this publish.

## Verification

- `npm test` passed in `packages/ts/eslint-plugin-astro-pipeline`.
- `npm pack --dry-run` passed for `eslint-plugin-astro-pipeline@0.1.3`.
- `npm view eslint-plugin-astro-pipeline version dist-tags.latest --json` reports `0.1.3` for both package version and `latest`.
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace` passed.
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force` installed `/Users/tartakovsky/.cargo/bin/g3ts`.
- `g3ts --help` prints the expected CLI commands.

## Key files for context

- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/package-lock.json`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/Cargo.toml`

## Next steps

- Use `eslint-plugin-astro-pipeline@0.1.3` in downstream Astro apps.
- Use `/Users/tartakovsky/.cargo/bin/g3ts` for local validation runs.
