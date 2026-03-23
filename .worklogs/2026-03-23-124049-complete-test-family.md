# Complete Test Family

**Date:** 2026-03-23 12:40
**Scope:** `.plans/todo/checks/rs/test.md`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/test/*`

## Summary
Implemented the new `rs/test` family in the strict checker architecture with `RS-TEST-01..19`, one production file per rule, and one rule-specific test file per rule. The family replaces the old `WalkDir`/string-heavy validator baseline with `ProjectTree` discovery, typed root/file/function/module inputs, syn-based source analysis, and an explicit fail-closed input-failure rule.

## Context & Problem
The old Rust test validator existed only in `app/rs/validate/test_checks.rs` and `test_quality_checks.rs`, and the plan overstated that baseline. Before implementation, the plan still treated `01..09` as “implemented” even though the legacy code failed open on unreadable and unparsable inputs, used raw line matching for `[profile.mutants]`, and only did substring checks for mutation hooks.

The current breadth-first phase requires each family to be implemented to the new architecture standard, not as a placeholder. For `rs/test`, that meant:
- no grouped rule files
- no grouped family tests
- no direct filesystem crawling for cached config files
- no silent parse/read skips
- preserving useful old adversarial ideas without treating the old code as a correctness oracle

## Decisions Made

### Added An Explicit Input-Failure Rule
- **Chose:** introduce `RS-TEST-19` as a hard error for unreadable or unparsable test-family inputs.
- **Why:** the old family silently skipped broken Cargo, Rust source, and hook inputs. That creates bypass holes, especially for `RS-TEST-04` and `RS-TEST-09`.
- **Alternatives considered:**
  - Keep old fail-open behavior — rejected because it would preserve the exact blind spots we wanted to remove.
  - Fold parse errors into each existing rule — rejected because that conflates concerns and bloats unrelated rule inputs.

### Split The Family Into Global, Root, File, Function, And Module Inputs
- **Chose:** model `rs/test` as:
  - tool input for `RS-TEST-01`
  - hook input for `RS-TEST-08`
  - root inputs for mutation/nextest config rules
  - coverage inputs for root-level test presence/inventory rules
  - file inputs for ignore/inline-test/file-length rules
  - function inputs for naming/assertion/should-panic/matches rules
  - module inputs for `#[cfg(test)] mod ...` naming
- **Why:** this matches the “one input instance = one opportunity for the rule to fire” contract and keeps AST-heavy rules local.
- **Alternatives considered:**
  - One giant parsed-file or family bag input — rejected because it would push orchestration logic back into rules.
  - Rule functions reading `ProjectTree` directly — rejected because that breaks the architecture contract.

### Kept Hook Detection On Cached Pre-Commit Files
- **Chose:** implement `RS-TEST-08` against cached pre-commit files already present in `ProjectTree` instead of reviving `.claude/` scanning.
- **Why:** the current project direction no longer treats `.claude/` as the active hook architecture, and the walker already caches relevant pre-commit files. This let the new family use executable-line matching instead of old whole-file substring checks.
- **Alternatives considered:**
  - Preserve `.claude/` scanning — rejected because it would require reintroducing config caching and semantics that are no longer current.
  - Defer the rule entirely until hook refactor — rejected because breadth-first completion still requires a working test-hook rule now.

### Treated Rust Test Coverage As Root-Scoped
- **Chose:** compute test presence, public function count, test function count, and integration-test existence per workspace root / standalone package root.
- **Why:** `rs/test` should behave like the other Rust families and work correctly for nested workspaces and standalone package roots, not just repo-root `Cargo.toml`.
- **Alternatives considered:**
  - Repo-root only semantics from the old validator — rejected because it undercounts nested roots and would be inconsistent with the rest of the new Rust checks.

### Reused Old AST Helpers Only Where They Were Still Structurally Sound
- **Chose:** reuse old AST helpers for:
  - public fn counting
  - test attribute counting
  - `#[ignore]` reason detection
- **Why:** these helpers were already structural and useful attack seeds, unlike the old walk/orchestration layer.
- **Alternatives considered:**
  - Rebuild all helper logic from scratch — rejected because it would duplicate working AST logic without improving the family contract.
  - Reuse old family orchestration — rejected because it was line-based, root-only, and fail-open.

## Architectural Notes
`rs/test` follows the same architecture now used by the completed Rust families:
- `mod.rs` orchestrates only
- `facts.rs` owns root discovery, root config parsing, hook discovery, and file assignment
- `parse.rs` owns syn-based source analysis and test-specific AST visitors
- `inputs.rs` defines atomic typed inputs
- each `RS-TEST-*` rule lives in its own production file
- each rule has its own sidecar test file

The family adds a slightly richer layering than the earlier config families because test rules naturally split across:
- tool presence
- hook/config roots
- per-root aggregates
- per-file AST findings
- per-function AST findings
- per-module AST findings

That split is intentional and should be reused when the remaining heavy families need similar local-vs-aggregate separation.

## Information Sources
- `.plans/todo/checks/rs/test.md` — active rule contract and stale-status cleanup target
- `apps/guardrail3/crates/app/rs/validate/test_checks.rs` — old baseline for `R-TEST-01/02/03/04/09`
- `apps/guardrail3/crates/app/rs/validate/test_quality_checks.rs` — old baseline for `R-TEST-05/06/07/08`
- `apps/guardrail3/crates/app/rs/validate/ast_helpers.rs` — reusable structural AST helpers
- `apps/guardrail3/crates/app/rs/validate/extra_visitors.rs` — old `#[ignore]` behavior reference
- `apps/guardrail3/tests/unit/rs_test_checks_test.rs` — old unit seeds for `01/02/03/04/09`
- `apps/guardrail3/tests/unit/rs_test_quality_checks_test.rs` — old unit seeds for `05/06/07/08`
- `.worklogs/2026-03-23-120029-complete-garde-family.md` — immediate prior family pattern
- `.worklogs/2026-03-23-120846-harden-garde-audit-findings.md` — recent family hardening style and fail-closed baseline

## Open Questions / Future Considerations
- `RS-TEST-08` currently checks cached pre-commit hook files, not a future decomposed hook script model. When Rust/shared hook families are implemented, this rule should likely validate against the canonical hook facts instead of scanning cached script lines itself.
- `RS-TEST-17` currently treats `_` wildcards anywhere in the `matches!` pattern as weak. That is intentionally harsh for breadth-first implementation, but later hardening may want more nuanced payload-vs-shape semantics.
- `RS-TEST-12` currently expects timeouts under `[profile.default]`. If the project standardizes a different nextest profile strategy, this rule and generator/repo policy should be aligned together.

## Key Files for Context
- `AGENTS.md` — project-wide working contract, worklog rules, and checker architecture constraints
- `.plans/todo/checks/rs/test.md` — current `rs/test` rule inventory and implementation notes
- `apps/guardrail3/crates/app/rs/checks/rs/test/mod.rs` — family orchestrator and rule fan-out
- `apps/guardrail3/crates/app/rs/checks/rs/test/facts.rs` — root discovery, hook discovery, and fail-closed input gathering
- `apps/guardrail3/crates/app/rs/checks/rs/test/parse.rs` — syn-based test AST analysis
- `apps/guardrail3/crates/app/rs/checks/rs/test/test_support.rs` — rule-test helpers and temp-tree scaffolding
- `apps/guardrail3/crates/app/rs/validate/test_checks.rs` — old legacy baseline for comparison only
- `apps/guardrail3/crates/app/rs/validate/test_quality_checks.rs` — old legacy baseline for comparison only
- `.worklogs/2026-03-23-120846-harden-garde-audit-findings.md` — nearest prior hardened family worklog

## Next Steps / Continuation Plan
1. Run an adversarial audit over `rs/test` against the updated plan and old attack seeds, focusing on:
   - inline-test bypasses in `src/`
   - `#[ignore]` reason edge cases
   - `matches!` payload wildcard false positives/negatives
   - mutation hook comment-only false positives
2. Start `rs/release` as the last breadth-first Rust family using the same standard:
   - strict rule/file split
   - one test module per rule
   - explicit input-failure rule if needed
3. After `rs/release` exists, begin the cross-family hardening pass:
   - revisit larger-rule tests where sidecars are becoming too dense
   - expand per-rule adversarial coverage
   - decide where rule-specific test directories are preferable to single sidecar files
