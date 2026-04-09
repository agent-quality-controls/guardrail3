# Harden Code AST Single-File Tests

**Date:** 2026-04-09 10:59
**Scope:** `.plans/todo/checks/rs/code.md`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_09_too_many_effective_code_lines/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/direct.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Summary
This pass closed the test-attack gaps in the newly migrated single-file code AST rules. The changes tightened the `#[path]` exemption contract, added missing boundary and false-positive tests, and fixed stale plan text so the rule ledger matches the actual rule behavior.

## Context & Problem
After migrating the single-file code AST rules for effective line count, import count, large type inventory, and `#[path]`, an adversarial review found that the implementation was mostly sound but the proof around it was weak. The biggest issues were incomplete pipeline coverage for `RS-CODE-24`, missing string-payload coverage for the effective-line rule, missing threshold proofs for the large-type rule, and a stale severity entry for `RS-CODE-19` in the rule plan.

## Decisions Made

### Tighten `#[path]` sidecar exemptions to the written contract
- **Chose:** Restrict the exemption to canonical test sidecar wiring where the module name ends with `_tests` and the path matches `<module_name>/mod.rs`.
- **Why:** The previous helper exempted any path ending in `_tests/mod.rs`, which was broader than the plan text and easier to abuse.
- **Alternatives considered:**
  - Keep the broad suffix exemption — rejected because it allowed more patterns than the written rule contract.
  - Exempt only a single hard-coded `rule_tests/mod.rs` path — rejected because the project’s current sidecar pattern uses rule-specific module names.

### Add explicit boundary and false-positive coverage
- **Chose:** Add direct and end-to-end tests for raw-string payload lines, exact struct/enum thresholds, conditional `cfg_attr(..., path = ...)`, missing/weak `#[path]` reasons, parent escapes, and canonical test sidecars.
- **Why:** These were the concrete gaps surfaced by the attack pass, and they are exactly the kinds of edge cases that regress silently if only happy-path fixtures are tested.
- **Alternatives considered:**
  - Rely on rule-local happy-path tests only — rejected because the pipeline layer is where selection and aggregation mistakes show up.
  - Add one broad golden fixture instead — rejected for now because the current package already uses focused rule-local and pipeline tests, and these gaps were narrow and specific.

### Align the plan with live rule behavior
- **Chose:** Change `RS-CODE-19` in the plan from `Info` to `Warn`.
- **Why:** Legacy behavior, the migrated implementation, and the tests all treat the rule as `Warn`; leaving the plan at `Info` would keep creating false mismatches in future audits.
- **Alternatives considered:**
  - Downgrade the implementation to `Info` — rejected because that would create a behavior change instead of fixing stale documentation.

## Architectural Notes
These changes keep the same package boundary:
- ingestion still owns `crawl -> ingest_for_ast_checks`
- checks runtime still owns AST parsing and local rule fan-out
- rule files remain pure and single-file

The only behavioral boundary change was in `RS-CODE-24`: its test-sidecar exemption is now explicitly tied to the current rule-specific sidecar naming pattern rather than a broad filename suffix.

## Information Sources
- `.plans/todo/checks/rs/code.md` — rule inventory, severity, and exemption wording
- `packages/rs/code/g3rs-code-ast-checks/...` rule and parser files — current implementation
- `packages/rs/code/g3rs-code-ast-ingestion/.../pipeline.rs` — end-to-end crawl -> ingestion -> checks proof
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — prior migration context
- `.worklogs/2026-04-09-101003-fix-code-rule-8-plan-severity.md` — recent rule-ledger cleanup context

## Open Questions / Future Considerations
- The pipeline tests now lock the key edge cases for this batch, but they still assert mostly by rule id and title rather than every exact message in every scenario.
- `RS-CODE-24` is now aligned with the current sidecar naming convention; if the project changes that convention later, the exemption helper and plan text will need to move together.

## Key Files for Context
- `.plans/todo/checks/rs/code.md` — current code-family rule ledger and severity source of truth
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — `#[path]` exemption logic and policy parsing
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule.rs` — `#[path]` rule behavior
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — full crawl -> ingestion -> checks proof for the single-file AST lane
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — previous migration step for these rules

## Next Steps / Continuation Plan
1. Continue the code-family AST migration with the remaining profile-sensitive single-file rules (`RS-CODE-26`, `27`, `29`, `31`, `33`) once `profile_name` is resolved in code AST ingestion.
2. Keep the rule ledger and migrated behavior aligned as each new batch lands so audits do not keep surfacing stale severity mismatches.
3. When the profile-sensitive batch lands, extend the pipeline tests with library-profile fixtures so those rules are proven end to end the same way this batch now is.
