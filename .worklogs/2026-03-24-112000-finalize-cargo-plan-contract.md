# Finalize Cargo Plan Contract

**Date:** 2026-03-24 11:20
**Scope:** `.plans/todo/checks/rs/cargo.md`

## Summary
This pass tightened the `RS-CARGO` family plan so it is much closer to a finished contract instead of a vague migration note. It locked in the clean multi-root ownership pieces, added a dedicated cargo-family input-failure rule, clarified standalone-package shape, and resolved the previously ambiguous policy decisions around lint weakening, resolver policy, and manifest-side escape hatches.

## Context & Problem
The user asked for a careful review of the cargo family plan because it was still the obvious outlier among the Rust family plans. The plan had already been improved earlier in the day, but it still had real defects:
- `RS-CARGO-08` was scoped as a policy-root rule even though resolver is a workspace concern
- the plan required fail-closed behavior but did not give cargo-family input failures their own rule
- standalone-package expectations were still only implied
- profile resolution for `guardrail3.toml` in multi-root repos was not explicit enough
- several rule severities and policies were still written as unresolved `Error/Warn` placeholders

The user explicitly did not want hidden unilateral policy decisions. The right move was to patch only the clearly correct structural gaps first, then discuss the genuinely ambiguous rules in concrete terms before freezing them.

## Decisions Made

### Make multi-root ownership and root-local policy inputs explicit
- **Chose:** Add explicit nested-root / ownership semantics and define root-local `guardrail3.toml` ownership in `cargo.md`.
- **Why:** The family’s biggest historical miss was accidental repo-root-only behavior. The plan needed to say exactly how owned roots are classified and which policy file governs each owned root.
- **Alternatives considered:**
  - Keep ownership wording generic and rely on implementation judgment — rejected because that is how the root-only bug happened.
  - Copy the clippy/deny model without cargo-specific explanation — rejected because cargo has workspace-member semantics the other config families do not.

### Add a dedicated cargo-family input-failure rule
- **Chose:** Introduce `RS-CARGO-14` for malformed owned-root `Cargo.toml`, malformed member `Cargo.toml`, and malformed root-local `guardrail3.toml` when profile-sensitive expectations matter.
- **Why:** The plan already said cargo must be fail-closed, but without a dedicated rule ID the failure surface would remain smeared across unrelated rules and likely duplicate or drift.
- **Alternatives considered:**
  - Let each rule report parse failures independently — rejected because that overloads rule semantics and makes failure reporting inconsistent.
  - Leave input failures undocumented until implementation — rejected because the plan itself should encode fail-closed ownership.

### Freeze only-weaker-fails semantics for lint levels
- **Chose:** Set `RS-CARGO-02` so weaker-than-baseline levels are errors, while stricter-than-baseline settings are accepted silently.
- **Why:** This is the most robust satisfiable option: it prevents agents from weakening policy while not punishing tighter local hardening.
- **Alternatives considered:**
  - Require exact canonical levels — rejected because it would produce unnecessary noise for benign tightening.
  - Warn on stronger-than-baseline levels — rejected because it comments on code that is not actually weakening policy.

### Split edition policy from rust-version / MSRV policy
- **Chose:** Convert the old mixed `RS-CARGO-05` concept into:
  - `RS-CARGO-05` for edition policy
  - `RS-CARGO-15` for `rust-version` / MSRV policy
- **Why:** Edition and MSRV are related but distinct concerns. Splitting them keeps rule ownership crisp and makes both implementation and testing cleaner.
- **Alternatives considered:**
  - Keep one combined metadata rule — rejected because it hides two separate policies behind one result surface.

### Require explicit workspace resolver
- **Chose:** Make `RS-CARGO-08` require explicit `resolver = "2"` or `"3"` for every workspace root, with standalone packages out of scope.
- **Why:** Explicit resolver is easier to audit and harder for agents to bypass by relying on Cargo inference.
- **Alternatives considered:**
  - Allow omission on modern non-virtual workspaces — rejected because it is technically acceptable but weaker policy and less explicit.

### Treat manifest-side escape hatches as hard failures
- **Chose:** Freeze:
  - `RS-CARGO-11` as `Error`
  - `RS-CARGO-12` as `Error`
- **Why:** Missing `clippy::disallowed_macros = "deny"` makes macro bans toothless, and unapproved manifest-level `allow`s are exactly the kind of escape hatch guardrail3 exists to block.
- **Alternatives considered:**
  - Keep them as warnings — rejected because that would surface bypasses without actually preventing them.

## Architectural Notes
The cargo family contract is now much closer to the stronger config-family standard already used by clippy and deny:
- explicit multi-root ownership
- explicit rule applicability by root kind
- explicit root-local policy input ownership
- explicit fail-closed rule surface

It still differs from clippy/deny because cargo is not just a “policy-file coverage” family. It also owns:
- workspace-member inheritance
- member-local weakening checks
- edition/MSRV manifest policy

This means cargo still needs a more involved orchestrator rewrite than clippy/deny, but the plan is now much more trustworthy as the build target for that rewrite.

## Information Sources
- `.plans/todo/checks/rs/cargo.md`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/{mod.rs,discover.rs,facts.rs,inputs.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/{rs_cargo_01_workspace_lints.rs,rs_cargo_04_lint_inheritance.rs,rs_cargo_05_workspace_metadata.rs,rs_cargo_08_resolver.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/{mod.rs,facts.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/deny/{mod.rs,facts.rs}`
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md`

## Open Questions / Future Considerations
- The implementation is still behind the plan. Cargo discovery and inputs are still workspace-shaped in code, so the orchestrator rewrite remains necessary.
- `RS-CARGO-15` is now planned but not implemented. The old mixed metadata rule in code will need to be split cleanly during implementation.
- The cargo family still needs conversion from old `*_tests.rs` sidecars to the rule-specific `*_tests/` directory standard.

## Key Files for Context
- `.plans/todo/checks/rs/cargo.md` — the active cargo family contract after this tightening pass
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/discover.rs` — current root-only discovery that still lags the plan
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/inputs.rs` — current workspace-shaped input model that will need redesign
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — current orchestrator showing which rules and input shapes exist today
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — reference multi-root policy modeling
- `apps/guardrail3/crates/app/rs/checks/rs/deny/facts.rs` — reference multi-root policy modeling with allowed/forbidden config handling
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md` — earlier plan reconciliation pass that set up this cargo follow-up

## Next Steps / Continuation Plan
1. Do not hand `cargo` to a hardening agent yet unless the agent is explicitly tasked with orchestrator/discovery redesign, not just test hardening.
2. When `cargo` work resumes, start with `discover.rs`, `facts.rs`, and `inputs.rs` so the family truly becomes multi-root and root-local.
3. After the ownership rewrite, split the current mixed metadata implementation into `RS-CARGO-05` and `RS-CARGO-15`.
4. Then convert cargo rule tests from `*_tests.rs` to rule-specific `*_tests/` directories and harden them under the attack-vector model.
