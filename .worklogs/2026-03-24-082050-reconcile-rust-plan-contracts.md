# Reconcile Rust Plan Contracts

**Date:** 2026-03-24 08:20
**Scope:** `.plans/todo/checks/rs/{cargo,clippy,code,deny,fmt,garde,hexarch,test,toolchain}.md`

## Summary
This pass reconciled the active Rust family plans with the current checker architecture and the actual implemented family surface. It removed stale old-validator framing, replaced the obsolete `*_tests.rs` expectation with the current rule-specific test-module-directory standard, and made the remaining unfinished plan gaps explicit, especially the root-only architectural miss in `RS-CARGO`.

## Context & Problem
The user wanted the Rust-side plan set to be trustworthy enough to use as the active contract before continuing family work and parallel hardening. During review, several plans were found to be stale in one of three ways:
- they still described the old validator as the active implementation source
- they still encoded the superseded “one sidecar `*_tests.rs` file per rule” pattern instead of the current per-rule test-module-directory standard
- they hid or blurred real architecture constraints, especially around scope, root ownership, fail-closed behavior, and cross-family dependencies

`cargo` was the most serious case: its code currently behaves as a repo-root-only family, but the real intended contract for Cargo lint policy needs multi-root ownership similar to `clippy` and `deny`. `fmt`, `toolchain`, and `garde` were less broken, but still incomplete as plans because the discovery model and fail-closed expectations were not explicit enough.

## Decisions Made

### Reconcile all active Rust plans to the current test architecture
- **Chose:** Update the non-legacy Rust family plans so they describe rule-specific `*_tests/` module directories rather than `*_tests.rs` files.
- **Why:** The user had already frozen the stronger attack-vector testing model and the old wording was now misleading future implementation and review work.
- **Alternatives considered:**
  - Leave the older wording and rely on the global architecture doc — rejected because the family plans themselves must be self-consistent and implementation-ready.
  - Update only the families currently under hardening — rejected because stale test-architecture wording across the remaining active plans would keep causing drift.

### Demote old validator references to seed material, not current truth
- **Chose:** Rewrite `fmt`, `toolchain`, `garde`, and `test` plan headers so the new-family code under `crates/app/rs/checks/rs/**` is the active implementation surface, with the old validator called out only as historical seed material where relevant.
- **Why:** The user explicitly wanted to know which plans were actually finished and trustworthy. Plans that still pointed at old validator files as “current code” were not trustworthy.
- **Alternatives considered:**
  - Keep the old references for migration context — rejected because that obscures which code is actually authoritative now.

### Make root ownership and fail-closed behavior explicit in the weaker plans
- **Chose:** Add explicit scope / ownership / fail-closed sections to `fmt`, `toolchain`, and `garde`.
- **Why:** The big lesson from the `cargo` miss is that under-specified discovery rules turn into incorrect implementations. The plans need to state whether a family is intentionally root-level, genuinely multi-root, or conditional, and what inputs are allowed to fail the family closed.
- **Alternatives considered:**
  - Treat those details as implementation-time concerns — rejected because the repo has already shown that vague plan contracts become wrong orchestrators.

### Fully specify the intended `cargo` contract even though the implementation still lags
- **Chose:** Rewrite `cargo.md` around an explicit multi-root policy-root model:
  - workspace roots
  - standalone package roots not inside a workspace
  - workspace-only vs policy-root rule applicability
  - fail-closed behavior for malformed `Cargo.toml` / `guardrail3.toml`
  - cross-family dependency on Clippy enforcement
- **Why:** The current code is still effectively root-only. The plan needed to stop encoding that accidental implementation as if it were the intended architecture.
- **Alternatives considered:**
  - Leave `cargo.md` root-shaped until implementation begins — rejected because that would preserve the exact ambiguity that produced the miss.
  - Mark the whole family “unfinished” without writing the real contract — rejected because the point of this pass was to make the plan itself precise enough to build against.

