# Summary

Implemented the Astro delegation guardrail plan: renamed the Astro pipeline ESLint package to `g3ts-eslint-plugin-astro-pipeline`, added `g3ts-astro-nuasite-checks`, extended Astro parser/ingestion/config checks through `TS-ASTRO-CONFIG-22`, and installed the current `g3ts` CLI locally. Multiple adversarial passes found and drove fixes in plugin import-closure traversal, content path detection, parser mutation handling, and Astro family ownership boundaries.

NPM publication was attempted but blocked because the current root `NPM_TOKEN` is rejected by npm with `E401 Unauthorized` even through a temporary userconfig. The packages are mechanically ready to publish.

# Decisions Made

- Kept ESLint rule namespace as `astro-pipeline` while changing the npm package name to `g3ts-eslint-plugin-astro-pipeline`.
- Removed Astro family ownership of generic Syncpack package/script setup; Astro 09/10 now enforce only Astro-specific Syncpack pin/ban facts.
- Delegated rendered-output validation to `@nuasite/checks` and shared JSON-LD presence to `g3ts-astro-nuasite-checks`; G3TS checks setup, not rendered HTML.
- Hardened `astro-config-parser` to fail closed when exported config facts depend on mutated static bindings, including aliases, blocks/control flow, optional calls, spread mutating calls, callable config bodies, and var/let initializer side effects.
- Hardened the Astro ESLint plugin import closure/path analysis for URL path helpers, `process.cwd()` aliases, named `posix` path imports, dynamic template content reads, and approved adapter/loader exemptions.

# Key Files For Context

- `.plans/2026-04-25-161058-astro-delegation-boundaries.md`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/utils/ast-helpers.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/no-authored-content-imports.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/g3ts-astro-nuasite-checks/src/index.ts`

# Verification

- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `npm test` in `packages/ts/g3ts-astro-nuasite-checks`
- `npm pack --dry-run` in both npm package directories
- `cargo test -q --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/parsers/syncpack-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --check` for touched Rust parser/Astro crates
- `git diff --check`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- Final adversarial convergence pass returned PASS for plugin URL helper traversal and parser stale-binding mutation classes.

# Next Steps

- Replace or repair the npm token, then run `npm publish --access public` for `packages/ts/g3ts-eslint-plugin-astro-pipeline` and `packages/ts/g3ts-astro-nuasite-checks`.
- Hand the installed local `g3ts` CLI to the landing agent; current landing validation correctly reports the new Astro setup gaps.
