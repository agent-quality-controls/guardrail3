Goal

Harden the `RS-APPARCH-CONFIG-06` test surface so it proves exact member attribution, tests the clean acyclic branch, and stops recreating ingestion behavior inside rule-test helpers.

Approach

- Rewrite the `rs_apparch_config_06_same_layer_cycles` helper to construct the post-ingestion input directly from prebound same-layer edges, not from raw crate and dependency bags.
- Tighten the assertions crate for `RS-APPARCH-CONFIG-06` so tests can prove exact member display in error messages and exact inventory text in the clean branch.
- Update the rule tests to:
  - assert exact cycle members for the error path
  - assert exact self-loop attribution
  - assert the inventory message for an acyclic same-layer graph
- Move the dev-only ignore behavior to an ingestion pipeline test, where dev-edge filtering actually belongs.

Key decisions

- Keep dev-edge filtering out of the rule tests.
  - Reason: the rule input no longer carries dependency kinds; dev-only suppression is ingestion-owned behavior.
- Harden assertions instead of adding looser ad hoc checks in each test.
  - Reason: this keeps the rule test surface consistent with the rest of the family.
- Do not change the production rule unless a real runtime bug appears.
  - Reason: the attack findings are test-surface weaknesses, not a confirmed new check bug.

Files to modify

- `packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_06_same_layer_cycles.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/helpers.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config_tests/pipeline.rs`
