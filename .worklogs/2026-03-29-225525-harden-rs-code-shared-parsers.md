# Harden RS-CODE Shared Parsers

**Date:** 2026-03-29 22:55
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/{discover.rs,facts.rs,inputs.rs,parse.rs,parse/*,rs_code_01_crate_level_allow.rs,rs_code_03_item_level_allow_without_reason.rs,rs_code_04_item_level_allow_with_reason.rs,rs_code_05_garde_skip_without_comment.rs,rs_code_06_garde_skip_with_comment.rs,rs_code_08_cfg_attr_allow_inventory.rs,rs_code_09_file_length.rs,rs_code_10_use_count_error.rs,rs_code_11_use_count_warn.rs,rs_code_13_todo_macros.rs,rs_code_15_direct_fs_usage.rs,rs_code_16_panic_macro.rs,rs_code_17_impl_allow_blast_radius.rs,rs_code_18_always_true_cfg_attr_bypass.rs,rs_code_19_large_type_inventory.rs,rs_code_20_extern_allow.rs,rs_code_21_fs_glob_import.rs,rs_code_22_deny_forbid_without_reason.rs,rs_code_23_include_bypass.rs,rs_code_24_path_attr.rs,rs_code_25_public_result_error_type.rs,rs_code_26_lib_glob_reexport.rs,rs_code_27_facade_only_lib.rs,rs_code_29_large_trait_inventory.rs,rs_code_32_test_expect_message_quality.rs}` and their touched tests, `apps/guardrail3/crates/app/rs/ast/src/{ast_helpers.rs,ast_helpers_tests/mod.rs,ast_visitors.rs}`, `apps/guardrail3/crates/app/rs/validate/allow_checks.rs`, `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/worker/crates/app/processor/src/lib.rs`

## Summary
Closed the shared correctness gaps that were making `RS-CODE` untrustworthy as a family: test-context handling is now unified, same-line reason parsing is token-aware and exact, `cfg_attr` classification is conservative and recursive, `#[expect(...)]` is owned alongside `#[allow(...)]`, `garde(skip)` uses an explicit exemption set, reachable public API is path-aware, and build-script `include!` no longer silently blesses upward escapes. I also updated the code-family tests and the golden fixture to match those corrected contracts, and the family test suite now passes cleanly again.

## Context & Problem
The user pointed directly at [`apps/guardrail3/crates/app/rs/families/code/FIXES.md`](apps/guardrail3/crates/app/rs/families/code/FIXES.md) and asked for the fixes from that audit to be implemented without weakening the rules. The concrete problem was not one broken rule file: the `RS-CODE` family had drifted around a set of shared parser/classifier mistakes.

The highest-risk issues were:
- test-only context was fragmented across path heuristics and partial AST awareness
- reason parsing accepted forged or malformed comment spellings
- `cfg_attr` truth was overconfident and inconsistent across rules
- `garde(skip)` blessed suffix-shaped aliases like `UserMap`
- public-API error-type scanning was still vulnerable to path/name collisions
- `include!(concat!(env!("OUT_DIR"), ...))` short-circuited to benign inventory even when the appended path escaped upward

Because these are shared parsing/model layers, patching one rule at a time would have kept the family inconsistent and brittle.

## Decisions Made

### Unify parser ownership instead of patching rule-local symptoms
- **Chose:** Fix shared code-family parsers and shared AST helpers first, then repair rule-local tests and expectations on top of that.
- **Why:** `RS-CODE` is dominated by parser/model reuse. Fixing test-context, reason parsing, and `cfg_attr` semantics once is safer than leaving each rule to interpret those concepts differently.
- **Alternatives considered:**
  - Patch only the failing rules (`08`, `16`, `18`, `23`, `25`, `32`) — rejected because the audit findings were cross-cutting and this would have kept the family internally inconsistent.
  - Relax expectations in tests only — rejected because the user explicitly asked not to weaken the family.

### Treat `#[expect(...)]` as owned lint-policy surface
- **Chose:** Fold `expect` into the same shared lint-policy parsing lane as `allow`.
- **Why:** Modern suppression surfaces should not bypass `RS-CODE-03/04/17/20` simply because the attribute spelling changed.
- **Alternatives considered:**
  - Keep `expect` ignored until a later family pass — rejected because that leaves a live suppression hole.

### Make `garde(skip)` semantics explicit and syntax-driven
- **Chose:** Use a narrow exempt set in the AST layer: primitives, explicit map/set roots, references, trait objects, `Option<exempt>`, and clap subcommand fields. Type-level skip is exempt only when all fields are exempt.
- **Why:** The previous suffix-based `*Map/*Set` heuristic was a semantic loophole and did not match the family’s contract.
- **Alternatives considered:**
  - Keep suffix heuristics because tests already encoded them — rejected because the audit specifically identified those tests as locking in a bug.
  - Collapse back to “primitive only” — rejected because references, trait objects, subcommands, and explicit maps/sets are real non-validateable surfaces in practice.

### Prefer conservative `cfg_attr` reasoning
- **Chose:** Keep `KnownTrue` / `KnownFalse` / `Unknown` tri-state classification and only let definitely-always-true branches trip `RS-CODE-18`.
- **Why:** Unknown cfg predicates are not safe to collapse into always-false or always-true just to make one rule easier to implement.
- **Alternatives considered:**
  - Preserve `any(unix, windows)` as “effectively exhaustive” — rejected because the audit correctly pointed out that it is not exhaustive.

### Keep the build-script carveout but stop blessing upward escapes
- **Chose:** `include!(concat!(env!("OUT_DIR"), ...))` inventories only when the appended path is non-traversing; if the appended path contains a real parent segment, `RS-CODE-23` now warns on traversal instead of treating it as benign inventory.
- **Why:** Generated-code boundaries are legitimate, but parent-directory escapes inside that carveout are still suspicious.
- **Alternatives considered:**
  - Leave all `OUT_DIR` concat forms as benign inventory — rejected because it silently blesses suspicious path shapes.
  - Convert all `OUT_DIR` concat uses into hard errors — rejected because the build-script carveout is intentional and useful.

## Architectural Notes
This slice continues the design direction for `RS-CODE`: the family should own a small set of shared structural parsers/classifiers, and individual rules should be thin projections over typed local facts.

Notable structural changes:
- `discover.rs` now distinguishes real test-root paths from ad hoc “looks test-ish” names
- `parse/helpers.rs`, `parse/analysis_helpers.rs`, and `parse/types.rs` now own the shared lint-policy and `cfg_attr` truth model
- `parse/attrs.rs` owns public-API reachability and include macro ownership in a path-aware way
- AST-level `garde(skip)` semantics live in `app/rs/ast`, because that layer is already the source of truth for typed skip discovery

The family tests are now stronger than before because the fixes came with exact regressions for:
- same-line reason syntax
- string-literal `//` decoys
- nested `cfg_attr`
- private-module/private-type public-API false positives
- `UserMap` / `FeatureSet` / `Option<UserMap>` garde-skip loopholes
- build-script include traversal

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — the concrete audit backlog this slice targeted
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/*` — shared parser/model ownership for the family
- `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs` — AST-layer source of truth for typed garde-skip discovery
- `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/worker/crates/app/processor/src/lib.rs` — live fixture that proved the stronger `RS-CODE-32` threshold
- prior `RS-CODE` worklog:
  - `.worklogs/2026-03-29-221132-advance-rs-code-path-and-expect-cleanup.md`
- subagent guidance captured during this slice for:
  - `garde(skip)` exemption design
  - reachable-public-API ownership
  - conservative `cfg_attr` truth classification

## Open Questions / Future Considerations
- The family correctness fixes are in, but live repo-root `RS-CODE` still has substantial debt in real project files: weak test `expect(...)` messages, direct `std::fs` usage in test support crates, oversize files, and use-count violations.
- `RS-CODE-24` still emits a very large warning bucket on justified `#[path]` sidecar wiring. That is repo debt, not a family-correctness bug, and will require actual layout cleanup if the goal is zero live warnings.
- The legacy `apps/guardrail3/crates/app/rs/validate/allow_checks.rs` path was updated only enough to keep the old lane consistent with the new `garde(skip)` semantics; it still remains legacy code.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/FIXES.md` — the audit backlog this commit addresses
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` — shared item/`cfg_attr` lint-policy parsing
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs` — tri-state `cfg_attr` truth and structural path-traversal helpers
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs` — include macro ownership and public-API result-error reachability
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` — unified AST test-context handling for macros and `expect(...)`
- `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs` — explicit garde-skip exemption semantics
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass.rs` — build-script include carveout behavior
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_25_public_result_error_type.rs` — rule surface that now relies on path-aware reachable API parsing
- `.worklogs/2026-03-29-221132-advance-rs-code-path-and-expect-cleanup.md` — previous `RS-CODE` reduction checkpoint

## Next Steps / Continuation Plan
1. Re-run live repo-root `RS-CODE` and work down the remaining real buckets in order of leverage:
   - `RS-CODE-32` weak test `expect(...)` messages
   - `RS-CODE-15` direct `std::fs` usage in family `test_support` crates
   - `RS-CODE-09` oversize files
   - `RS-CODE-10/11` use-count violations
   - `RS-CODE-05` live `garde(skip)` findings in domain crates
2. Keep repo-debt cleanup separate from this parser-correctness slice. The next commit should be a repo-root debt reduction checkpoint, not more shared parser churn.
3. Once live repo-root `RS-CODE` is substantially reduced, run an adversarial code-family pass focused on:
   - `cfg_attr` ownership boundaries (`08/18/20/22/24`)
   - garde-skip exemption edge cases
   - public result-error reachability collisions
   - build-script include carveouts