### Record newly identified clean `cargo` hardening rules in the plan
- **Chose:** Add planned `RS-CARGO-CONFIG-07..13` to capture the most concrete clean misses already surfaced by adversarial review:
  - `clippy::disallowed_macros = "deny"` in the canonical lint baseline
  - surfacing explicit `allow` lints outside the approved baseline
  - forbidding member-local `allow` entries when a member uses `[lints] workspace = true`
- **Why:** These are real enforceable gaps and belong in the family contract rather than in transient review chat.
- **Alternatives considered:**
  - Leave them in ephemeral review notes — rejected because they would be easy to lose and they are part of the intended Cargo lint-policy surface now.

## Architectural Notes
- `fmt` and `toolchain` are now explicitly documented as repository-root families:
  - one effective root formatting contract
  - one effective root toolchain contract
- `garde` is now explicitly documented as:
  - multi-root
  - conditional on garde actually being enabled for the owned root
  - dependent on correct covering `clippy.toml` resolution
- `cargo` is now explicitly documented as:
  - intended multi-root policy-root family
  - still under-implemented in current code
  - meaning the plan is now ahead of the implementation on purpose

This split matters because earlier plan ambiguity blurred “root-only by design” with “root-only by accident”.

## Information Sources
- `.plans/todo/checks/rs/fmt.md`
- `.plans/todo/checks/rs/toolchain.md`
- `.plans/todo/checks/rs/garde.md`
- `.plans/todo/checks/rs/cargo.md`
- `.plans/todo/checks/rs/clippy.md`
- `.plans/todo/checks/rs/deny.md`
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/{mod.rs,facts.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/{mod.rs,discover.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/{mod.rs,facts.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/{mod.rs,discover.rs,facts.rs,inputs.rs}`
- `.worklogs/2026-03-23-212157-plan-libarch-family.md`
- `.worklogs/2026-03-23-212741-reactivate-typescript-frontend-planning.md`
- `.worklogs/2026-03-23-220029-hexarch-structural-rule-04-05-hardening.md`

## Open Questions / Future Considerations
- `cargo` is now clear as a plan, but the code is still not there. The orchestrator/discovery layer must be redesigned to match the multi-root policy-root model before the family can be considered implementation-complete.
- `fmt` and `toolchain` are now explicit about being root-level; if the project later wants true multi-root formatting or toolchain policy, that should be a new architecture decision rather than an accidental drift.
- `garde` still has live carry-forward requirements around wrapper-based validation surfaces and expanded extractor bans. Those are now clearly documented, but not fully implemented.

## Key Files for Context
- `.plans/todo/checks/rs/cargo.md` — newly explicit multi-root intended contract for Cargo lint policy
- `.plans/todo/checks/rs/fmt.md` — now explicit about root-level scope and override semantics
- `.plans/todo/checks/rs/toolchain.md` — now explicit about root-level scope and MSRV/toolchain interaction
- `.plans/todo/checks/rs/garde.md` — now explicit about multi-root ownership, gating, and clippy dependency
- `.plans/todo/checks/rs/clippy.md` — already-strong config-family contract used as one reference model
- `.plans/todo/checks/rs/deny.md` — already-strong multi-root config-family contract used as the other reference model
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — the current root-only implementation that motivated the `cargo` rewrite
- `apps/guardrail3/crates/app/rs/checks/rs/garde/facts.rs` — reference for a properly multi-root Rust family with explicit input-failure handling
- `.worklogs/2026-03-23-212157-plan-libarch-family.md` — recent planning work with the current test-module-directory standard
- `.worklogs/2026-03-23-220029-hexarch-structural-rule-04-05-hardening.md` — recent worklog showing the stronger attack-vector testing direction now expected by plans

## Next Steps / Continuation Plan
1. Implement the `cargo` orchestrator/discovery rewrite so the family actually follows the newly documented multi-root policy-root model.
2. Keep the remaining rule-severity hardening findings for `cargo`, `clippy`, and `deny` attached to their family plans rather than leaving them only in chat review.
3. When reviewing Spark’s `RS-CODE-20` work, compare it against the current `RS-CODE-19` baseline and the new stronger per-rule testing standard, not just whether tests happen to pass.
