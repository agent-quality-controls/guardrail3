# Summary

Implemented the remaining Astro suppression and validate-script guardrails. The Astro setup/content/MDX/SEO families now enforce eslint-comments wiring, protected delegated-rule disable restrictions, parser-backed disable inventory, and one standard fail-closed `validate` script.

# Decisions made

- Kept directive inventory because hidden `eslint-disable` comments are escape hatches and must be visible.
- Used the shared `eslint-directive-parser` in ingestion instead of raw text scanning in check rules.
- Made directive inventory fail closed on unreadable, unsupported, parse-error, or ambiguous parser states.
- Kept validate-script checking in Astro setup because it is the app-level Astro validation contract; SEO still owns only generated artifact checker ordering and arguments.
- Updated the plan to state the parser-backed inventory contract explicitly.

# Key files for context

- `.plans/2026-04-28-140349-astro-suppression-and-validate-contract.md`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/validate_script.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/protected_setup_rule_disables_restricted.rs`
- `packages/ts/astro/content/g3ts-astro-content-config-checks/crates/runtime/src/eslint_disable_inventory.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/eslint_suppression`
- `packages/ts/astro/seo/g3ts-astro-seo-config-checks/crates/runtime/src/eslint_disable_inventory.rs`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/eslint_directives.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint_directives.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/eslint_directives.rs`

# Verification

- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline --locked`
- `g3rs validate` on touched Astro setup/content/MDX/SEO config-check and ingestion packages
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- Adversarial review `019dd466-59e9-7973-ad95-81025343a19b`: PASS, no MUST FIX blockers

# Next steps

- Add ingestion-level tests for directive parser fail-closed paths when that test layer is expanded.
- Landing app should now install/configure eslint-comments, Syncpack pins, validate script, crawler checker packages, and trailingSlash policy until G3TS is clean.
