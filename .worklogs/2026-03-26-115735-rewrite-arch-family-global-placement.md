# Rewrite Arch Family Around Global Placement Facts

**Date:** 2026-03-26 11:57
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/**`, `apps/guardrail3/crates/app/rs/placement/**`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/adapters/inbound/cli/init.rs`, `apps/guardrail3/crates/app/commands/src/messages.rs`, `apps/guardrail3/crates/domain/modules/guide.rs`, `fuzz/Cargo.toml`, `.plans/todo/checks/rs/arch.md`, `.plans/todo/check_review/test_hardening/29-arch-agent-brief.md`

## Summary
Rewrote the `rs/arch` family around a shared placement crate so global Rust-root discovery, classification, exclusions, overlap checks, and auxiliary-root markers no longer live inside the family rule module itself. The overloaded old `RS-ARCH-05` coherence bucket was split into separate rules, runtime/config surfaces were tightened to keep `arch` global-only, and the repo now supports explicitly declared auxiliary Rust roots while still rejecting unexpected stray roots.

## Context & Problem
The `arch` family had drifted into three problems at once. First, root placement logic lived privately under the family, even though the user wanted classification/discovery to belong to substrate/orchestrator code rather than to individual rules. Second, `RS-ARCH-05` had become a mixed bucket covering scoped-config rejection, owner-family coherence, and fail-closed input behavior, which made the rule model muddy. Third, repo-wide validation was too blunt: anything outside `apps/*` and `packages/*` was treated as forbidden `other`, even though the repo legitimately contains development-only Rust roots like `fuzz/` and ephemeral copies under `.claude/worktrees/`.

The user clarified the intended policy:
- `arch` is global, not app/package scoped
- rules should detect violations, not own classification
- `other` roots still need to be checked
- legitimate non-product Rust roots need an explicit policy
- auxiliary-marked roots should be visible in CLI/report output

Those constraints forced a rewrite rather than another local patch on the old family.

## Decisions Made

### Extract placement into a shared crate
- **Chose:** Move root discovery, path classification, overlap logic, workspace-root context handling, structural exclusions, and auxiliary-role resolution into `guardrail3-app-rs-placement`.
- **Why:** Classification is substrate/orchestrator work, not a rule’s job. This also gives future families a reusable Rust-root ownership model without depending on `arch` internals.
- **Alternatives considered:**
  - Keep `rust_root_placement.rs` private under `arch` and only clean up the rules — rejected because it preserves the wrong ownership boundary.
  - Push placement logic into runtime — rejected because the logic is Rust-root/domain specific, not generic family dispatch.

### Keep `arch` global-only and reject scoped `arch` config explicitly
- **Chose:** Treat `arch` as a repo-global family and surface scoped app/package `arch` config as its own rule violation.
- **Why:** `arch` decides the repo-wide ownership map. Discovery and reporting for overlap, ambiguous ownership, and misplaced roots do not localize cleanly to one app/package boundary.
- **Alternatives considered:**
  - Allow scoped `arch` toggles and filter findings by applicability — rejected because it creates contradictory ownership policies inside one repo.
  - Leave scoped config accepted but ignored — rejected because it creates misleading dead config.

### Split the old coherence bucket into multiple rules
- **Chose:** Replace the old `RS-ARCH-05` with:
  - `RS-ARCH-05` scoped `arch` config forbidden
  - `RS-ARCH-06` owner-family enablement coherence
  - `RS-ARCH-07` required inputs fail closed
  - `RS-ARCH-08` declared auxiliary roots inventory/info
- **Why:** One rule should represent one local violation type. The old bucket mixed unrelated concerns and hid what the family was actually proving.
- **Alternatives considered:**
  - Keep one “coherence” rule and only rename it — rejected because it would still bundle multiple violation types under one rule id.
  - Split only config vs input failures and leave auxiliary reporting implicit — rejected because the user explicitly wanted auxiliary roots surfaced.

### Use explicit auxiliary-root metadata instead of path allowlists
- **Chose:** Support `[package.metadata.guardrail3] arch_role = "auxiliary"` and `[workspace.metadata.guardrail3] arch_role = "auxiliary"` in Cargo manifests.
- **Why:** The repo can legitimately contain arbitrary development-only Rust roots. Trying to enumerate every acceptable path would be brittle and never complete.
- **Alternatives considered:**
  - Hardcode a wider path allowlist such as `fuzz/**`, `tools/**`, `xtask/**` — rejected because the user explicitly pointed out that arbitrary legitimate dev-only roots can appear.
  - Allow all `other` roots — rejected because it defeats the purpose of catching accidental stray Cargo roots.

### Exclude obvious non-architecture roots structurally
- **Chose:** Exclude fixtures, snapshots, `target/`, and `.claude/worktrees/` from live root discovery.
- **Why:** These are not “allowed others”; they are not architecture at all and should never enter `RS-ARCH` evaluation.
- **Alternatives considered:**
  - Treat them as auxiliary — rejected because ephemeral/generated copies should not require explicit metadata.
  - Leave them visible and require user config to silence them — rejected because they are structural noise, not policy exceptions.

## Architectural Notes
The new shape is:

`ProjectTree` / runtime
-> shared placement crate (`guardrail3_app_rs_placement`)
-> normalized `arch` facts
-> minimal rule inputs
-> pure rule checks

This keeps `arch` aligned with the broader checker architecture: substrate performs discovery/classification once, the family orchestrator binds facts into the smallest useful rule inputs, and each rule only asserts one violation type.

The placement crate also introduced workspace-root contextual classification so validating `apps/guardrail3` directly still understands its inner crates as belonging to the `apps/guardrail3` app zone even though the repo root is not the validation root.

Auxiliary roots are intentionally reported as info-level `RS-ARCH-08` findings rather than silently disappearing. That keeps the repo map visible without conflating declared exceptions with violations.

## Information Sources
- User requirements from this session, especially:
  - `arch` should remain global
  - rules should not own classification
  - `other` still needs enforcement
  - arbitrary legitimate dev-only roots make path allowlists unsuitable
  - auxiliary markers should be visible in CLI output
- `apps/guardrail3/crates/app/rs/families/test/README.md` — the broader test-architecture direction that reinforced keeping rule semantics distinct from discovery/orchestration
- Prior branch state and worklogs:
  - `.worklogs/2026-03-26-101556-rs-test-exact-self-hosting.md`
  - `.worklogs/2026-03-26-110506-rs-test-assertions-contract.md`
  - `.worklogs/2026-03-26-112409-rs-test-direct-component-shape-only.md`
- Live files touched for this rewrite:
  - `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/families/arch/src/inputs.rs`
  - `apps/guardrail3/crates/app/rs/runtime.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
  - `fuzz/Cargo.toml`

## Open Questions / Future Considerations
- The full product `cargo run ... rs validate . --family arch --format json` path was previously proven before the latest auxiliary-reporting change, but a fresh proof is currently blocked by unrelated `rs/test` family compile errors elsewhere on the branch. The `arch`, runtime, and init crate-level proofs are green.
- If future non-product Rust roots need richer categorization than a single auxiliary role, the placement crate is now the correct layer to extend rather than re-expanding `arch` rules.
- The repo may still need a broader policy decision for how auxiliary/info findings should appear in default human CLI output versus JSON/inventory modes.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/placement/src/lib.rs` — shared placement crate entrypoint and exported ownership model
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — live-root discovery, exclusions, and auxiliary-role extraction
- `apps/guardrail3/crates/app/rs/placement/src/classification.rs` — app/package/auxiliary/unexpected-other classification
- `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs` — family fact normalization and fail-closed config handling
- `apps/guardrail3/crates/app/rs/families/arch/src/inputs.rs` — one-input-per-violation-type fan-out after the rule split
- `apps/guardrail3/crates/app/rs/families/arch/src/lib.rs` — orchestrator dispatch across `RS-ARCH-01` through `RS-ARCH-08`
- `apps/guardrail3/crates/app/rs/families/arch/README.md` — updated user-facing contract for global placement, auxiliary roots, and split rules
- `apps/guardrail3/crates/app/rs/runtime.rs` — global-only applicability behavior for `arch`
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime proof for `arch` dispatch and scoped-config reporting
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs` — init proof that generated config keeps `arch` global
- `.plans/todo/checks/rs/arch.md` — updated family plan/handoff state

## Next Steps / Continuation Plan
1. Fix the unrelated `rs/test` compile breakage currently blocking the fresh product `cargo run ... rs validate . --family arch --format json` proof:
   - inspect `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
   - inspect `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/facts.rs`
   - restore a green product build without widening this `arch` batch
2. Re-run the product-level `arch` validation command against repo root and confirm the live JSON output now includes `RS-ARCH-08` info for `fuzz/Cargo.toml`.
3. Decide whether auxiliary info should remain visible in normal CLI output or be moved behind inventory/report formatting only, but keep the rule itself intact.
4. If future families need Rust-root placement facts, depend on `guardrail3-app-rs-placement` directly rather than copying `arch`-local logic.
