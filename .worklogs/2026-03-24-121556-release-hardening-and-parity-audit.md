# Release Hardening And Parity Audit

**Date:** 2026-03-24 12:15
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/release/**`, `.plans/todo/check_review/test_hardening/03-release.md`, `.plans/todo/check_review/test_hardening/13-release-agent-brief.md`, `.plans/todo/check_review/test_hardening/13-release-audit-matrix.md`, `.plans/todo/check_review/test_hardening/13-release-execution-plan.md`

## Summary
Completed the release-family hardening pass under a strict repeated adversarial loop, with every `RS-RELEASE-*`, `RS-PUB-*`, and `RS-BIN-*` rule driven through multiple four-agent attack rounds until findings converged. The work ended with deeper binary workflow semantics, richer release facts, and materially stronger rule-local tests.

This session also audited generator parity for release artifacts. The result is partial parity only: the checker now expects more than the generator currently produces.

## Context & Problem
The release family had already been migrated into the new family layout, but it still needed two things:
1. rule-by-rule adversarial hardening instead of broad family-level confidence
2. a reality check on whether `guardrail3 rs generate` and `rs init` actually scaffold the release artifacts that the release rules enforce

The user explicitly required a slow, exhaustive loop: four attack agents per rule, fix every actionable finding, then repeat on the same rule until only garbage or low-value variants remained. After that, the user asked whether the release generator had parity with the release checks and whether the generated artifacts were actually sufficient for the enforced guardrails.

## Decisions Made

### Run The Release Family Through Strict Per-Rule Adversarial Convergence
- **Chose:** Iterate through every release rule in order with repeated four-agent attack waves, only advancing after convergence.
- **Why:** Family-level testing had already found shared bugs, but the user explicitly wanted rule-by-rule exhaustion. This surfaced several rule-specific and helper-level misses that broad hardening would have left ambiguous.
- **Alternatives considered:**
  - One more family-wide attack sweep — rejected because it does not prove per-rule convergence.
  - Manual local brainstorming without attack agents — rejected because the instruction was explicit and the attack skill was the point.

### Preserve Release Workflow Structure Instead Of Pre-Collapsing It
- **Chose:** Refactor release workflow facts to store parsed workflow analysis rather than summary booleans, then evaluate workflow semantics directly in `RS-RELEASE-05/06/07` and `RS-BIN-01/02`.
- **Why:** The remaining plausible checker bugs were concentrated in workflow matching. Boolean summaries were too lossy for job linkage, target attribution, and publish-context token wiring.
- **Alternatives considered:**
  - Keep bolting more exceptions onto boolean helpers — rejected because the false-positive/false-negative surface was already showing that pattern’s limits.
  - Build a full GitHub Actions execution engine — rejected as out of scope for this pass.

### Tighten Binary Release Semantics Around Real Crate Targeting
- **Chose:** Make binary release detection crate-aware by threading binary target names, publishable binary crate names, and more exact cargo/release-action matching through `facts.rs` and `release_support.rs`.
- **Why:** Adversarial passes kept finding cases where generic `cargo build --release` or generic Linux mentions could accidentally credit the wrong binary crate in multi-binary repos.
- **Alternatives considered:**
  - Leave binary rules repo-global and heuristic — rejected because the adversarial findings were real and fixable.
  - Disable generic build-path positives entirely — rejected because single-binary repos should still get credit for conventional workflows.

### Treat Generator Parity As A Read-Only Audit In This Commit
- **Chose:** Audit generator parity now, but do not fold generator changes into this release hardening commit.
- **Why:** The checker hardening is a coherent unit of work and already large. The parity audit showed real generator gaps, but fixing generation and scaffolding is a separate change set and should not be mixed into this commit without deliberate implementation.
- **Alternatives considered:**
  - Patch generator parity immediately before committing — rejected because it would blur the boundary between checker hardening and generator behavior.
  - Ignore generator parity and just commit — rejected because the user asked for the parity answer explicitly.

## Architectural Notes
The release family now leans much harder into the intended family architecture:
- `facts.rs` owns normalization and preserves richer workflow and crate facts.
- rule files stay narrow and consume typed inputs rather than rediscovering context.
- rule-local test directories now encode both golden behavior and adversarial boundaries.

The biggest release-family semantic upgrades landed in shared helpers and facts, because multiple rules depended on the same mistaken assumptions:
- workflow execution recognition
- workspace inheritance resolution
- publishability semantics
- dependency edge naming/version resolution
- binary crate targeting

Generator parity is currently asymmetric:
- `release-plz.toml` and `cliff.toml` templates are emitted by generate for service profile projects
- `rs init` intentionally does not scaffold them
- there is no current release workflow module in the domain module registry
- generate does not enumerate real publishable packages into `[[package]]`
- generate does not scaffold binary release workflow files or binstall metadata

That means the checker contract is now broader and stricter than the generator/scaffold contract.

## Information Sources
- Release checker implementation:
  - `apps/guardrail3/crates/app/rs/checks/rs/release/facts.rs`
  - `apps/guardrail3/crates/app/rs/checks/rs/release/release_support.rs`
  - all rule files under `apps/guardrail3/crates/app/rs/checks/rs/release/`
- Release handoff and audit docs:
  - `.plans/todo/check_review/test_hardening/03-release.md`
  - `.plans/todo/check_review/test_hardening/13-release-agent-brief.md`
  - `.plans/todo/check_review/test_hardening/13-release-audit-matrix.md`
  - `.plans/todo/check_review/test_hardening/13-release-execution-plan.md`
- Generator and module sources:
  - `apps/guardrail3/crates/domain/modules/release.rs`
  - `apps/guardrail3/crates/domain/modules/mod.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
- Design/spec references:
  - `.plans/todo/checks/rs/release.md`
  - `.plans/2026-03-15-190814-domain-specs-v2.md`
  - `.plans/per-app-config-design/01-rust-config-scoping.md`
- Prior worklogs:
  - `.worklogs/2026-03-24-121223-repair-rust-agent-briefs.md`
  - `.worklogs/2026-03-24-114556-tighten-rust-plan-contracts.md`

## Open Questions / Future Considerations
- The generator parity gap for release is real and still open:
  - `release-plz.toml` still uses placeholder `your-crate-name`
  - no generated `.github/workflows/release.yml`
  - no generated binary release workflow
  - no generated binstall metadata
- `RS-RELEASE-08` checks environment state (`cargo-semver-checks` on PATH), which generation alone cannot fully solve. If parity is a product goal, it needs either setup guidance or bootstrap automation beyond static file generation.
- This commit does not attempt to make generator output satisfy every release rule. It records that mismatch explicitly so the next step can be intentional.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/release/facts.rs` — normalized release facts, including workflow, publishability, dependency, and binary targeting data
- `apps/guardrail3/crates/app/rs/checks/rs/release/release_support.rs` — shared release semantics helpers, especially workflow and binary matching
- `apps/guardrail3/crates/app/rs/checks/rs/release/rs_release_05_release_plz_workflow.rs` — workflow-driven release rule using parsed workflow analysis
- `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_01_binary_release_workflow.rs` — binary workflow rule using crate-aware matching
- `apps/guardrail3/crates/app/rs/checks/rs/release/rs_bin_02_linux_target.rs` — Linux target rule using the same richer workflow path facts
- `apps/guardrail3/crates/domain/modules/release.rs` — current generated release templates
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — current release file generation wiring
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs` — confirms init does not scaffold release files
- `.plans/todo/checks/rs/release.md` — live release checker contract
- `.plans/todo/check_review/test_hardening/03-release.md` — handoff state for the release hardening lane
- `.worklogs/2026-03-24-121223-repair-rust-agent-briefs.md` — nearby planning/doc hygiene context

## Next Steps / Continuation Plan
1. Implement release generator parity in `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` by replacing the static `RELEASE_PLZ_TOML` package placeholder with generated `[[package]]` entries for discovered publishable crates.
2. Add a release workflow module under `apps/guardrail3/crates/domain/modules/` and register it in `apps/guardrail3/crates/domain/modules/mod.rs`, then wire generate to emit `.github/workflows/release.yml` for service-profile Rust projects.
3. Decide whether binary workflow scaffolding belongs in generate. If yes, add a second workflow module for binary release and align it with `RS-BIN-01/02`.
4. Decide whether binstall metadata belongs in generated `Cargo.toml` content or remains an explicit warning-only follow-up. If parity is the goal, generation needs a strategy for `RS-BIN-03`.
5. After generator changes land, add generator-focused tests in `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers_tests.rs` that assert the generated release artifacts actually satisfy the current release checker contract for a representative service workspace.
