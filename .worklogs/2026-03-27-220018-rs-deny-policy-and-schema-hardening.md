# Align g3rs-deny/tokio-full-ban Policy And Harden g3rs-deny/allow-override-channel Schema Checks

**Date:** 2026-03-27 22:00
**Scope:** `.plans/todo/checks/rs/deny.md`, `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`, `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_25_unknown_keys.rs`, `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_25_unknown_keys_tests/*`

## Summary
Updated the active `g3rs-deny/tokio-full-ban` plan text to the stricter policy already enforced by the runtime: `allow-registry` is exact and extra registries are errors. Then fixed `g3rs-deny/allow-override-channel` so wrong-type critical deny containers warn as unsupported schema instead of silently disappearing from validation, and added focused attack coverage for those cases.

## Context & Problem
An adversarial review against the deny plans found two concrete plan/runtime mismatches. First, `g3rs-deny/tokio-full-ban` was implemented as an exact crates.io-only policy while the active plan still described a weaker “must contain crates.io” requirement. Second, `g3rs-deny/allow-override-channel` claimed ownership of “unknown keys / unsupported schema” but the implementation only checked unknown keys inside already-correct table/array shapes, so malformed critical containers were skipped instead of warned.

The user explicitly chose the stricter `g3rs-deny/tokio-full-ban` policy on merit rather than preserving the weaker prose, so the job was to make the plan match the better policy and then close the real `g3rs-deny/allow-override-channel` schema gap before rerunning attacks.

## Decisions Made

### Tighten the written g3rs-deny/tokio-full-ban policy to the existing stricter runtime behavior
- **Chose:** Update the deny plan and follow-up backlog to say `allow-registry` must allow only the accepted crates.io forms and that extra registries are errors.
- **Why:** Extra registries expand the supply-chain trust surface directly; the current runtime and tests already enforced the stricter rule, and the user agreed that the stricter policy is the better one.
- **Alternatives considered:**
  - Loosen the runtime to permit extra registries when crates.io is present — rejected because it weakens the guardrail and creates a broader escape surface.
  - Leave the drift unresolved — rejected because future agents would keep treating the mismatch as an implementation bug.

### Treat wrong-type critical deny containers as g3rs-deny/allow-override-channel schema warnings
- **Chose:** Extend `g3rs-deny/allow-override-channel` to warn on unsupported schema for core sections and nested critical containers such as `[licenses.private]`, `[licenses].exceptions`, `[bans].skip`, `[bans].features`, and `[advisories].ignore`.
- **Why:** The active rule contract explicitly owns unsupported schema, and silently skipping malformed containers is a fail-open path that weakens surrounding rules.
- **Alternatives considered:**
  - Keep wrong-type handling owned only by neighboring rules such as `g3rs-deny/duplicate-entries` and `g3rs-deny/unknown-keys` — rejected because `g3rs-deny/allow-override-channel` explicitly claims broader schema hardening across critical sections.
  - Promote unsupported schema to hard errors — rejected for now because the current rule contract and severity are warning-oriented.

### Add direct attack tests instead of relying on prose-only closure
- **Chose:** Add a dedicated `unsupported_schema.rs` sidecar for `g3rs-deny/allow-override-channel` and prove root-local ownership and family-run attribution for wrong-type containers.
- **Why:** This closes the exact adversarial gap that was found and keeps the tests aligned with the family’s rule-local sidecar pattern.
- **Alternatives considered:**
  - Fold the new cases into existing unknown-key tests — rejected because unsupported schema is a different attack vector than unknown-key drift.

## Architectural Notes
`g3rs-deny/allow-override-channel` now acts as the deny family’s generic schema-hardening backstop for critical sections: if a section exists but its shape is unsupported, the rule emits an explicit warning instead of silently ignoring it. This complements the more specific container/content rules rather than replacing them.

The plan/backlog updates also deliberately collapse ambiguity around `g3rs-deny/tokio-full-ban`. There is now one authoritative contract across docs, runtime, and tests: exact crates.io-only registry allow-list, with the two accepted crates.io URL forms.

## Information Sources
- `.plans/todo/checks/rs/deny.md`
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`
- `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_16_allow_registry_baseline.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_25_unknown_keys.rs`
- prior adversarial review findings produced in-session
- verification commands:
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/deny/Cargo.toml rs_deny_config_25_unknown_keys`
  - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/deny/Cargo.toml --workspace`

## Open Questions / Future Considerations
- `RS-DENY` still has one material fail-open path: malformed `guardrail3.toml` falls back to default profile selection in deny facts, which can silently degrade library-sensitive checks to service defaults.
- The deny hardening matrix still has broader adversarial backlog items even after the `g3rs-deny/allow-override-channel` schema fix, especially around end-to-end parity and mixed-root/profile coverage.

## Key Files for Context
- `.plans/todo/checks/rs/deny.md` — active deny contract after the `g3rs-deny/tokio-full-ban` policy decision
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — backlog notes updated to match the stricter policy
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_16_allow_registry_baseline.rs` — current exact registry policy enforcement
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_25_unknown_keys.rs` — schema-hardening rule that now warns on unsupported critical-section shapes
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_config_25_unknown_keys_tests/unsupported_schema.rs` — adversarial coverage for wrong-type critical containers
- `.worklogs/2026-03-27-214045-rs-deny-stabilization.md` — prior deny family stabilization context

## Next Steps / Continuation Plan
1. Fix the remaining deny facts fail-open on malformed `guardrail3.toml` profile parsing, then add a facts-level regression proving library-sensitive baselines do not silently degrade to service.
2. Re-run an adversarial pass against the deny hardening matrix and close the next highest-signal gaps, especially generator/root parity and mixed-root/profile coverage.
3. Continue the family-by-family stabilization inventory to identify which Rust families still lack the self-hosted family shape or still have open semantic backlog.
