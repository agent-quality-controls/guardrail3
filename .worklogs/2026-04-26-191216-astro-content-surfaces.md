# Summary

Implemented Astro strict content helper surfaces for G3TS and the Astro pipeline ESLint plugin. G3TS now requires configured MDX component-map, metadata helper, and JSON-LD helper surfaces, verifies those surfaces exist, and verifies ESLint delegates enforcement to the published `g3ts-eslint-plugin-astro-pipeline` rules.

# Decisions

- Added policy fields to `guardrail3-ts.toml` parsing instead of inventing per-app filenames, because the Astro app owns configured surfaces.
- Kept source-AST validation in `g3ts-eslint-plugin-astro-pipeline`; G3TS checks package/config/rule effectiveness and does not duplicate ESLint parsing.
- Made helper-surface ESLint options compare against configured policy paths, not discovered files, so directory surfaces remain policy-driven.
- Made MDX component imports fail closed for unresolved dynamic import and require calls.
- Narrowed route metadata/JSON-LD helper acceptance after adversarial review: unused imports, void usage, discarded calls, assigned-unused calls, noop object properties, and unrelated exported functions do not satisfy the rule.
- Left `g3rs validate --path packages/ts/g3ts-eslint-plugin-astro-pipeline` out of the clean set because that package is a TypeScript npm package without Rust guardrail files; Rust guardrails are clean on the touched Rust/G3TS packages.

# Key Files

- `.plans/2026-04-26-181050-astro-policy-driven-content-surfaces.md`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_29_policy_helper_surfaces.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_30_mdx_component_map_rule.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_31_metadata_helper_rule.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_32_json_ld_helper_rule.rs`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-component-imports-from-approved-map.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/require-approved-metadata-helper-in-routes.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/require-approved-json-ld-helper-in-routes.ts`

# Verification

- `npm test --prefix packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `cargo test --workspace` in `packages/parsers/guardrail3-rs-toml-parser`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path packages/parsers/guardrail3-rs-toml-parser`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path apps/landing --family astro --inventory` from the websmasher app repo; expected landing failures are `TS-ASTRO-CONFIG-09`, `23`, `29`, `30`, `31`, and `32`.
- Adversarial agents iterated until the final pass returned no MUST FIX findings.

# Next Steps

- Release `g3ts-eslint-plugin-astro-pipeline@0.1.6` before landing can satisfy `TS-ASTRO-CONFIG-09`.
- Landing must add `[ts.astro].mdx_component_maps`, `metadata_helpers`, `json_ld_helpers`, and wire the three new ESLint rules with those configured surfaces.
