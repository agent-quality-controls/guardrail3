# RS-TEST Direct Component Shape Only

**Date:** 2026-03-26 11:24
**Scope:** `apps/guardrail3/crates/app/rs/families/test/README.md`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs`, runtime rule fixtures under `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src`

## Summary
Removed support for the old nested `crates/<name>/{runtime,assertions}` option from the `rs/test` family validator and converted the family’s own runtime fixtures to the direct `crates/runtime` + `crates/assertions` shape only. The README now matches that stricter interpretation.

## Context & Problem
After flattening the family’s self-hosting layout, the validator still supported both the new direct shape and the old nested component shape. The user explicitly rejected that compromise: the old option was considered wrong, not merely legacy. Leaving both paths in place would have preserved the wrong model in code and in the rule fixtures.

## Decisions Made

### Make discovery direct-only for this family
- **Chose:** `collect_components(...)` now recognizes only `crates/runtime` plus sibling `crates/assertions`.
- **Why:** This matches the user’s required structure exactly and prevents the validator from silently blessing the old nested option.
- **Alternatives considered:**
  - Keep both direct and nested discovery paths — rejected because it preserves the disallowed old model.
  - Keep direct-only for self-hosting but nested for fixtures through a hidden special case — rejected because it would still encode two models in the validator.

### Rewrite the family fixtures to prove only the direct shape
- **Chose:** Update runtime rule fixtures from `crates/demo/...` to direct `crates/...`.
- **Why:** The rule suite should test the actual supported shape, not a shape the validator no longer accepts.
- **Alternatives considered:**
  - Leave old fixture repos untouched and rely on legacy behavior — rejected because the tests would no longer reflect the live contract.

### Update the README to stop implying a named child component
- **Chose:** Replace the generic `crates/x/{runtime,assertions}` examples and trigger bullets with direct `crates/runtime` and `crates/assertions`.
- **Why:** The written contract must match the implemented validator now that the nested option is gone.
- **Alternatives considered:**
  - Leave the README generic and let the implementation be stricter than the docs — rejected because it would recreate spec drift immediately.

## Architectural Notes
This makes the `rs/test` family intentionally narrower than the earlier generalized component interpretation. The family now treats the owned root as the component and expects the production/assertions split directly under `crates/`. That keeps self-hosting and the rule fixtures aligned to one model, but it also means the family no longer serves as a generic validator for nested multi-component repos using `crates/<name>/...`.

## Information Sources
- `.worklogs/2026-03-26-111822-rs-test-flatten-component-layout.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- Runtime rule fixtures under `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_*`

## Open Questions / Future Considerations
- This change makes the family stricter and less general. If future Rust families need named subcomponents again, that support should be designed intentionally rather than copied back from the old `rs/test` implementation.
- The README now speaks in the direct shape only. If that contract is meant to be reused outside this self-hosting context, it may need a broader architectural rethink rather than reintroducing the old nested variant by default.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs` — direct-only component discovery
- `apps/guardrail3/crates/app/rs/families/test/README.md` — updated direct-only target shape and structural triggers
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — representative fixture rewrite from nested to direct shape
- `.worklogs/2026-03-26-111822-rs-test-flatten-component-layout.md` — prior checkpoint that flattened the live tree before removing the old option entirely

## Next Steps / Continuation Plan
1. Keep an eye on downstream assumptions in any callers or future fixtures that still think `crates/<name>/...` is valid for this family.
2. If the direct-only model is now the intended long-term rule contract, propagate that explicitly anywhere else the old named-child shape is still discussed.
3. Re-run both family crate tests and validator-on-self after any future fixture edits, because the direct-only shape is now part of the actual rule semantics, not just the self-hosting layout.
