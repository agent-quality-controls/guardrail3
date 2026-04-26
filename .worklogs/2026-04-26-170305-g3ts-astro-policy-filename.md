# Summary

Fixed the G3TS Astro strict content policy filename from `guardrail3-rs.toml` to `guardrail3-ts.toml`. The shared TOML parser package remains reused internally, but the app-facing TypeScript/Astro contract no longer exposes the Rust filename.

# Decisions

- Changed Astro ingestion to look for app-local `guardrail3-ts.toml`.
- Updated config-check messages and tests to tell app agents to create `guardrail3-ts.toml`.
- Added a regression test proving `guardrail3-rs.toml` is ignored for Astro app policy, so this does not drift back.
- Did not rename `guardrail3-rs-toml-parser`; that package is shared by active Rust checks and parser package renaming is separate scope.

# Key Files

- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_23_strict_content_policy.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.plans/2026-04-26-170111-g3ts-astro-policy-filename.md`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Next Steps

- Landing agent should add `apps/landing/guardrail3-ts.toml`, not `guardrail3-rs.toml`.
