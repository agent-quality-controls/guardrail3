# Summary

Replaced the overgrown Astro strict-content policy plan with a minimal enforcement policy and started implementing it. The shared `guardrail3-rs.toml` parser now exposes only the minimal `[ts.astro]` fields, Astro ingestion reads that policy through the shared parser, and config checks include `TS-ASTRO-CONFIG-23` for missing or incomplete strict content policy.

# Decisions

- Removed typed support for the old `*_globs` route taxonomy instead of adding compatibility aliases.
- Kept parser-global unknown-key preservation unchanged, but no Astro typed policy or rule consumes old field names.
- Added Astro policy facts to `G3TsAstroIntegrationContractInput` because the strict content policy is app-level Astro configuration.
- Added the first semantic rule in config checks rather than file-tree checks because it validates configuration shape, not discovered files.

# Key Files

- `.plans/2026-04-26-155340-minimal-astro-policy.md`
- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_23_strict_content_policy.rs`

# Verification

- `cargo test --workspace` in `packages/parsers/guardrail3-rs-toml-parser`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/parsers/guardrail3-rs-toml-parser`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`

# Next Steps

- Implement `TS-ASTRO-CONFIG-24` for app-relative path/glob validation.
- Implement `TS-ASTRO-CONFIG-25` by matching discovered files against `content_routes`, `non_content_routes`, and `endpoints`.
- Implement `TS-ASTRO-CONFIG-26` so ESLint plugin route coverage is checked against policy-derived route scopes.
