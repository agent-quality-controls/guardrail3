# Deny Family Hardening

**Date:** 2026-03-30 20:58
**Scope:** `apps/guardrail3/crates/app/rs/families/deny/**`, `apps/guardrail3/crates/app/rs/family_mapper/{src/rs.rs,src/views.rs}`, `apps/guardrail3/crates/domain/modules/deny.rs`, `deny.toml`, `apps/guardrail3/deny.toml`, `.plans/todo/checks/rs/deny.md`, `.plans/by_family/rs/deny.md`

## Summary
Hardened the `RS-DENY` family against route bleed, malformed policy context, malformed exception channels, and ambiguous registry policy. The family now fails closed on bad profile routing, requires documented exception entries, and standardizes `[sources].allow-registry` on one canonical sparse crates.io value.

## Context & Problem
The family had already been split into an independently compilable unit, but several stability risks remained in the rule surface. Scoped runs could still see deny configs outside the active validation scope, malformed `guardrail3.toml` profile selection could degrade silently to the service profile, and several exception/escape-hatch channels accepted malformed or undocumented shapes as warn/info instead of treating them as policy failures. The user also clarified that if a reason is required, that must be enforced as an error, and later chose to standardize `allow-registry` on a single semantically clear crates.io form.

## Decisions Made

### Route-bound deny config discovery
- **Chose:** Thread validation scope into `RsDenyRoute` and use it in deny facts collection.
- **Why:** Scoped runs must not discover sibling deny configs or ancestor-root policy outside the active subtree.
- **Alternatives considered:**
  - Recompute route bounds inside deny facts — rejected because route ownership belongs in the mapper.
  - Leave validation-root discovery repo-global — rejected because it causes scope bleed.

### Fail closed on invalid deny profile routing
- **Chose:** Treat unknown profile names and conflicting `type`/`profile` selectors in active `guardrail3.toml` as policy-context errors.
- **Why:** Profile-sensitive deny rules cannot safely choose a baseline if profile routing is ambiguous or invalid.
- **Alternatives considered:**
  - Fall back to `service` — rejected because it silently weakens the family and hides bad policy state.
  - Ignore bad selectors and continue with partial checks — rejected because partial semantics are misleading for deny.

### Make documented exceptions truly mandatory
- **Chose:** Turn missing/non-string reasons and bare-string shortcut forms into hard errors for `skip`, advisory `ignore`, license `exceptions`, and ban reasons.
- **Why:** These are explicit policy escape hatches. Allowing undocumented forms defeats the family’s purpose and makes drift invisible in inventory.
- **Alternatives considered:**
  - Keep warn/info severity — rejected per user policy and because inventory is for valid checked state, not missing justification.
  - Allow bare strings for convenience — rejected because they cannot carry a reason.

### Standardize on one canonical crates.io registry form
- **Chose:** Require exactly one `[sources].allow-registry` entry and make it `sparse+https://index.crates.io/`.
- **Why:** One exact value removes ambiguity, reduces churn, and matches the user’s preference for the semantically clearest form.
- **Alternatives considered:**
  - Continue accepting both sparse and GitHub forms — rejected because it leaves unnecessary ambiguity in a guardrail-owned field.
  - Standardize on the GitHub index URL — rejected because it encodes an older implementation detail rather than the intended registry policy.

## Architectural Notes
`RS-DENY` remains a routed policy-root family. It should own deny placement, coverage, shadowing, and deny-file semantics, but not general Cargo workspace structure. The planned move away from validation-root and standalone-package-root policy should therefore split cleanly: deny can narrow its allowed policy roots later, but “no standalone package roots inside a workspace tree” belongs in cargo/placement rules.

## Information Sources
- `AGENTS.md`
- `.plans/by_family/rs/deny.md`
- `.plans/todo/checks/rs/deny.md`
- `apps/guardrail3/crates/app/rs/families/deny/README.md`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts_support.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_19_allow_registry_baseline.rs`
- `apps/guardrail3/crates/domain/modules/deny.rs`
- lean isolated family runs via `cargo check/build/run --no-default-features --features family-deny`
- recent prior worklogs read at session start, especially:
  - `.worklogs/2026-03-30-182333-hooks-shared-command-query-refactor.md`
  - `.worklogs/2026-03-30-172629-workspace-compile-green-checkpoint.md`
  - `.worklogs/2026-03-30-165132-compile-frontier-cleanup-checkpoint.md`
  - `.worklogs/2026-03-30-152626-rs-migration-batch.md`
  - `.worklogs/2026-03-30-152626-plans-and-handoffs-refresh.md`

## Open Questions / Future Considerations
- When the repo moves to workspace-only Rust structure, deny should likely stop allowing validation-root and standalone-package-root policy. That is a deliberate policy narrowing still to be made.
- `apps/guardrail3` still contains standalone Cargo packages (`crates/app/rs/family_mapper/assertions`, `crates/app/rs/runtime/assertions`, `crates/app/rs/validate`) that deny correctly inventories as standalone roots today. Those should be handled by cargo/placement rules, not by hiding them in deny.
- The remaining deny hardening backlog is mostly mixed-root/profile edge coverage and broader parity evidence, not basic family migration.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts.rs` — route-bounded deny discovery and effective coverage facts
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts_support.rs` — deny profile-map validation and coverage helpers
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_19_allow_registry_baseline.rs` — exact sparse crates.io policy
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_23_skip_hygiene.rs` — documented skip exception enforcement
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_24_ignore_hygiene.rs` — documented advisory-ignore enforcement
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/licenses/rs_deny_17_license_exceptions_inventory.rs` — documented license exception enforcement
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — deny route shape including validation scope
- `apps/guardrail3/crates/domain/modules/deny.rs` — generator baseline for deny policy
- `.plans/by_family/rs/deny.md` — current deny-family planning state and boundary notes
- `.plans/todo/checks/rs/deny.md` — live deny rule ledger
- `.worklogs/2026-03-30-152626-rs-migration-batch.md` — migration background

## Next Steps / Continuation Plan
1. Run another adversarial pass against remaining deny gaps, especially mixed-root/profile cases and rule-surface false positives.
2. Decide whether to narrow deny from validation-root/standalone-root policy to workspace-only once cargo/placement owns standalone-package structure errors.
3. Add or tighten tests for any remaining malformed schema surfaces that still only warn because the policy intentionally allows the channel.
