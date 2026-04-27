# Complete RS-DEPS Family

**Date:** 2026-03-23 10:46
**Scope:** `.plans/todo/checks/rs/deps.md`, `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`, `apps/guardrail3/crates/app/rs/checks/rs/deps/*`

## Summary
Implemented the new `rs/deps` family in the strict checker architecture, including all planned rules, one production file per rule, and one rule-specific test file per rule. Tightened the family after an adversarial audit to avoid fail-open behavior around missing allowlists, hybrid root crates, wildcard `.gitignore` patterns, and workspace member discovery.

## Context & Problem
The Rust config families and the first source-heavy families were already migrated into the new architecture, but `rs/deps` was still missing. The old dependency checks were split across `dependency_scan.rs` and `dependency_allowlist.rs` and were both stale and too weak for the current direction: they skipped `workspace = true`, had no per-section allowlist rules, and did not model input failure surfacing through typed facts.

The immediate goal was breadth-first completion of the remaining Rust families without compromising the structural rules we established:
- one rule per file
- one rule-specific test module per rule
- real `ProjectTree`-driven discovery
- meaningful adversarial tests instead of grouped happy-path family tests

## Decisions Made

### Recast RS-DEPS Around Policy, Not Just Tool Presence
- **Chose:** Expanded the plan from the old mixed 9-rule form into an 11-rule family covering tool availability, dependency allowlists, lockfile policy, and input/parse failure surfacing.
- **Why:** The old rules were too close to legacy implementation details and missed current hardening requirements such as `workspace = true` allowlist enforcement and `Cargo.lock`/`.gitignore` policy.
- **Alternatives considered:**
  - Keep the old 9-rule plan and implement it literally — rejected because it still referenced deleted/stale policy like `CLAUDE.md` and did not cover the current dependency policy surface.
  - Fold lockfile policy into `deny` or `release` — rejected because `RS-DEPS` is the family that owns dependency-policy preconditions and external tooling expectations.

### Keep Allowlist Enforcement Silent When No Allowlist Exists
- **Chose:** `g3rs-deps/dependencies-allowlisted..07` now run only when a crate actually has an `allowed_deps` policy; missing allowlists are handled separately by `g3rs-deps/library-allowlist-present` for library-profile crates.
- **Why:** An adversarial audit caught that the first pass treated `allowed_deps = None` as “all dependencies unauthorized,” which flooded service/binary crates with false findings and was stricter than the plan.
- **Alternatives considered:**
  - Treat missing allowlists as “empty allowlist” for every crate — rejected because it makes the family unusably noisy and contradicts the plan’s separate library-only coverage warning.
  - Skip allowlist coverage warnings entirely — rejected because library crates still need explicit least-privilege pressure.

### Treat Hybrid `[workspace]` + `[package]` Roots As Real Dependency Roots
- **Chose:** Included hybrid root crates in dependency discovery and coverage.
- **Why:** The first pass only included standalone packages when they were not also workspaces, which silently skipped a common monorepo shape and failed open for `g3rs-deps/dependencies-allowlisted..08`.
- **Alternatives considered:**
  - Keep hybrid roots excluded and rely on workspace members only — rejected because root dependencies and root allowlist coverage would never be checked.
  - Model root hybrid crates as a separate special-case family — rejected because this is a normal `rs/deps` discovery concern, not a new family boundary.

### Harden Discovery Against Silent Skips
- **Chose:** Surfaced additional input failures for workspace members matched by glob patterns but missing `Cargo.toml`, unresolved `workspace = true` references, and parse failures used during dependency root discovery.
- **Why:** The family must not silently skip dependency-policy inputs when the whole point is to make dependency control hard to bypass.
- **Alternatives considered:**
  - Allow discovery to skip malformed or incomplete roots and let other families catch them — rejected because `RS-DEPS-11` is explicitly the family-level fail-closed rule for dependency-policy inputs.

