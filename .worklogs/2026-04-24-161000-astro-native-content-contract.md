## Summary

Implemented the Astro native-content enforcement slice across the ESLint plugin and `ts/astro`. Astro apps now fail closed against Velite package/filetree usage and route closures that source public-page copy from ad hoc data modules.

## Decisions made

- Kept the contract plugin-first.
  - Source-policy lives in `eslint-plugin-astro-pipeline`.
  - `ts/astro` only enforces package, filetree, and required-rule wiring.
- Added two new plugin rules instead of a broad route-adapter heuristic.
  - `no-content-data-modules-in-routes`
  - `no-velite-imports`
  - Rejected a fuzzy "every public page must import an adapter" rule because it would be noisy and under-specified.
- Added Astro-side fail-closed rules at three separate surfaces.
  - `TS-ASTRO-CONFIG-04` bans `velite` in `package.json`
  - `TS-ASTRO-FILETREE-05` bans `velite.config.*`
  - `TS-ASTRO-FILETREE-06` bans `.velite/**`
- Fixed the inert-rule hole in `TS-ASTRO-CONFIG-07`.
  - The new content-data rule now requires non-empty `contentDataModuleGlobs`, not just route scope options.
  - Rejected leaving it as a best-effort rule because it would have reported green while doing nothing.

## Key files for context

- `.plans/2026-04-24-153434-astro-native-content-contract.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-content-data-modules-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-velite-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/options.ts`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_04_no_velite_package.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_05_no_velite_config.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_06_no_velite_output.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`

## Adversarial review

- Found and fixed:
  - false positive in `no-content-data-modules-in-routes` from overlap semantics
  - stale recommended-config expectation
  - stale `TS-ASTRO-CONFIG-07` messages/tests after the new rules landed
  - missing `contentDataModuleGlobs` effectiveness enforcement for `TS-ASTRO-CONFIG-07`
- Final pass found no remaining concrete gap against the plan implemented in this slice.

## Next steps

- Attach this contract to a real Astro app and verify the rule messages are usable on live violations.
- Decide whether the next Astro-native rule should ban route-local raw JSON/MDX content imports directly, or whether the current data-module and Velite bans are enough for the next app rollout.
