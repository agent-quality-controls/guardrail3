# Astro auditor and i18n plan audit

## Summary

Planned and audited the remaining Astro generator/auditor and i18n guardrail slice. The implementation was already present in the current tree, so no code changes were made; verification confirms the existing G3TS rules enforce the planned contracts.

## Decisions made

- Did not edit already-correct code. The SEO family already requires sitemap/robots/llms generator/auditor wiring, Nuasite fail-closed config, structured-data presence through Nuasite custom checks, and artifact package presence.
- Did not add artifact validation to Rust G3TS. The existing code keeps sitemap, robots, llms, and rendered-output validation delegated to Astro integrations and Nuasite.
- Did not add more i18n source parsing in Rust. The i18n family already verifies delegated ESLint wiring for `i18next/no-literal-string`, `astro-i18n-policy/no-unlocalized-internal-hrefs`, raw date/number `no-restricted-syntax` bans, helper exceptions, and protected disables.
- Treated landing failures as expected app-side configuration gaps. Landing already passes the new SEO artifact integration checks but still needs stale ESLint/i18n/media setup work.

## Key files for context

- `.plans/2026-04-30-122459-astro-auditor-and-i18n-finish.md`
- `.plans/2026-04-29-122938-astro-auditor-generator-architecture.md`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/sitemap_integration.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/robots_integration.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/llms_integration_present.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/nuasite_checks.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/structured_data_check.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/forbidden_script_targets.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/crates/runtime/src/rule_wiring.rs`
- `packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/src/eslint/settings.rs`

## Verification

- `npm test` in `packages/ts/astro/sitemap/g3ts-astro-sitemap-auditor`
- `npm test` in `packages/ts/astro/robots/g3ts-astro-robots-auditor`
- `npm test` in `packages/ts/astro/llms/g3ts-astro-llms-auditor`
- `npm test` in `packages/ts/astro/integrations/g3ts-astro-llms-generator`
- `cargo test --manifest-path packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-config-checks/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path packages/ts/astro/i18n/g3ts-astro-i18n-ingestion/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate` on SEO setup/i18n packages
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

## Next steps

- Landing should update its stale ESLint/i18n/media config based on current G3TS errors.
- If the next guardrail slice is needed, pick a new rule family; the planned SEO artifact and i18n delegated-rule slice is already implemented.
