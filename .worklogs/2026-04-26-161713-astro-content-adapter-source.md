# Summary

Added `TS-ASTRO-CONFIG-27` to require the configured Astro `content_adapter` path to resolve to included adapter source. Ingestion now derives app-relative adapter source paths from the parsed `[ts.astro]` policy and workspace crawl.

# Decisions

- Stored adapter source paths on the config contract so the rule stays pure.
- Treated `content_adapter` as either a module path or a directory prefix.
- Limited adapter source detection to source file extensions used by Astro/TS apps.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_27_content_adapter_exists.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`

# Next Steps

- Implement `TS-ASTRO-FILETREE-12` using the configured `forbidden_state` patterns instead of hard-coded legacy state discovery only.
