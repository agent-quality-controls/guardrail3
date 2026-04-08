# Harden Code AST Lane After Test Attack

**Date:** 2026-04-08 18:02
**Scope:** `packages/rs/code/g3rs-code-ast-checks`, `packages/rs/code/g3rs-code-ast-ingestion`

## Summary
Hardened the first code AST lane after an adversarial test pass. Added explicit parse-failure reporting (`RS-CODE-30`), restored several old false-positive and edge-case tests for `RS-CODE-13`, `RS-CODE-15`, and `RS-CODE-16`, and tightened the end-to-end pipeline tests.

## Context & Problem
The first attack pass found two real weaknesses:

1. parse failures in `g3rs-code-ast-checks` silently returned no results
2. the new tests were much weaker than the old family tests for the three migrated rules

The user asked to fix those issues and run another attack round instead of just listing them.

## Decisions Made

### Surface parse failures as `RS-CODE-30`
- **Chose:** emit an explicit `RS-CODE-30` result from the checks runtime when AST parsing fails
- **Why:** silent drop is fail-open behavior; the old family already treated source parse failures as owned findings
- **Alternatives considered:**
  - keep returning an empty result set — rejected because it hides broken input
  - move parse-failure handling into ingestion — rejected because AST parsing still belongs in the checks runtime

### Restore old false-positive and edge-case coverage
- **Chose:** port the important old scenarios directly into the new checks package tests
- **Why:** those cases already encoded known behavior and previous bug history; the first extracted slice had kept only the happy-path tests
- **Alternatives considered:**
  - rely only on end-to-end pipeline tests — rejected because rule-local coverage is still needed for precise branch behavior

### Tighten the pipeline tests
- **Chose:** assert exact count behavior for the main positive fixtures and add parse-failure plus mixed-workspace pipeline cases
- **Why:** the first pipeline tests proved wiring, but they were still too loose for regression prevention
- **Alternatives considered:**
  - leave pipeline tests as “some rule id appears” — rejected because duplicate or mis-attributed findings could slip through

## Architectural Notes
`RS-CODE-30` now lives in the new AST checks package as a runtime-owned input-failure rule, which matches the package architecture better than a bare `return Vec::new()`.

The implementation fix in `fs_visitors.rs` also corrected a real behavior bug: `#[cfg(test)]` on `use` items now enters test context before `RS-CODE-15` evaluates them.

## Information Sources
- `.worklogs/2026-04-08-175001-wire-code-ast-pipeline-tests.md`
- old rule tests under `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene`
- old `RS-CODE-30` rule under `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_30_input_failures`
- current package tests under `packages/rs/code/g3rs-code-ast-checks`

## Open Questions / Future Considerations
- There is still no full repo-golden parity fixture for the new extracted package lane; current coverage is strong enough for the migrated rules, but not yet a full legacy replacement
- `profile_name` is still unresolved in ingestion and will matter once profile-sensitive code rules move over

## Key Files for Context
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/run.rs` — parse failure now routes to `RS-CODE-30`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_30_input_failures/rule.rs` — new input-failure rule
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/parse/fs_visitors.rs` — fixed `#[cfg(test)]` handling for `use` items
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule_tests/false_positives.rs` — restored tricky no-hit cases
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_15_direct_fs_usage/rule_tests/false_positives.rs` — restored tricky no-hit cases
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_16_panic_macro/rule_tests/false_positives.rs` — restored tricky no-hit cases
- `packages/rs/code/g3rs-code-ast-ingestion/crates/runtime/src/ingest_tests/pipeline.rs` — tightened end-to-end lane coverage
- `.worklogs/2026-04-08-171620-code-ast-checks-initial-slice.md` — initial extracted checks package
- `.worklogs/2026-04-08-174415-build-code-ast-ingestion.md` — initial ingestion package

## Next Steps / Continuation Plan
1. Keep migrating the next `code` AST rules and extend both rule-local and pipeline tests as each rule lands.
2. Add `profile_name` resolution in `g3rs-code-ast-ingestion` before moving any profile-sensitive `code` rules.
3. Reuse this hardened test pattern for the first multi-file AST family.
