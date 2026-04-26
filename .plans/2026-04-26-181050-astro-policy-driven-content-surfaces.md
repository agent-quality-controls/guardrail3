# Goal

Finish the policy-driven Astro/content hardening slice without forcing one exact app file layout.

# Required End State

- `guardrail3-ts.toml` owns approved Astro content architecture modules by configured paths, not hardcoded locations.
- G3TS requires the configured module surfaces to exist.
- G3TS requires ESLint to enforce the configured surfaces on the relevant source lanes.
- The ESLint plugin owns source-level import enforcement where an existing delegated tool does not exist.

# Policy Fields

Extend `[ts.astro]` with:

- `mdx_component_maps`
  - App-relative file or directory paths.
  - MDX files may import React components only from these modules.
- `metadata_helpers`
  - App-relative file or directory paths.
  - Public routes must use either approved metadata helpers or approved content adapters for route metadata.
- `json_ld_helpers`
  - App-relative file or directory paths.
  - Public routes must use approved JSON-LD helpers for structured data.

# ESLint Plugin Rules

Add these to `g3ts-eslint-plugin-astro-pipeline`:

- `astro-pipeline/mdx-component-imports-from-approved-map`
  - Applies to files matched by `mdxContentGlobs`.
  - Reports every runtime import whose resolved module path is outside `approvedMdxComponentModules`.
  - Allows imports such as `@/mdx-components` when that module is configured.
  - Blocks imports such as `@/ui/private/widget` and `../../components/chart`.
- `astro-pipeline/require-approved-metadata-helper-in-routes`
  - Applies to route files matched by `routeGlobs`, excluding `endpointGlobs`.
  - Passes when a route imports a configured `approvedMetadataHelperModules` module or a configured `approvedContentAdapterModules` module.
  - Fails when route metadata can be hardcoded without touching the approved surfaces.
- `astro-pipeline/require-approved-json-ld-helper-in-routes`
  - Applies to route files matched by `routeGlobs`, excluding `endpointGlobs`.
  - Passes only when a route imports a configured `approvedJsonLdHelperModules` module.

# G3TS Rules

Add config checks:

- `TS-ASTRO-CONFIG-29`
  - Requires strict policy to configure non-empty `mdx_component_maps`, `metadata_helpers`, and `json_ld_helpers`.
  - Requires each configured path to be app-relative and non-overlapping with `content_root`.
- `TS-ASTRO-CONFIG-30`
  - Requires every configured `mdx_component_maps` source to exist.
  - Requires ESLint MDX lane to activate `astro-pipeline/mdx-component-imports-from-approved-map` at error severity with non-empty `mdxContentGlobs` and `approvedMdxComponentModules`.
- `TS-ASTRO-CONFIG-31`
  - Requires every configured `metadata_helpers` source to exist.
  - Requires Astro, TS, and TSX lanes to activate `astro-pipeline/require-approved-metadata-helper-in-routes` at error severity with route coverage, endpoint coverage, non-empty `approvedMetadataHelperModules`, and non-empty `approvedContentAdapterModules`.
- `TS-ASTRO-CONFIG-32`
  - Requires every configured `json_ld_helpers` source to exist.
  - Requires Astro, TS, and TSX lanes to activate `astro-pipeline/require-approved-json-ld-helper-in-routes` at error severity with route coverage, endpoint coverage, and non-empty `approvedJsonLdHelperModules`.

# Files

- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/*`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/*`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/tests/*`

# Verification

- `npm test --prefix packages/ts/g3ts-eslint-plugin-astro-pipeline`
- `cargo test --workspace` in each touched Rust package.
- `g3rs validate --path` on touched Rust packages and `apps/guardrail3-ts`.
- Install local `g3ts`.
- Run G3TS on landing.
- Run adversarial test-attack agents against this plan and the code.
