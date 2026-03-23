# Complete Hexarch Family

**Date:** 2026-03-23 10:07
**Scope:** `.plans/todo/checks/rs/hexarch.md`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/*`

## Summary
Implemented the new `rs/hexarch` family end-to-end in the new checker architecture. This included the structural `01..12` rules, the dependency/policy/source-content `13..25` rules, and a full one-rule/one-test sidecar layout with migrated coverage from the old hexarch structural tests.

## Context & Problem
The Rust check migration had already completed config families plus `rs/code`, leaving `rs/hexarch` as the largest remaining architecture family. The old hexarch behavior was split between the old structural checker (`rs_arch_01`) and the older dependency checker (`hex_arch_checks.rs`), with a large test corpus in `tests/unit/rs_arch_01` and `tests/unit/test_hex_arch_checks.rs`.

The user explicitly required that:
- old hexarch tests be accounted for
- relevant old cases be migrated
- the new family be at least as harsh as the old behavior
- the implementation follow the strict architecture rules already established in this repo:
  - one rule file per rule
  - one sidecar test file per rule
  - no grouped rule modules
  - no grouped family test files

## Decisions Made

### Structural half uses the old golden fixture as the baseline
- **Chose:** Build `RS-HEXARCH-01..12` against the old `tests/fixtures/r_arch_01/golden` fixture and port the sharp edge cases into new per-rule sidecar tests.
- **Why:** The old structural checker had the harshest and most battle-tested behavior in the repo. Reusing the same golden fixture keeps the new family honest and avoids a silent softening during migration.
- **Alternatives considered:**
  - Recreate smaller synthetic fixtures for all structure rules — rejected because it would weaken coverage of nested hex, inner-hex recursion, and non-double-fire behavior.
  - Leave the old tests in place and only add smoke tests for the new family — rejected because that would not actually validate the new architecture.

### Split structural and dependency/source facts instead of one giant helper blob
- **Chose:** Keep `facts.rs` for structural tree facts and add `dependency_facts.rs` plus `source_facts.rs` for the second half of the family.
- **Why:** The structural rules and dependency/source rules have different inputs and different failure modes. Splitting the fact collectors made the orchestrator readable without violating the one-rule-per-file contract.
- **Alternatives considered:**
  - Put all collection logic into one oversized `facts.rs` — rejected because it would become a second monolith immediately.
  - Push dependency parsing into individual rule files — rejected because it would violate the family orchestrator/input design.

### Preserve old structural edge semantics even when they are non-obvious
- **Chose:** Keep the old distinctions around:
  - `crates/` missing vs `.gitkeep`-only
  - `RS-HEXARCH-04` vs `05` ownership for files-only containers
  - inner-hex recursion
  - `src/` only banned at app root, not for inner hex
- **Why:** These were exactly the edge cases the old `rs_arch_01` tests existed to pin down. The migration goal was to keep the harsh behavior, not to “simplify” it.
- **Alternatives considered:**
  - Simplify the structural rules to a more uniform directory-shape checker — rejected because it would regress behavior the old tests were explicitly protecting.

### Implement dependency-direction rule precedence explicitly
- **Chose:** Split the dependency-direction space into distinct rule ownership:
  - `13` explicit non-dev, non-target path violations
  - `17` workspace-inherited path violations
  - `18` renamed dependency violations
  - `20` dev-dependency violations
  - `25` target-specific violations
- **Why:** The hardening rules overlap unless precedence is explicit. Without that split, the same edge would fire multiple rule IDs or the wrong ID.
- **Alternatives considered:**
  - Let one generic dependency-direction rule handle all cases — rejected because it would collapse the plan’s intended separation and make configuration/reporting weaker.

### Treat `RS-HEXARCH-15` as per-app boundary config, not per-member spam
- **Chose:** Model `15` as “missing `rust.apps.<app>` boundary configuration” for app roots.
- **Why:** The older checker text talked about unconfigured workspace members, but the current repo direction is app/package-root policy. Emitting the same warning once per internal crate would be noise, not signal.
- **Alternatives considered:**
  - Warn once per workspace member crate — rejected because it duplicates the same app-level omission over and over.
  - Drop the rule entirely — rejected because missing app-boundary config still weakens policy enforcement.

### Use synthetic tree tests for dependency rules and real FS tests for structural/source behavior
- **Chose:** Add synthetic `ProjectTree` helpers in `hexarch/test_support.rs` for dependency graph rules, while structural rules continue to use real tempdir fixture copies and source rules use direct source-fact inputs.
- **Why:** Dependency rules are easier to attack with tiny isolated trees; structural recursion and source file walking are better exercised through real filesystem state.
- **Alternatives considered:**
  - Use tempdirs for every rule — rejected because tiny dependency cases become noisy and slow.
  - Use only direct input construction everywhere — rejected because it would stop testing the orchestrator/fact collection for the structural half.

## Architectural Notes
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/mod.rs` now orchestrates two fact domains:
  - `facts.rs` for `01..12`
  - `dependency_facts.rs` and `source_facts.rs` for `13..25`
- The family still follows the project contract:
  - one production file per rule
  - one sidecar test file per rule
  - shared family support only in non-rule helper files (`facts.rs`, `inputs.rs`, `test_support.rs`, `dependency_facts.rs`, `source_facts.rs`)
- The dependency fact collector resolves:
  - workspace members
  - workspace-inherited dependencies
  - renamed/package dependencies
  - patch/replace path overrides
  - same-layer cycles
  - app-boundary membership
- The source fact collector uses `syn` to collect:
  - public trait counts
  - impl counts
  for `RS-HEXARCH-22` and `23`

## Information Sources
- `.plans/todo/checks/rs/hexarch.md`
- `apps/guardrail3/crates/app/rs/validate/arch/rs_arch_01/*`
- `apps/guardrail3/crates/app/rs/validate/hex_arch_checks.rs`
- `apps/guardrail3/tests/unit/rs_arch_01/*`
- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
- `apps/guardrail3/tests/fixtures/r_arch_01/golden`
- prior worklogs:
  - `.worklogs/2026-03-22-224446-fix-rs-code-audit-gaps.md`
  - `.worklogs/2026-03-22-215957-finish-leftover-rust-check-test-tightening.md`

## Open Questions / Future Considerations
- The adversarial explorer agents did not return concrete findings before commit time. This family was verified locally, but it still deserves a follow-up audit against both the old dependency tests and the current plan.
- `RS-HEXARCH-15` is intentionally reinterpreted as app-boundary configuration rather than per-member config spam. If the product wants a different boundary model later, this rule should be revisited explicitly rather than drifting.
- The second-half dependency/source tests are currently smaller than the structural corpus. They cover rule ownership and major branches, but can still be expanded with more adversarial fixtures.

## Key Files for Context
- `AGENTS.md` — repo-wide active architecture and workflow rules
- `.plans/todo/checks/rs/hexarch.md` — current hexarch inventory and rule semantics
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/mod.rs` — family orchestrator for `01..25`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — structural fact collection for `01..12`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/dependency_facts.rs` — dependency/policy graph collection for `13..21`, `24`, `25`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/source_facts.rs` — AST-derived source stats for `22` and `23`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/test_support.rs` — family-local fixture/synthetic-tree helpers
- `apps/guardrail3/tests/unit/rs_arch_01/mod.rs` — old structural test index
- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs` — old dependency-direction and boundary tests
- `.worklogs/2026-03-23-100722-complete-hexarch-family.md` — this worklog

## Next Steps / Continuation Plan
1. Run an adversarial audit specifically against `rs/hexarch` and compare:
   - the migrated structural behavior (`01..12`)
   - the old dependency behavior (`13..15`)
   - the new hardening semantics (`16..25`)
2. Tighten the second-half test corpus:
   - add more old `test_hex_arch_checks.rs` cases where the new rules are supposed to be at least as harsh
   - add more adversarial cycle / workspace-inherited / alias / cross-app cases
3. Move to the next heavy Rust family after hexarch audit cleanup:
   - `rs/deps`
   - then `rs/garde`
   - then `rs/test`
