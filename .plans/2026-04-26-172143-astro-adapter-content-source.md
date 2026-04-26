# Goal

Make G3TS reject Astro `content_adapter` folders that only contain arbitrary TypeScript helpers.

# Rule

Add `TS-ASTRO-CONFIG-28`.

For a strict Astro content app:

- `guardrail3-ts.toml [ts.astro].content_adapter` selects the adapter source path.
- Every included runtime source file under that path must be a real Astro collection adapter module.
- A real Astro collection adapter module has a runtime import from `astro:content`, for example `import { getEntry } from "astro:content"`.
- Type-only imports from `astro:content` do not satisfy the rule.

# Why This Belongs To Astro

This is Astro-specific because `astro:content` is Astro's collection API.
The framework-independent content family must not know this module exists.

# Implementation

- Extend `G3TsAstroIntegrationContractInput` with `content_adapter_astro_content_source_paths`.
- In Astro ingestion, parse each adapter source module with SWC and collect adapter files that have a runtime `astro:content` import/export.
- Keep `TS-ASTRO-CONFIG-27` as the existence check.
- Add `TS-ASTRO-CONFIG-28` as the semantic check that all adapter source files import `astro:content`.
- Add tests proving:
  - valid adapter source with runtime `astro:content` passes.
  - adapter source without `astro:content` fails.
  - `import type` from `astro:content` alone fails.

# Files

- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/Cargo.toml`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_28_content_adapter_astro_content.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in Astro types, ingestion, and config checks.
- `g3rs validate --path` for touched packages.
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`.
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`.