### Strengthen `.gitignore` Detection Beyond Literal `Cargo.lock`
- **Chose:** Detect wildcard ignore patterns such as `**/Cargo.lock`, not just exact `Cargo.lock` and `/Cargo.lock`.
- **Why:** The initial literal matching was an obvious bypass and was caught in local audit before commit.
- **Alternatives considered:**
  - Keep exact-match detection only — rejected because it is too easy to evade with common glob syntax.
  - Fully reimplement gitignore semantics — rejected for now as unnecessary complexity; the current hardening step catches the common dangerous cases.

## Architectural Notes
`rs/deps` follows the same new-family pattern as the completed Rust families:
- `mod.rs` orchestrates fact collection and fans out minimal typed inputs
- `facts.rs` owns discovery from `ProjectTree`, policy resolution from `guardrail3.toml`, and dependency entry normalization
- `inputs.rs` contains one typed input shape per rule category
- every `RS-DEPS-*` rule has its own production file and its own test file
- `test_support.rs` provides typed fact builders and `ProjectTree` fixtures, but does not hide rule predicates or bundle rule behavior

The family deliberately takes `(&ProjectTree, &dyn ToolChecker)` rather than only `&ProjectTree`, because tool-installation rules fundamentally depend on the outbound tool port. There is still no single top-level new-architecture Rust-family dispatcher enforcing a uniform family signature, so this does not create an architectural conflict yet.

## Information Sources
- `.plans/todo/checks/rs/deps.md` — reconciled target rule inventory for `RS-DEPS`
- `apps/guardrail3/crates/app/rs/validate/dependency_scan.rs` — old tool-presence dependency checks
- `apps/guardrail3/crates/app/rs/validate/dependency_allowlist.rs` — old allowlist behavior
- `apps/guardrail3/tests/unit/dependency_allowlist_test.rs` — legacy adversarial dependency cases, especially the old `workspace = true` assumptions
- `apps/guardrail3/crates/domain/config/types.rs` — `GuardrailConfig` / `CrateConfig` shape for profile and allowlist resolution
- `apps/guardrail3/crates/domain/project_tree.rs` — `ProjectTree` helper APIs used for workspace/member discovery
- prior family implementations under `apps/guardrail3/crates/app/rs/checks/rs/{cargo,clippy,deny}`

## Open Questions / Future Considerations
- Root-level `allowed_deps` is still not representable in the current config schema. `rs/deps` now behaves consistently with that limitation, but root package allowlist support would require a config-model change, not just checker tweaks.
- The `.gitignore` detection is hardened but not a full gitignore parser. If future audits find bypasses through more complex patterns, that should be tightened in `facts.rs`.
- `rs/deps` now has meaningful adversarial rule tests, but this is still the breadth-first pass. It will need a later depth pass just like the other families.

## Key Files for Context
- `AGENTS.md` — current project direction, architecture constraints, and test/file layout rules
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker pipeline and family design rules
- `.plans/todo/checks/rs/deps.md` — canonical `RS-DEPS` rule inventory and semantics
- `apps/guardrail3/crates/app/rs/checks/rs/deps/mod.rs` — `rs/deps` orchestrator and rule fan-out
- `apps/guardrail3/crates/app/rs/checks/rs/deps/facts.rs` — dependency-policy discovery, policy resolution, and fail-closed input handling
- `apps/guardrail3/crates/app/rs/checks/rs/deps/test_support.rs` — typed fact/test helpers for the family
- `.worklogs/2026-03-23-100722-complete-hexarch-family.md` — previous family-completion checkpoint directly before `rs/deps`

## Next Steps / Continuation Plan
1. Start `rs/garde` from the same strict baseline: one rule per file, one rule-specific test module per rule, real typed facts from `ProjectTree`, and no grouped family tests.
2. Before implementing `rs/garde`, reconcile its plan against the old validation code and any existing tests, the same way `rs/deps` was reconciled before coding.
3. Keep the breadth-first approach for the remaining families (`garde`, `test`, `release`), but continue doing one adversarial audit pass before each family commit so structural and semantic shortcuts do not accumulate again.
