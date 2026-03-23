# Hardening Wave And Cargo Unblockers

**Date:** 2026-03-23 18:29
**Scope:** `.plans/todo/check_review/test_hardening/`, `apps/guardrail3/crates/app/rs/checks/hooks/`, `apps/guardrail3/crates/app/rs/checks/rs/{hexarch,clippy,deny,code,release}/`, `apps/guardrail3/crates/app/rs/checks/rs/release/release_support.rs`, `apps/guardrail3/crates/main.rs`, `apps/guardrail3/tests/*.rs`

## Summary
This batch carried the rule-hardening campaign forward across multiple Rust families, with the heaviest deepening in `hexarch`, and completed broad conversion from legacy `*_tests.rs` sidecars to rule-specific `*_tests/` directories. It also removed three repo-wide Cargo blockers: a recursion-limit compile bug in release shell-command parsing, stale dead-code test helpers, and crate-root unused-dependency failures in the binary and integration-test targets.

## Context & Problem
The repo had moved into a hardening phase where the main job was no longer adding new rules, but proving that existing rules cannot be bypassed cheaply and that their tests reflect intended business semantics instead of narrow happy paths. At the same time, repo-wide Cargo verification was blocked by infrastructure issues unrelated to individual rule logic: generic recursion in `release_support`, stale unused helpers in test-support modules, and `unused-crate-dependencies` failures in test/binary crate roots. The user explicitly wanted stepwise, exhaustive hardening with updated handoff documents that remain usable in a fresh session.

## Decisions Made

### Keep strict Cargo policy and fix local offenders instead of relaxing lints
- **Chose:** Fix the recursion-limit bug structurally, delete genuinely dead helpers, and add explicit crate-root keepalive imports for dependencies required by policy.
- **Why:** The repo’s strict lint posture is useful; broad `allow(...)` or manifest relaxation would hide real drift and change expectations for other agents.
- **Alternatives considered:**
  - Raise `recursion_limit` in the crate — rejected because it masks the compile-shape bug instead of removing it.
  - Add blanket `#[allow(dead_code)]` / disable `unused-crate-dependencies` — rejected because that weakens repo policy globally.
  - Remove `semver` / `serde_yaml` from manifests immediately — rejected because those crates are legitimately used elsewhere and crate-root keepalive imports are the safer minimal fix.

### Treat hardening as semantic attack coverage, not just test-file reshaping
- **Chose:** For `hexarch`, deepen rules by inferred intent first and only keep test reorganization where it materially supports that hardening.
- **Why:** The real risk was fail-open behavior, collector blind spots, and under-specified ownership semantics, not the mechanical folder move by itself.
- **Alternatives considered:**
  - Do a blind `*_tests.rs` to `*_tests/` rename sweep first — rejected because it adds churn without guaranteeing improved semantics.
  - Leave thin rule-logic tests in place — rejected because they do not prove collector/orchestrator behavior against the shared fixture.

### Record family progress in the handoff docs after each tranche
- **Chose:** Keep `01-hexarch.md` and `11-hexarch-agent-brief.md` current as the main lane state, while also updating the broader hardening packet set for release/clippy+deny/hooks.
- **Why:** The user wanted a handoff that can survive a new session without reconstructing context from git history.
- **Alternatives considered:**
  - Rely on code diff only — rejected because too much of this pass was about inferred intent and remaining gaps.
  - Maintain only one monolithic checklist — rejected because family-local lane files are better for sequencing and adversarial review.

## Architectural Notes
The main code-shape fixes in this batch were:
- `release_support::line_has_command` now uses a thin generic wrapper plus a non-generic recursive helper, so nested shell-wrapper parsing no longer generates `F`, `&F`, `&&F`, ... monomorphization blowups.
- `hexarch` source collection now traverses inline `mod { ... }` bodies and records read/parse failures explicitly, so rules `22/23` can fail closed instead of silently skipping malformed source.
- `hexarch` root facts now carry top-level files as well as directories, so `RS-HEXARCH-02` can see stray `crates/*` files that previously evaded the exact-contents rule.
- Repo-wide integration/binary targets now declare `semver` / `serde_yaml` as intentional crate-root keepalive imports, satisfying `unused-crate-dependencies` without changing runtime behavior.

