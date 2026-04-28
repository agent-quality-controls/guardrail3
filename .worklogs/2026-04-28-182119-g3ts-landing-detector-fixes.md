# Summary

Fixed G3TS detector false positives found on the `landing-seo-artifact-guardrails` app branch. G3TS now recognizes effective eslint-comments suppression rules and no longer lets an unrelated `start` script parse blocker invalidate `lint`, `typecheck`, `build`, or artifact-check validation.

# Decisions Made

- Used ESLint effective plugin namespace plus the separately enforced package dependency, instead of requiring package-name fingerprinting from `eslint --print-config`.
- Replaced the setup protected-disable wildcard contract with concrete protected eslint-comments rules: `require-description`, `no-unused-disable`, and `no-restricted-disable`.
- Scoped package-script parse blockers to the script graph being checked, so Railway's `${PORT:-3001}` start contract does not break unrelated validation checks.
- Added a waiver for the MDX directive DTO because type packages intentionally expose public contract structs across ingestion and check crates.

# Key Files

- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/eslint_disable_descriptions_required.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/protected_setup_rule_disables_restricted.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-types/src/types.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/eslint_suppression/disable_descriptions_required.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/artifact_validate_scripts.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/support.rs`

# Verification

- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `g3rs validate` on touched setup, MDX, and SEO packages

# Next Steps

- The landing branch is clean under the locally installed G3TS.
- The only remaining G3RS signal in touched packages is a warning-level type-package facade import inventory item.
