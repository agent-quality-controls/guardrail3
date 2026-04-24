## Summary

Closed the remaining real Astro hardening gaps from the latest attack pass. The Astro slice now catches direct `*.data.*` route imports, treats route/endpoint coverage as universal across real discovered Astro routes and endpoints, detects nested Velite surfaces under the app root without false-positiving on route basenames, and has lane-specific regression tests for `TS-ASTRO-CONFIG-07`.

## Decisions made

- Fixed `no-content-data-modules-in-routes` at the rule root.
  - Removed the import-chain length blind spot instead of widening globs or adding special-case reporting.
- Tightened `TS-ASTRO-CONFIG-07` at ingestion-time facts.
  - Route-scoped, content-data, and authored-content rules only count as effective when configured globs cover all real Astro page routes and all real Astro endpoints present in the app.
  - Rejected the earlier existential matching because one matched route could hide uncovered routes in the same app.
- Kept nested Velite detection recursive, but excluded `src/pages/**` from `velite.config.*` file discovery.
  - Reason: route basename alone is not enough to identify a Velite config surface.
  - Rejected restoring the broader basename-only rule because it reintroduced a concrete false positive on `src/pages/velite.config.ts`.
- Expanded tests instead of only changing implementation.
  - Added coverage for direct endpoint imports, partial route coverage, partial endpoint coverage, Astro-only lane failure, TS-only lane failure, TSX-only lane failure, and route-file false positives.

## Key files for context

- `.plans/2026-04-24-163649-fix-astro-hardening-gaps.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-content-data-modules-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-velite-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-content-data-modules-in-routes.test.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/no-velite-imports.test.ts`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/Cargo.toml`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_07_pipeline_plugin_wired.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/crates/runtime/Cargo.toml`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
  - expected result: landing still fails on real Velite usage and missing Astro-native content wiring, but no longer because of the guardrail bugs fixed in this pass

## Adversarial review

- First pass found and forced fixes for:
  - direct `*.data.*` import blind spot
  - route-file basename false positive in `no-velite-imports`
  - recursive Velite discovery gap
  - existential route/endpoint coverage in `TS-ASTRO-CONFIG-07`
  - missing partial-route, partial-endpoint, and per-lane regression tests
- Final convergence pass result:
  - `No concrete findings.`

## Next steps

- If Astro work continues, the next real missing contract is inline hardcoded content literals in route/component code. That is a separate policy surface from the bypass/coverage fixes in this pass.
