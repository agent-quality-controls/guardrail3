# Summary

Added `TS-ASTRO-CONFIG-26` to compare delegated Astro pipeline ESLint scopes against the minimal Astro policy. Ingestion now preserves each route-scoped Astro pipeline rule's `routeGlobs` and `endpointGlobs` per Astro, TS, and TSX lane.

# Decisions

- Replaced the old route-global assumption for this rule with policy-derived discovered file coverage.
- Used `globset` for matching both policy globs and ESLint option globs.
- Kept source AST enforcement delegated to `g3ts-eslint-plugin-astro-pipeline`; G3TS only proves the delegated plugin is scoped to the same files as `[ts.astro]`.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_26_policy_eslint_coverage.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`

# Next Steps

- Implement `TS-ASTRO-CONFIG-27` to require the configured `content_adapter` path to exist and contain adapter modules.
