## Summary

Restored the Astro Nuasite package contract to the TypeScript 5 compatible version. G3TS now requires `@nuasite/checks@0.18.0` and `g3ts-astro-nuasite-checks@0.1.2`.

## Decisions made

- Corrected the previous mistake where G3TS was moved to `@nuasite/checks@0.36.1`; that release peers on `typescript@^6.0.2` and conflicts with the current `typescript@5.9.3` floor.
- Published `g3ts-astro-nuasite-checks@0.1.2` because `0.1.1` had already been published with the wrong peer metadata.
- Kept `0.1.1` unrequired. npm package metadata cannot be edited in place.

## Key files for context

- `.plans/2026-04-29-183026-restore-nuasite-ts5-pin.md`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `packages/ts/g3ts-astro-nuasite-checks/package.json`
- `packages/ts/g3ts-astro-nuasite-checks/package-lock.json`
- `packages/ts/g3ts-astro-nuasite-checks/tests/structured-data-present.test.ts`

## Verification

- `npm view @nuasite/checks@0.18.0 peerDependencies --registry=https://registry.npmjs.org/`
- `npm view @nuasite/checks@0.18.1 peerDependencies --registry=https://registry.npmjs.org/`
- `npm test` in `packages/ts/g3ts-astro-nuasite-checks`
- `npm publish --access public --registry=https://registry.npmjs.org/` in `packages/ts/g3ts-astro-nuasite-checks`
- `npm view g3ts-astro-nuasite-checks@0.1.2 version peerDependencies --registry=https://registry.npmjs.org/`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `git diff --check`

## Real app result

`g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family astro --inventory` now only reports the app's stale Syncpack pins for `@nuasite/checks` and `g3ts-astro-nuasite-checks`. The required versions are `0.18.0` and `0.1.2`.
