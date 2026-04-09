# Harden Code AST After Test Attack

**Date:** 2026-04-09 11:00
**Scope:** `.plans/todo/checks/rs/code.md`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_09_too_many_effective_code_lines/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/direct.rs`, `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule_tests/false_positives.rs`, `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Summary
Hardened the newly migrated `code` AST rules after an adversarial test pass. The main work was tightening `#[path]` sidecar exemption behavior, filling missing boundary and negative tests, fixing one stale pipeline expectation, and bringing the `RS-CODE-19` plan severity back in line with the live rule.

## Context & Problem
The previous migration pass added `RS-CODE-09`, `10`, `11`, `19`, and `24`, but the follow-up attack found several weak spots. The biggest issues were incomplete pipeline coverage for `#[path]`, missing negative coverage for raw-string payload lines and exact thresholds, and a stale plan/code mismatch for `RS-CODE-19`. There was also a risk that the `#[path]` test-sidecar exemption had become broader than the written contract.

## Decisions Made

### Tighten `#[path]` sidecar exemption to the canonical pattern
- **Chose:** Require test context plus `<module_name>_tests/mod.rs` that exactly matches the module name.
- **Why:** The plan only blesses canonical rule-sidecar wiring. The earlier exemption logic accepted broader `*_tests/mod.rs` forms and could hide noncanonical path redirects.
- **Alternatives considered:**
  - Keep the broad `*_tests/mod.rs` exemption — rejected because it exceeded the written contract.
  - Remove the exemption entirely — rejected because canonical sidecar test modules are an intentional project pattern.

### Prove thresholds and negative cases directly
- **Chose:** Add rule-local and pipeline tests for raw-string payload lines, exact import thresholds, test-file import exemption, exact large-type thresholds, weak/missing `#[path]` reasons, path escape, conditional path attrs, and canonical sidecar exemption.
- **Why:** These rules are mostly boundary-driven. Without explicit threshold and false-positive tests, they can drift while the suite stays green.
- **Alternatives considered:**
  - Rely on existing direct happy-path tests — rejected because that misses the main failure modes for these checks.
  - Add only pipeline coverage — rejected because rule-local tests still need tight exact-result assertions.

### Keep `RS-CODE-19` as Warn and fix the plan
- **Chose:** Update the ledger to `Warn`.
- **Why:** The implemented rule and legacy behavior already use `Warn`. The plan was stale.
- **Alternatives considered:**
  - Change the rule to `Info` — rejected because it would create parity drift from the established behavior with no architectural benefit.

## Architectural Notes
These changes keep the intended lane split intact:
- `g3rs-code-ast-ingestion` still owns `crawl -> ingest_for_ast_checks`
- `g3rs-code-ast-checks` still owns AST parsing and rule execution
- boundary and false-positive behavior is now locked at both rule-local and pipeline levels

The `#[path]` exemption remains a rule concern, not an ingestion concern, because it is a single-file source policy exception rather than a file-discovery failure.

## Information Sources
- `.plans/todo/checks/rs/code.md` — rule ledger and expected behavior
- `packages/rs/code/g3rs-code-ast-checks/...` rule implementations and test patterns
- `packages/rs/code/g3rs-code-ast-ingestion/.../pipeline.rs` — end-to-end lane proof
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — prior migration pass
- `.worklogs/2026-04-08-180202-harden-code-ast-lane-after-test-attack.md` — earlier AST hardening pattern

## Open Questions / Future Considerations
- Pipeline tests still mostly lock file-to-rule attribution and a few key titles, not every exact message for every migrated rule.
- The remaining profile-sensitive `code` AST rules still need real `profile_name` resolution before migration.

## Key Files for Context
- `.plans/todo/checks/rs/code.md` — current `code` rule ledger and severities
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/attrs.rs` — source-attribute parsing, including `#[path]` exemption logic
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_24_path_attr_with_reason/rule.rs` — `#[path]` policy ownership
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — end-to-end AST lane proof
- `.worklogs/2026-04-09-102753-migrate-more-code-ast-single-file-rules.md` — prior batch that introduced the affected rules
- `.worklogs/2026-04-09-110050-harden-code-ast-after-test-attack.md` — this hardening pass

## Next Steps / Continuation Plan
1. Migrate the remaining profile-sensitive single-file `code` AST rules after adding real `profile_name` resolution to `g3rs-code-ast-ingestion`.
2. Keep using adversarial boundary tests after each rule batch, especially for warning/error threshold splits and sidecar exemptions.
3. Once the remaining `code` AST rules are migrated, decide whether to add one broader package golden fixture or continue with the current rule-local plus pipeline pattern.
