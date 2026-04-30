# ESLint ESM plugin resolution

## Summary

Fixed G3TS ESLint plugin package identity detection for ESM-only plugin packages. The shared ESLint config parser now imports candidate plugin packages through CommonJS resolution and ESM package import resolution before comparing them to the effective ESLint plugin object.

## Decisions made

- Fixed the shared parser because the bug affects any delegated ESLint plugin, not only `eslint-plugin-tailwind-ban`.
- Added a regression test with an ESM-only `eslint-plugin-tailwind-ban` package using `exports.import`, proving the old resolver missed a package that ESLint itself can load.
- Kept the existing object/fingerprint comparison gate so namespace or package name alone still cannot prove plugin identity.
- Ran the Node helper from the ESLint config directory so ESM package imports resolve relative to the app config location.

## Key files for context

- `.plans/2026-04-30-212657-eslint-esm-plugin-resolution.md`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`

## Verification

- `cargo test --manifest-path packages/parsers/eslint-config-parser/crates/runtime/Cargo.toml resolves_plugin_package_identity_from_esm_only_import_export --offline`
- `cargo test --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/parsers/eslint-config-parser`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-landing/apps/landing --family style --inventory`

## Next steps

- The landing branch can rerun the merge hook with the locally installed G3TS.