The hardening lane also established a stronger invariant across migrated families:
- one rule per production file
- one rule-specific `*_tests/` module directory
- multiple attack-class files per rule where needed

## Information Sources
- `AGENTS.md`
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`
- `.plans/todo/check_review/test_hardening/13-release-agent-brief.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-agent-brief.md`
- `.plans/todo/check_review/test_hardening/15-hooks-agent-brief.md`
- legacy adversarial corpus:
  - `apps/guardrail3/tests/unit/rs_arch_01/`
  - `apps/guardrail3/tests/unit/test_hex_arch_checks.rs`
- compiler long-type artifacts under `target/debug/deps/*.long-type-*.txt` for tracing the recursion-limit failure
- direct Cargo verification during this batch:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml`
  - `cargo test --lib --manifest-path apps/guardrail3/Cargo.toml`
  - `cargo check --tests --manifest-path apps/guardrail3/Cargo.toml`
  - `cargo fmt --check --manifest-path apps/guardrail3/Cargo.toml`
- many file-level `rustfmt` runs for touched rule/test files while repo-wide verification was partially blocked

## Open Questions / Future Considerations
- Repo-wide Cargo now runs, but many actual test failures remain in hooks, hexarch, clippy, and deny. Those are behavioral/expectation failures, not infrastructure blockers.
- `hexarch` still has remaining depth work in source-policy and dependency-policy rules (`20..23` especially) plus some mixed-combination structural cases.
- Several release/clippy/deny family migrations are staged into the new `*_tests/` directory shape but still need their failing expectations reconciled.
- The binary/integration keepalive imports solve the current `unused-crate-dependencies` policy failures, but if manifests are later split or dependencies removed, those imports should be reevaluated rather than assumed permanent.

## Key Files for Context
- `.plans/todo/check_review/test_hardening/01-hexarch.md` — current hexarch coverage matrix and remaining gaps
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md` — current hexarch handoff with next tranche notes
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/dependency_facts.rs` — fail-closed and same-layer cycle collection logic, including the unlayered-cycle fix
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/source_facts.rs` — source parsing/read failure handling and inline-module traversal
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — root/container/leaf fact collection, now including root files
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents.rs` — exact-contents rule with top-level loose-file visibility
- `apps/guardrail3/crates/app/rs/checks/rs/release/release_support.rs` — recursion-limit fix in shell command parsing
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/test_support.rs` — cleaned dead helper surface
- `apps/guardrail3/crates/app/rs/checks/rs/release/test_support.rs` — cleaned dead helper surface
- `apps/guardrail3/crates/main.rs` — crate-root keepalive imports for binary lint policy
- `apps/guardrail3/tests/unit.rs` — crate-root keepalive imports for unit test target
- `.worklogs/2026-03-23-160918-add-family-hardening-agent-briefs.md` — prior context for the family-by-family hardening packet structure
- `.worklogs/2026-03-23-173255-populate-code-golden-fixture.md` — prior context on the shared golden fixture realism used by later hardening

## Next Steps / Continuation Plan
1. Start from the now-running `cargo test` output and work failures family-by-family rather than treating them as repo infra. The smallest/highest-signal buckets currently exposed are hooks, hexarch, clippy, and deny.
2. For hooks, inspect the failing command-parsing expectations first; the recursion bug is fixed, so remaining failures should be semantic parser/expectation mismatches.
3. For hexarch, reconcile the failing `RS-HEXARCH-01/02` nested-root assertions against the new collector/rule behavior, then continue the queued adversarial depth on `20/22/23`.
4. For clippy and deny, use the newly migrated `*_tests/` directories plus their lane docs to review each failing expectation before changing rule logic.
5. Keep the handoff docs current after each repaired tranche so a fresh session can resume from lane state rather than rediscovering failures from scratch.
