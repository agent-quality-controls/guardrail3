# Summary

Added `TS-ASTRO-CONFIG-28` so strict Astro apps cannot satisfy `content_adapter` with arbitrary TypeScript files. G3TS now verifies every source file under the configured adapter path imports `astro:content` at runtime.

# Decisions

- Kept `TS-ASTRO-CONFIG-27` as the adapter-source existence check.
- Added a separate semantic rule, `TS-ASTRO-CONFIG-28`, for runtime Astro collection usage.
- Reused the SWC-backed Astro parser package to detect runtime source imports instead of adding SWC directly to Astro ingestion.
- Rejected type-only `astro:content` imports as insufficient because they do not prove the adapter reads Astro content collections.

# Key Files

- `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_28_content_adapter_astro_content.rs`
- `.plans/2026-04-26-172143-astro-adapter-content-source.md`

# Verification

- `cargo test --workspace` in `packages/parsers/astro-config-parser`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-config-checks`
- `g3rs validate --path packages/parsers/astro-config-parser`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-config-checks`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Next Steps

- Landing now fails `TS-ASTRO-CONFIG-28` because `guardrail3-ts.toml` sets `content_adapter = "src/content"` and that folder includes `src/content/schemas.ts`, which does not import `astro:content` at runtime.
