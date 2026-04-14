Summary

- Migrated the still-useful non-structural `hexarch` semantics into `apparch` instead of deleting them blind.
- Added sneaky failing regressions first, then implemented `RS-APPARCH-CONFIG-05..09` and `RS-APPARCH-SOURCE-05` with package-owned types, ingestion, and rule-local tests.
- Verified the widened `apparch` family through direct rule tests, end-to-end pipeline tests, and the new CLI app.

Decisions made

- Carried old patch/replace bypass semantics forward as waiver-tracked apparch policy instead of folding them into plain dependency-direction errors.
  - Reason: root overrides are architectural escape hatches and should stay explicit.
- Kept same-layer cycle detection scoped to non-dev edges and split forbidden dev-only direction into its own warn-level rule.
  - Reason: test-only coupling should not be conflated with runtime graph integrity.
- Re-derived old domain purity into separate `types/*` and `logic/*` purity rules driven by external dependency facts plus `guardrail3-rs.toml`.
  - Rejected alternative: reintroducing a fake hex/domain layer model inside apparch.
- Re-derived old ports-surface discipline into a `types/*` public-surface rule that flags public free functions and public inherent methods on concrete types.
  - Rejected alternative: keeping that behavior only in `hexarch` and deleting the family later.
- Added rule-local sidecar tests for every new config rule after the first adversarial pass showed the pipeline-only coverage was too weak for this repo's testing pattern.

Key files for context

- `.plans/2026-04-14-172308-apparch-migrate-hexarch-semantics.md`
- `packages/rs/apparch/g3rs-apparch-types/src/lib.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_05_patch_replace_bypass.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_07_dev_dependency_direction.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_08_types_purity.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_09_logic_purity.rs`
- `packages/rs/apparch/g3rs-apparch-source-checks/crates/runtime/src/rs_apparch_source_05_types_public_surface.rs`

Next steps

- Decide whether the remaining repo-global `hexarch` concerns should become a separate repo-global family or die with `hexarch`.
- After that decision, delete `packages/rs/hexarch` only if nothing semantically unique remains.
- Sweep repo docs once the `hexarch` deletion decision is settled so the documented architecture matches the live family set.
