Summary
- Fixed G3TS Astro false positives for effective ESLint plugin detection and content adapter glob equivalence.
- The ESLint parser now recognizes package identity for normalized effective plugin objects without source-regex attribution, while spoofed plugin objects remain rejected.

Decisions made
- Kept setup and MDX checks strict: they still require non-ignored probes, plugin namespace, package identity, and the required error-level rule.
- Moved plugin-shape tolerance into the shared ESLint parser instead of weakening Astro family checks.
- Accepted `src/content`, `src/content/**`, and `src/content/**/*` as the same recursive adapter module contract for rule 18.
- Rejected source-text import attribution for package identity after adversarial review showed namespace-global smearing could bless fake effective plugins.

Key files for context
- packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs
- packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs
- packages/ts/astro/setup/g3ts-astro-setup-config-checks/crates/runtime/src/ts_astro_config_05_astro_eslint_plugin_wired.rs
- packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_20_mdx_lane.rs
- packages/ts/astro/content/g3ts-astro-content-config-checks/crates/runtime/src/ts_astro_config_18_content_adapter_rule.rs

Verification
- cargo test --workspace --offline --locked in apps/guardrail3-ts
- cargo test --workspace --offline --locked in packages/parsers/eslint-config-parser
- cargo test --workspace --offline --locked in setup/content/mdx Astro config-check packages
- g3rs validate on changed parser/setup/content/mdx packages with no Warn/Error findings
- cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force
- g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory with no Warn/Error findings
- Final adversarial review reported no blockers.

Next steps
- If future ESLint plugin packages expose normalized objects that fingerprint poorly, improve the parser with a syntax-aware config evaluation bridge rather than adding family-specific exceptions.
