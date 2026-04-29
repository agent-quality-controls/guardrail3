# Goal

Restore the Astro Nuasite contract to the TypeScript 5 compatible version.

# Bug

G3TS currently requires `@nuasite/checks@0.36.1`. That is wrong for the current Astro stack because:

- the project TypeScript floor is `5.9.3`
- `@nuasite/checks@0.18.0` peers on `typescript: ^5`
- `@nuasite/checks@0.18.1` and every newer release peers on `typescript: ^6.0.2`

The previous commit compounded the mistake by publishing `g3ts-astro-nuasite-checks@0.1.1` with peer `@nuasite/checks@0.36.1`.

# Approach

- Change G3TS Astro Syncpack required pin back to `@nuasite/checks@0.18.0`.
- Publish a corrected `g3ts-astro-nuasite-checks@0.1.2` with peer/dev dependency `@nuasite/checks@0.18.0`.
- Change G3TS Astro Syncpack required pin for `g3ts-astro-nuasite-checks` to `0.1.2`.
- Keep `g3ts-astro-nuasite-checks@0.1.1` published but do not require it anywhere.
- Do not force TypeScript 6.

# Files to modify

- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/syncpack.rs`
- `packages/ts/g3ts-astro-nuasite-checks/package.json`
- `packages/ts/g3ts-astro-nuasite-checks/package-lock.json`
- `packages/ts/g3ts-astro-nuasite-checks/tests/structured-data-present.test.ts`
- `.worklogs/<timestamp>-restore-nuasite-ts5-pin.md`

# Verification

- `npm view @nuasite/checks@0.18.0 peerDependencies --registry=https://registry.npmjs.org/`
- `npm view @nuasite/checks@0.18.1 peerDependencies --registry=https://registry.npmjs.org/`
- `npm test` in `packages/ts/g3ts-astro-nuasite-checks`
- publish `g3ts-astro-nuasite-checks@0.1.2`
- `npm view g3ts-astro-nuasite-checks@0.1.2 peerDependencies --registry=https://registry.npmjs.org/`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-ingestion/Cargo.toml --offline --locked`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `git diff --check`
