# G3TS hook contract implementation

## Goal

Implement the TypeScript hook-contract architecture from `.plans/2026-04-28-185643-g3ts-hooks-family.md` in the same ownership style as the Rust hook contracts.

## Scope

- Add TypeScript hook contract types.
- Add family-owned Astro hook-contract packages for setup, content, mdx, and seo.
- Add TS hooks family packages: types, ingestion, file-tree checks, source checks, config checks.
- Add a TS hooks runner that aggregates family hook contracts and injects them into hook checks.
- Wire `g3ts validate --family hooks`.
- Keep checks parser-backed through `hook-shell-parser`.
- Verify on the real landing app.

## First-slice rules

- `g3ts-hooks/pre-commit-exists`
- `g3ts-hooks/hooks-path-configured`
- `g3ts-hooks/pre-commit-executable`
- `g3ts-hooks/modular-directory-inventory`
- `g3ts-hooks/modular-scripts-inventory`
- `g3ts-hooks/local-override-inventory`
- `g3ts-hooks/script-stats-inventory`
- `g3ts-hooks/pre-commit-file-size-inventory`
- `g3ts-hooks/g3ts-binary-available`
- `g3ts-hooks/g3ts-validate-staged-present`
- `g3ts-hooks/ts-app-validate-step-present`
- `g3ts-hooks/ts-guardrail-config-changes-trigger-validation`
- `g3ts-hooks/no-fail-open-wrappers`

## Contract ownership

- `g3ts-astro-setup-hook-contract` owns Astro setup trigger patterns and requires app validate.
- `g3ts-astro-content-hook-contract` owns Astro content trigger patterns and requires app validate.
- `g3ts-astro-mdx-hook-contract` owns Astro MDX trigger patterns and requires app validate.
- `g3ts-astro-seo-hook-contract` owns Astro SEO trigger patterns and requires app validate.
- Hooks packages only consume contracts and parsed hook facts.

## Files to modify

- `apps/guardrail3-ts/Cargo.toml`
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/logic/family-runner-hooks/**`
- `packages/ts/hooks/**`
- `packages/ts/astro/**/g3ts-*-hook-contract/**`

## Verification

- `cargo fmt --manifest-path apps/guardrail3-ts/Cargo.toml --all`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo clippy --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `g3rs validate --path apps/guardrail3-ts`
- install local `g3ts`
- run `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family hooks --inventory`
