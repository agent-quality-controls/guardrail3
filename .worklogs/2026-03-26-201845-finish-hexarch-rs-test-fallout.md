# Finish Hexarch RS-Test Fallout

**Date:** 2026-03-26 20:18
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_01_crates_exists_tests/core.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_02_exact_contents_tests/compound_attacks.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_03_inbound_outbound_tests/valid_variants.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_04_loose_files_tests/gitkeep_edges.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_06_leaf_valid_tests/valid_variants.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs_tests/ownership.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_10_members_within_app_boundary_tests/ownership.rs`

## Summary
Finished the post-tightening fallout from `RS-TEST` in the `hexarch` family. The remaining work was split between validator proof-detection gaps in `RS-TEST-07` and stale `hexarch` test expectations that still reflected older result ownership and file-target behavior.

## Context & Problem
The earlier `RS-TEST` tightening correctly exposed `hexarch` structural and semantic-proof issues, but after the first cleanup pass two problems remained:

1. `RS-TEST-07` was still under-crediting legitimate proof sites in assertions modules. In practice this showed up as dozens of false `test lacks real proof site` warnings in `hexarch`, even when sidecars were calling exported assertion helpers.
2. Several `hexarch` tests were still asserting old result ownership/file targeting, especially in rule 02 compound attacks and the ownership splits around rules 07/09/10.

The goal for this checkpoint was to make the tightened validator and the rewritten families converge cleanly instead of preserving either false greens or false reds.

## Decisions Made

### Extend proof detection instead of mass-rewriting remaining valid tests
- **Chose:** Tighten `RS-TEST-07` proof analysis in the validator so it recognizes:
  - public `use` bindings as public imports in the AST model
  - public re-exported `assert_*` functions in assertions modules as proof-bearing exported APIs
  - bare local calls from one assertions helper to another proof-bearing helper in the same module
  - bare imported external `assert_*` helpers as proof-bearing when they are used inside assertions helpers
- **Why:** The remaining `hexarch` warnings were concentrated in modules already using owned assertions helpers, but those helpers proved through imported `assert_*` functions from assertions-common. Treating those as non-proof would force a large family rewrite for no semantic gain.
- **Alternatives considered:**
  - Rewrite all remaining warned `hexarch` tests to use a different local helper surface — rejected because the validator, not the family intent, was the limiting factor.
  - Count any imported helper as proof-bearing — rejected because that is too broad and would weaken `RS-TEST-07`.

### Fix stale `hexarch` expectations instead of changing rule behavior
- **Chose:** Update the failing `hexarch` tests to match current rule ownership and result paths.
- **Why:** Targeted reruns showed the rule behavior was coherent and the failures were expectation drift:
  - `RS-HEXARCH-01` still emits `RS-HEXARCH-01`, not an empty id
  - mixed ownership under rules 07/09/10 now resolves to one rule-07 result plus separate 09/10 ownership, not duplicated rule-07 ownership
  - rule 02 loose-file and unexpected-dir compound cases attach files exactly where the rule currently reports them
- **Alternatives considered:**
  - Change runtime rule logic to match the stale tests — rejected because the current behavior was internally consistent and already validated by targeted adversarial review.
  - Leave the tests red and only rely on `rs validate` — rejected because the family crate itself must stay green.

## Architectural Notes
- `RS-TEST-07` is now less heuristic and more explicit about proof ownership. The important boundary is that proof can come from an owned assertions API even when the actual assertion macros live in shared assertions-common helpers.
- `parse.rs` now records `pub use` visibility so the family can distinguish exported assertion APIs from ordinary imports.
- The `hexarch` fallout is now isolated to test expectations and no longer indicates a mismatch between the validator and the live family structure.

## Information Sources
- `.worklogs/2026-03-26-194521-tighten-rs-test-validator-checkpoint.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_02_exact_contents_tests/compound_attacks.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs_tests/ownership.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_10_members_within_app_boundary_tests/ownership.rs`
- direct local test runs and validator runs during this session

## Open Questions / Future Considerations
- `RS-TEST-16` and `RS-TEST-18` are still known to be weaker than the intended semantic ownership contract. This checkpoint fixes the concrete fallout around `RS-TEST-07`, not the broader remaining strictness gaps.
- There is still an external subagent commit (`ee4834d`) referenced during this session for the earlier rule-11..25 `hexarch` sidecar cleanup. Before future history cleanup or squashing, review whether that commit needs to be preserved or whether its changes are already fully represented in the current branch state.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — proof catalog construction and `RS-TEST` orchestration
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — AST facts used for proof-site analysis
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_02_exact_contents_tests/compound_attacks.rs` — final rule-02 expectation fixes
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs_tests/ownership.rs` — ownership split expectation fixes
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_10_members_within_app_boundary_tests/ownership.rs` — ownership split expectation fixes
- `.worklogs/2026-03-26-194521-tighten-rs-test-validator-checkpoint.md` — validator-tightening checkpoint that created the remaining fallout

## Next Steps / Continuation Plan
1. If the goal is “strictly clean under the intended contract” rather than “clean under the live enforced contract,” tighten `RS-TEST-16` and `RS-TEST-18` next and rerun the same family sweep (`test`, `arch`, `cargo`, `hexarch`).
2. If commit hygiene matters later, inspect how the earlier `hexarch` sidecar-cleanup work was split across commits and whether any subagent-only commit hashes need to be merged or documented.
3. Use the same four-family sweep after any future `RS-TEST` tightening:
   - `rs validate .../families/test --family test`
   - `rs validate .../families/arch --family test`
   - `rs validate .../families/cargo --family test`
   - `rs validate .../families/hexarch --family test`
