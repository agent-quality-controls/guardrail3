## Goal

Build ts/apparch wave 2 purity checks that mirror live Rust apparch purity intent for Next apps.

## Approach

1. Read live Rust purity rules and current ts/apparch implementation.
2. Inspect real app imports under src/types and src/logic to choose a TS-appropriate purity contract.
3. Add tests first for each new purity rule and any helper behavior.
4. Extend ts/apparch types or config inputs only if needed, keeping the boundary minimal.
5. Implement purity rules in config-checks using existing external-import facts from ingestion.
6. Wire tests, fmt, g3rs validate, g3ts smoke, then run an adversarial review.

## Key decisions

- Mirror Rust purity intent, not Rust dependency mechanics.
- Keep purity in config lane because it is a dependency/import policy.
- Avoid widening to ui/lib or topology.

## Files to modify

- packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/run.rs
- packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/support.rs
- new purity rule files under packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src
- packages/ts/apparch/g3ts-apparch-config-checks/crates/runtime/src/run_tests/cases.rs
- optional assertion helpers if exact proof needs strengthening
