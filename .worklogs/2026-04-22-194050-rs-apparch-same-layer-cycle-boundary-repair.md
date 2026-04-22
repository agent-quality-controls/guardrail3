## Summary

Fixed the remaining `rs/apparch` config-boundary bug in same-layer cycle detection. The cycle rule no longer depends on a separate crate bag that can silently drop one side of a real cycle.

## Decisions made

- Added a red test that proved a real failure mode: when the first sorted node in a same-layer cycle was missing from the old `crates` bag, the rule emitted no finding.
- Moved crate identity binding for same-layer cycle edges fully into ingestion by changing `G3RsApparchSameLayerDependencyEdge` to carry `from` and `to` crates directly.
- Kept the cycle walk itself in the check package. The fix was to remove local rebinding from a lossy side bag, not to move graph traversal out of the rule.
- Updated the affected helper constructors to the new input shape so the package test surface stayed consistent with the repaired contract.

## Key files for context

- [types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-types/src/types.rs)
- [config.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs)
- [rs_apparch_config_06_same_layer_cycles.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles.rs)
- [cases.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_06_same_layer_cycles_tests/cases.rs)

## Next steps

- Continue the remaining Rust boundary audit from the confirmed small set of residual local-rebinding cases.
- Do not apply the `fmt`-style config slicing mistake to other config families; keep parsed config surfaces intact where the family subject is the config itself.
