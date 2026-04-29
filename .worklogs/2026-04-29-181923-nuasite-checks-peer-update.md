## Summary

Updated `g3ts-astro-nuasite-checks` to match the G3TS-required `@nuasite/checks@0.36.1` package version. Published `g3ts-astro-nuasite-checks@0.1.1` and updated G3TS Astro Syncpack policy to require the new helper version.

## Decisions made

- Fixed the owned mismatch by changing the helper peer/dev dependency from `@nuasite/checks@0.18.0` to `0.36.1`.
- Bumped the helper package to `0.1.1` because the already-published `0.1.0` package metadata cannot be changed.
- Updated the Astro setup Syncpack required pin from `g3ts-astro-nuasite-checks@0.1.0` to `0.1.1` so G3TS enforces the fixed package.
- Left the `@nuasite/checks@0.36.1` peer on `typescript@^6.0.2` untouched because that is upstream package metadata, not a G3TS package metadata bug.

## Key files for context

- `packages/ts/g3ts-astro-nuasite-checks/package.json`
- `packages/ts/g3ts-astro-nuasite-checks/package-lock.json`
- `packages/ts/g3ts-astro-nuasite-checks/tests/structured-data-present.test.ts`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`

## Verification

- `npm view @nuasite/checks@0.36.1 peerDependencies --registry=https://registry.npmjs.org/`
- `npm test` in `packages/ts/g3ts-astro-nuasite-checks`
- `npm publish --access public --registry=https://registry.npmjs.org/` in `packages/ts/g3ts-astro-nuasite-checks`
- `npm view g3ts-astro-nuasite-checks version peerDependencies --registry=https://registry.npmjs.org/`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `npm pack --dry-run --json --ignore-scripts` in `packages/ts/g3ts-astro-nuasite-checks`
- `git diff --check`

## Real app result

`g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family astro --inventory` now reports one expected app-side error: `.syncpackrc` still pins `g3ts-astro-nuasite-checks@0.1.0` and must move to `0.1.1`.
