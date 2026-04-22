## Summary

Hardened the `RS-APPARCH-CONFIG-06` test surface after the adversarial pass. The rule tests now use the real post-ingestion input shape, prove exact reported cycle members, and cover the clean acyclic branch; the dev-only ignore behavior is now proved at the ingestion pipeline boundary.

## Decisions made

- Rewrote the `RS-APPARCH-CONFIG-06` rule helper to accept prebound same-layer edges directly.
  - Why: the old helper was recreating ingestion behavior by filtering dependency kinds and matching crates from raw edge payloads. That duplicated the very boundary we just repaired.
- Tightened the rule assertions to prove exact message shape for cycle errors and exact inventory output for the acyclic branch.
  - Why: the previous assertions only checked the rule ID plus loose substrings, which would not catch partial or wrong member attribution.
- Moved the dev-only ignore assertion into the ingestion assertions crate.
  - Why: the first attempt put direct `CheckResult` field checks into a sidecar test, and `g3rs validate` correctly flagged that as `RS-TEST-SOURCE-16`.

## Key files for context

- [rs_apparch_config_06_same_layer_cycles.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs)
- [rs_apparch_config_06_same_layer_cycles_tests/helpers.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/helpers.rs)
- [rs_apparch_config_06_same_layer_cycles_tests/cases.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/cases.rs)
- [crates/assertions/src/rs_apparch_config_06_same_layer_cycles.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/assertions/src/rs_apparch_config_06_same_layer_cycles.rs)
- [config_tests/pipeline.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config_tests/pipeline.rs)
- [crates/assertions/src/run/config.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-ingestion/crates/assertions/src/run/config.rs)

## Next steps

- Continue only if another adversarial pass finds a real remaining gap in `RS-APPARCH-CONFIG-06`.
- Keep rule tests on prebound family inputs. Put ingestion-owned filtering behavior in pipeline tests, not rule helpers.
