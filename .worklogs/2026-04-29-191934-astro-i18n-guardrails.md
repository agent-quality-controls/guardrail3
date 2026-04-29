Summary
- Added the Astro i18n guardrail slice and published `g3ts-eslint-plugin-astro-i18n-policy@0.1.0`.
- Added G3TS `astro-i18n` family packages for types, ingestion, config checks, and hook contract, then wired the family into the CLI and Astro aggregate selection.
- Local `g3ts` was reinstalled from `apps/guardrail3-ts`.

Decisions made
- I18n source checks are delegated to ESLint: `eslint-plugin-i18next`, `g3ts-eslint-plugin-astro-i18n-policy`, `no-restricted-syntax`, and `@eslint-community/eslint-plugin-eslint-comments`.
- G3TS owns wiring enforcement only: package presence, TOML policy presence, effective ESLint lanes, rule severity, options matching, protected disable restrictions, and hook contract.
- Rule options must match `[ts.astro.i18n]`; "rule has some options" is not sufficient because it lets ESLint drift away from the guardrail policy.
- The new i18n Rust packages carry package-local Rust guardrail configs so G3RS can validate them directly.

Key files for context
- `.plans/2026-04-29-184826-astro-i18n-implementation.md`
- `packages/ts/g3ts-eslint-plugin-astro-i18n-policy/src/index.ts`
- `packages/ts/astro/i18n/g3ts-astro-i18n-types/src/types.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/eslint.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract/src/contract.rs`
- `apps/guardrail3-ts/crates/types/app-types/src/supported_family.rs`

Verification
- `npm test` in `packages/ts/g3ts-eslint-plugin-astro-i18n-policy`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-types/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `g3rs validate --path apps/guardrail3-ts --inventory`
- `g3rs validate --path packages/ts/astro/i18n/g3ts-astro-i18n-config-checks --inventory`
- `g3rs validate --path packages/ts/astro/i18n/g3ts-astro-i18n-ingestion --inventory`
- `g3rs validate --path packages/ts/astro/i18n/g3ts-astro-i18n-types --inventory`
- `g3rs validate --path packages/ts/astro/i18n/g3ts-astro-i18n-hook-contract --inventory`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher-integrate-static-railway/apps/landing --family astro --inventory`
- `git diff --check`

Next steps
- Landing must add `[ts.astro.i18n]`, pin `g3ts-eslint-plugin-astro-i18n-policy@0.1.0` in Syncpack, wire the new ESLint namespace and rules, and add raw date/number formatting bans.
- After landing is updated, rerun `g3ts validate --path apps/landing --family astro --inventory` in that worktree.
