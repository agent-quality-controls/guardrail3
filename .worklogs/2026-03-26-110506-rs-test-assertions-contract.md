# RS-TEST Assertions Contract Tightening

**Date:** 2026-03-26 11:05
**Scope:** `apps/guardrail3/crates/app/rs/families/test/**`, `.plans/todo/checks/rs/test.md`

## Summary
Tightened the `rs/test` family so proof through owned assertions is explicit instead of heuristic. `RS-TEST-07` now only credits proof-bearing assertions functions, and three new rules (`RS-TEST-16`..`18`) enforce non-hollow assertions modules, assertions-only external harness proof, and generic `test_support` boundaries.

## Context & Problem
The family had been made to self-host structurally, but its `assertions` crate was still mostly a thin wrapper layer around `run_family`, `finding`, and `rule_files`. That meant the validator could accept an `assertions` split without proving that semantic assertions actually lived there. The README also still contained an outdated family carveout even after the validator exemption had been removed. The user wanted the README brought in line with the stricter intent, the exemption removed, and the family made to fail honestly on its own hollow assertions layer.

## Decisions Made

### Tighten proof-site semantics instead of adding another heuristic
- **Chose:** Extend `RS-TEST-07` so a call into owned assertions only counts when the target function is proof-bearing.
- **Why:** The existing “any call into assertions counts” rule let thin wrappers satisfy proof without actually asserting anything.
- **Alternatives considered:**
  - Keep `RS-TEST-07` unchanged and add only a separate hollow-assertions rule — rejected because external harnesses would still get proof credit from wrappers.
  - Require direct assertion macros everywhere — rejected because it defeats reusable semantic assertions.

### Add deterministic structural rules rather than intent inference
- **Chose:** Add `RS-TEST-16`, `RS-TEST-17`, and `RS-TEST-18` as syntax-and-ownership rules.
- **Why:** The user wanted stronger structure, but it had to remain mechanically checkable. Proof-bearing exported functions, no direct assertions in external harnesses, and no sibling runtime/assertions imports in `test_support` are precise enough to validate.
- **Alternatives considered:**
  - Try to infer whether a helper “feels reusable” or “feels semantic” — rejected because that would be non-deterministic and brittle.
  - Ban all assertions outside the assertions crate — rejected because sidecars still need legitimate local assertions.

### Let the family fail on `RS-TEST-16` instead of preserving a carveout
- **Chose:** Keep the family on the same rules as any other target and accept a self-fail on the current wrapper-style assertions modules.
- **Why:** The user explicitly rejected any meta-exception for the rule family itself.
- **Alternatives considered:**
  - Keep the README carveout or reintroduce a validator carveout — rejected because it would make self-hosting dishonest.
  - Refactor the family assertions crate in the same commit — rejected because the immediate goal was to make the contract explicit and surface the failure first.

## Architectural Notes
Proof-bearing detection is now computed once in the family orchestrator from parsed assertions files. Exported assertions functions are marked proof-bearing if they contain an allowlisted assertion macro or call another proof-bearing owned assertions function by a supported local path shape. That proof catalog is then reused by `RS-TEST-07` and `RS-TEST-16`, which keeps the rules pure while avoiding repeated discovery work.

`RS-TEST-17` is intentionally narrow: it only applies to `runtime/tests/*.rs`, so sidecar tests can still keep local assertions where they genuinely need private setup or local edge-case checks. `RS-TEST-18` is also scoped narrowly to sibling runtime/assertions component crates, not every local crate in the repository, so generic helper crates can still depend on shared domain/app crates.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_07_real_proof_site.rs`
- Existing wrapper-style helper modules in `apps/guardrail3/crates/app/rs/families/test/crates/family/assertions/src/*.rs`
- Prior self-hosting worklog: `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md`

## Open Questions / Future Considerations
- The family now fails on 18 `RS-TEST-16` findings because its assertions crate is still a thin wrapper layer. The next session should decide whether to move semantic expectations into assertions modules or reduce what the assertions crate exposes.
- `RS-TEST-16` currently uses function-name-level proof propagation within an assertions crate. If multiple modules intentionally export the same function name, the rule may need qualified-path tracking.
- External harness fixtures in older tests still use patterns that are now acceptable only because those tests assert on other rule IDs. If “clean fixture” discipline matters, those fixtures should be tightened later.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/README.md` — live family contract, now updated to 18 rules and no family carveout
- `.plans/todo/checks/rs/test.md` — inventory mirror and gotchas for the family
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/lib.rs` — orchestrator wiring, proof-bearing catalog, new rule fan-out
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/parse.rs` — function/import/call extraction used by proof-bearing analysis
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_07_real_proof_site.rs` — tightened proof-site rule
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_16_assertions_modules_prove.rs` — new hollow-assertions rule
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_17_external_harnesses_use_assertions.rs` — new external-harness proof-placement rule
- `apps/guardrail3/crates/app/rs/families/test/crates/family/runtime/src/rs_test_18_test_support_generic.rs` — new generic-helper-boundary rule
- `apps/guardrail3/crates/app/rs/families/test/crates/family/assertions/src/rs_test_01_inline_test_bodies.rs` — representative thin wrapper that now fails `RS-TEST-16`
- `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md` — prior self-hosting refactor that this work tightens

## Next Steps / Continuation Plan
1. Refactor the family assertions crate so each `rs_test_*` assertions module owns real semantic assertions instead of only forwarding to `run_family`, `finding`, and `rule_files`.
2. Re-run `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib` after each small slice, because the runtime sidecar tests currently depend on the assertions crate shape.
3. Re-run `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/test --family test --inventory --format json` after each assertions refactor step until the family no longer fails `RS-TEST-16`.
