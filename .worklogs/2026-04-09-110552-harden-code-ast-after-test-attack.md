# Harden code AST after test attack

**Date:** 2026-04-09 11:05
**Scope:** `.plans/todo/checks/rs/code.md`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_09_too_many_effective_code_lines/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/direct.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Summary
Hardened the newly migrated single-file code AST rules after an adversarial test pass. The work tightened the `#[path]` sidecar exemption, added the missing false-positive and boundary tests, expanded end-to-end pipeline coverage, and aligned the rule ledger severity for `RS-CODE-19` with the live implementation.

## Context & Problem
After migrating `RS-CODE-09`, `10`, `11`, `19`, and `24`, an attack review found weak coverage rather than an obvious check bug. The main gaps were around exact thresholds, string-payload exemptions, `#[path]` conditional forms, and canonical sidecar behavior. There was also a stale plan mismatch where `RS-CODE-19` was still marked `Info` even though legacy behavior and the extracted package both emit `Warn`.

## Decisions Made

### Tightened the `#[path]` sidecar exemption
- **Chose:** Exempt only canonical test sidecar wiring where the module name ends with `_tests` and the path matches `<module_name>/mod.rs`.
- **Why:** The broader `ends_with("_tests/mod.rs")` behavior was looser than the written contract and risked letting unrelated `#[path]` redirections slip through.
- **Alternatives considered:**
  - Keep the broader suffix-only exemption — rejected because it was wider than the ledger wording.
  - Exempt only a single hard-coded `rule_tests/mod.rs` path — rejected because the extracted package pattern uses per-rule module names, not one shared literal.

### Treated the attack findings as coverage fixes, not new rule semantics
- **Chose:** Add missing tests and correct one stale pipeline expectation rather than changing rule logic.
- **Why:** The checks already behaved correctly; the missing proof was in the tests. The one failing pipeline assertion came from a wrong assumption that 20 imports should stay fully clean, but `RS-CODE-11` is supposed to warn at 16..=20.
- **Alternatives considered:**
  - Lower the warn threshold to keep the test passing — rejected because it would contradict the rule definition.
  - Add only rule-local tests and skip pipeline hardening — rejected because the user explicitly wanted the full crawl -> ingestion -> checks lane verified.

### Aligned the rule ledger to the extracted implementation
- **Chose:** Change `RS-CODE-19` in the rule plan from `Info` to `Warn`.
- **Why:** The legacy rule, the migrated implementation, and the test expectations all use `Warn`; the plan was stale.
- **Alternatives considered:**
  - Downgrade the implementation to `Info` — rejected because it would break parity with the old rule and with the freshly migrated tests.

## Architectural Notes
The code AST lane remains the same shape:
- crawl selects files
- AST ingestion builds one `G3RsCodeAstChecksInput` per source file
- AST checks parse once per file and fan out to small rule logic

This hardening work deliberately stayed inside that structure. The changes only narrowed exemptions and improved proof that the end-to-end lane preserves rule boundaries and threshold behavior.

## Information Sources
- `.plans/todo/checks/rs/code.md` — live rule ledger and severity source
- `packages/rs/code/g3rs-code-ast-checks/...` — extracted rule implementations and local test suites
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end crawl -> ingestion -> checks proof
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — prior migration context for these five rules
- follow-up four-angle attack review in this session: completeness, missing scenarios, pattern parity, false positives / intent-vs-implementation

## Open Questions / Future Considerations
- Pipeline tests still mostly lock rule ids and a few important titles rather than every exact message for every case. That is acceptable for now but could be tightened later if message stability becomes important.
- The remaining unmigrated `code` AST rules are the profile-sensitive ones. They should wait for real `profile_name` resolution in AST ingestion instead of adding ad hoc context now.

## Key Files for Context
- `.plans/todo/checks/rs/code.md` — code-family rule ledger and current severities
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — shared attribute parsing and `#[path]` exemption logic
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_09_too_many_effective_code_lines/rule_tests/false_positives.rs` — line-count false-positive coverage, including raw-string payloads
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/false_positives.rs` — exact-threshold proof for large-type inventory
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/direct.rs` — direct positive cases for `#[path]`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/false_positives.rs` — sidecar exemption and known-false conditional cases
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end pipeline coverage for the migrated single-file AST rules
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — prior worklog for the original migration

## Next Steps / Continuation Plan
1. Implement real `profile_name` resolution in `packages/rs/code/g3rs-code-ast-ingestion` so the remaining library-only code AST rules can migrate cleanly.
2. Migrate the profile-sensitive single-file code AST rules next: public API shape, facade-only `lib.rs`, weak public error forms, and large-trait checks.
3. Once the remaining single-file `code` AST rules are migrated, run another lane-wide attack focused on profile boundaries and library-only exemptions.
