## Summary

Hardened the Astro pipeline contract again by closing the authored/spec effectivity hole in `TS-ASTRO-CONFIG-07` and adding `astro-pipeline/no-authored-content-imports`. The Astro family now fails closed if the authored-content rules are enabled without non-empty `authoredContentGlobs` or `specContentGlobs`, and route closures now fail on direct raw content imports.

## Decisions made

- Fixed the existing green-but-inert contract before treating new policy as complete.
  - `no-authored-content-fs-read` and `no-authored-content-glob` were previously counted as effective without authored/spec globs.
  - Rejected leaving that gap because a green `TS-ASTRO-CONFIG-07` would still be lying.
- Added `no-authored-content-imports` in the ESLint plugin.
  - It fits the existing import-closure architecture cleanly because the rule only needs static import source strings from closure modules.
  - Rejected adding any parser-special-case for raw content files themselves.
- Kept the new raw import rule on the same fail-closed pattern.
  - It is now required by `TS-ASTRO-CONFIG-07`.
  - It also requires the authored/spec content option surface to be non-empty before the contract reports green.

## Key files for context

- `.plans/2026-04-24-161713-astro-pipeline-effectiveness-hardening.md`
- `.worklogs/2026-04-24-161000-astro-native-content-contract.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-authored-content-imports.test.ts`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`

## Adversarial review

- Found and fixed:
  - `TS-ASTRO-CONFIG-07` still treated authored/spec rules as effective without authored/spec globs
  - test helpers missed the new route-scoped rule and content-source effectivity fields
- Final readback found no remaining concrete gap in this slice against the plan.

## Next steps

- Attach the new raw-content-import rule to a real Astro app and confirm the diagnostics stay specific on real violations.
- Decide whether the next hardening slice should target inline route/component content literals, or whether that needs a separate architectural threshold policy first.
