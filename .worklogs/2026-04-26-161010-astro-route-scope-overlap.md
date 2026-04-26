# Summary

Added `TS-ASTRO-CONFIG-25` to reject discovered route files that match both `content_routes` and `non_content_routes`. Astro config-check contracts now carry app-relative discovered route and endpoint paths from ingestion.

# Decisions

- Used `globset` for route matching instead of hand-rolled glob logic.
- Checked actual discovered files instead of theoretical glob-vs-glob overlap.
- Kept endpoints in the contract for the next ESLint scope rule, but `TS-ASTRO-CONFIG-25` only checks route-page overlap.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_25_route_scope_overlap.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`

# Next Steps

- Implement `TS-ASTRO-CONFIG-26` so effective ESLint plugin coverage is checked against policy-derived content route and endpoint scopes.
