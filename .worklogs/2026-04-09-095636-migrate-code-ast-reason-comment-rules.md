# Migrate Code AST Reason and Comment Rules

**Date:** 2026-04-09 09:56
**Scope:** `packages/rs/code/g3rs-code-ast-checks`, `packages/rs/code/g3rs-code-ast-ingestion`

## Summary
Migrated the remaining clearly single-file, comment-sensitive `code` AST rules into `g3rs-code-ast-checks`: `RS-CODE-01`, `02`, `03`, `04`, `05`, `06`, `08`, and `22`. Added the parse/support helpers those rules need, added rule-local tests for each new rule, and extended the crawl -> ingestion -> AST checks pipeline test to prove the new rules fire end to end.

## Context & Problem
The first `g3rs-code-ast-checks` slice already covered syntax-local rules like `todo!`, `panic!`, and `std::fs` usage. The next batch was still single-file, but unlike the first slice it depended on raw source text and same-line comment heuristics:

- lint-policy rules that require a same-line `// reason:`
- `#[garde(skip)]` rules that need both AST type context and source comments
- conditional `cfg_attr` inventory rules
- deny/forbid rules that share the same reason policy as item-level `allow`

The main architectural question was whether these rules were still valid for the single-file AST lane. The answer was yes: they do not need cross-file context, but they do need both AST and raw file content.

## Decisions Made

### Keep these rules in the single-file AST lane
- **Chose:** Migrate `01`, `02`, `03`, `04`, `05`, `06`, `08`, and `22` into `g3rs-code-ast-checks`.
- **Why:** They all operate on one Rust source file. The extra complexity is comment and reason extraction, not file-tree or multi-file scope.
- **Alternatives considered:**
  - Move them to a text-only lane — rejected because the rules still need AST structure.
  - Defer them until multi-file AST exists — rejected because that would delay rules that already fit the current package boundary.

### Extend the AST runtime instead of pushing logic into ingestion
- **Chose:** Add source-comment and attribute-analysis helpers to the AST checks runtime.
- **Why:** Ingestion should keep doing selection and mapping only. Rule semantics like `same_line_reason`, `garde(skip)` type classification, and deny/forbid attribute analysis belong in the checks package runtime.
- **Alternatives considered:**
  - Compute reason/comment facts in ingestion — rejected because that would move rule semantics into the wrong layer.
  - Reuse legacy app AST helpers directly — rejected because `packages/` must not depend on app code.

### Reuse shared reason-policy logic
- **Chose:** Add `guardrail3-reason-policy` as the reason-quality dependency for the new rules.
- **Why:** The repo already has a shared package for useful-reason evaluation. Reusing it keeps the package boundary clean and preserves reason-policy behavior.
- **Alternatives considered:**
  - Copy the reason heuristic into `g3rs-code-ast-checks` — rejected because it would fork policy logic.
  - Depend on the old app crate — rejected because app code is legacy and not part of the target architecture.

### Lock behavior with rule-local tests plus pipeline coverage
- **Chose:** Add per-rule sidecar tests for all eight rules and extend the AST ingestion pipeline smoke test.
- **Why:** This batch is where false positives are most likely. Rule-local tests pin the exact behavior; pipeline tests prove the full crawl -> ingest -> check path still works.
- **Alternatives considered:**
  - Only rely on pipeline tests — rejected because it would make failures harder to localize.
  - Only rely on rule-local tests — rejected because it would not prove the lane wiring.

## Architectural Notes
- `g3rs-code-ast-checks` now explicitly supports rules that need both:
  - parsed Rust AST
  - raw source content from the same file
- The public AST input shape did not need to change. `G3RsSourceFile` already carries `content`, so the runtime can parse once and still inspect original source lines for `// reason:` coupling.
- `parse/comments` is now the source-text helper layer for same-line comment detection and reason extraction.
- `parse/garde_skips` is a local runtime helper that classifies `#[garde(skip)]` targets without pulling in app AST code.
- The ingestion package stayed simple. Its only change was stronger pipeline coverage for the new rules.

## Information Sources
- `.plans/todo/checks/rs/code.md` — current rule inventory and intended severity/behavior notes for the `code` family.
- `packages/shared/reason-policy/crates/reason-policy/src/lib.rs` — shared useful-reason heuristic used by `RS-CODE-03`, `04`, `06`, and `22`.
- Legacy rule implementations under `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/...` for:
  - `rs_code_01_crate_level_allow`
  - `rs_code_02_unused_crate_dependencies_allow`
  - `rs_code_03_item_level_allow_without_reason`
  - `rs_code_04_item_level_allow_with_reason`
  - `rs_code_05_garde_skip_without_comment`
  - `rs_code_06_garde_skip_with_comment`
  - `rs_code_08_cfg_attr_allow_inventory`
  - `rs_code_22_deny_forbid_without_reason`
- `.worklogs/2026-04-08-183810-migrate-more-code-ast-rules.md` — prior AST slice and the “annoying but still single-file” decision.

## Open Questions / Future Considerations
- `RS-CODE-08` currently matches the legacy app behavior (`Warn`, non-inventory). The plan file currently describes it as inventory/info, so either the plan or the rule semantics need to be reconciled later.
- The new comment scanners are intentionally scoped to same-line Rust source heuristics. If later rules need more global comment association, that should be added deliberately rather than inferred from these helpers.
- `profile_name` resolution is still absent from AST ingestion, so profile-sensitive `code` rules are still deferred.

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs` — the current list of migrated `code` AST rules.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/comments/mod.rs` — same-line comment and `// reason:` extraction logic.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — lint-policy, cfg-attr, deny/forbid, and module-level attribute scanning.
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/garde_skips.rs` — typed `#[garde(skip)]` discovery and exemption logic.
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end crawl -> ingest -> check coverage for the migrated AST rules.
- `.plans/todo/checks/rs/code.md` — the `code` rule ledger and remaining migration list.
- `.worklogs/2026-04-08-183810-migrate-more-code-ast-rules.md` — previous AST migration slice and test-attack hardening context.

## Next Steps / Continuation Plan
1. Migrate the next still-single-file `code` AST rules that need more bookkeeping than comment parsing, especially `RS-CODE-09`, `10`, `11`, `19`, `31`, and `33`.
2. Add real `profile_name` resolution in `g3rs-code-ast-ingestion` so profile-sensitive `code` rules can move into the AST lane without placeholder assumptions.
3. Reconcile the `RS-CODE-08` spec mismatch between `.plans/todo/checks/rs/code.md` and the retained legacy behavior, then update either the rule or the plan explicitly.
4. After the remaining single-file `code` rules are migrated, run another broader test-attack pass focused on overlap, duplicate findings, and rule interactions across the full `code` AST lane.
