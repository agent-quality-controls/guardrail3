# Goal

Fix the G3TS Astro strict content policy contract so external TypeScript/Astro apps use `guardrail3-ts.toml`, not `guardrail3-rs.toml`.

# Problem

The new Astro policy rule reused `guardrail3-rs.toml` as the app-facing filename because the existing shared TOML parser is named `guardrail3-rs-toml-parser`.
That leaked an internal parser/package name into the TypeScript app contract.

# Approach

- Keep the shared parser package for now because it already parses `[ts.astro]` and renaming the parser package would touch active Rust guardrail packages.
- Change G3TS Astro ingestion to select app-local `guardrail3-ts.toml`.
- Update Astro config-check messages and tests to reference `guardrail3-ts.toml`.
- Add/adjust tests so `guardrail3-ts.toml` is the accepted app policy file.
- Update current Astro/content plans so implementation agents add the right file to apps.
- Reinstall local `g3ts` after the change.

# Files To Modify

- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_23_strict_content_policy.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_24_strict_policy_paths.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_25_route_scope_overlap.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_26_policy_eslint_coverage.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `.plans/2026-04-26-155340-minimal-astro-policy.md`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
