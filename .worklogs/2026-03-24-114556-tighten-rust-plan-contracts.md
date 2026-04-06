# Tighten Rust Plan Contracts

**Date:** 2026-03-24 11:45
**Scope:** `.plans/todo/checks/hooks/{shared,rs}.md`, `.plans/todo/checks/rs/{release,deps,test,toolchain,fmt,garde,libarch}.md`

## Summary
This pass raised the remaining weak Rust-side plan docs to the same contract standard as the stronger specimen families. It tightened hook-family scope and ownership, added missing discovery and fail-closed sections to the weaker Rust family plans, fixed stale current-code references and statuses, and turned `libarch` from a design note into a much more explicit rule contract.

## Context & Problem
The user explicitly wanted the assistant’s role here to be planning quality, not implementation parity. The standard was: plans must be precise enough that if a later agent verifies work against them, that verification actually means something.

Using the stronger implemented families (`hexarch`, `code`, `release`, `clippy`, `deny`) as the bar, the remaining weak areas were:
- both hook plans
- `release`
- `deps`
- `test`
- `toolchain`
- `fmt`
- `garde`
- `libarch`

The common defects were:
- stale `Current code` references
- missing implementation-mapping contract sections
- missing discovery / ownership model sections
- missing or fuzzy fail-closed expectations
- hook plan rule wording that no longer matched the actual implemented family semantics
- `libarch` still reading like architecture notes rather than a verifiable family contract

## Decisions Made

### Bring hook-family plans up to the same structural standard as Rust family plans
- **Chose:** Add implementation-mapping, ownership, executable-command, and fail-closed sections to both `HOOK-SHARED` and `HOOK-RS`, and fix stale statuses/rule wording.
- **Why:** The hook plans were the least verification-grade remaining docs. They still mixed old-validator framing with under-specified executable-command semantics and stale “Planned” statuses for already-implemented rules.
- **Alternatives considered:**
  - Leave hook plans as inventory docs until hook implementation work resumes — rejected because the user wanted the plan set itself to be trustworthy now.
  - Only patch statuses — rejected because the bigger problem was missing contract detail, not just stale status text.

### Make mixed-scope families state their scope explicitly
- **Chose:** Add discovery/ownership and fail-closed sections to `release`, `deps`, and `test`.
- **Why:** These families combine repo-root, per-root, and per-package facts. Without that written down, later verification could “pass” the wrong discovery model.
- **Alternatives considered:**
  - Rely on current code or agent memory for scope — rejected because that is exactly the kind of ambiguity the user wants eliminated.

### Freeze the remaining obvious toolchain/fmt contract details
- **Chose:** Tighten `toolchain.md` around channel semantics (`beta`, pinned-nightly, stable, pinned stable) and update the MSRV cross-reference to the split cargo rule. Tighten `fmt.md` so `RS-FMT-CONFIG-01` names its exact owned settings and clarifies where parse failures surface today.
- **Why:** Those plans were no longer missing whole sections, but they still had enough ambiguity around severity/trigger semantics to weaken verification.
- **Alternatives considered:**
  - Leave the smaller config families “thin” because they are simple — rejected because thin plans are still unreliable if they leave key semantics implicit.

### Make `libarch` a contract rather than a sketch
- **Chose:** Add implementation mapping, owned-package classification, exact measurement semantics, fail-closed expectations, and split the grouped rule-intent prose into actual per-rule contracts.
- **Why:** `libarch` was the clearest remaining “design note” rather than a family plan. Since it is planned but not implemented, its plan quality matters even more: later implementation will anchor to this file directly.
- **Alternatives considered:**
  - Wait until implementation starts — rejected because the user specifically wants the planning layer finished before later verification work.

### Clarify remaining garde semantics without reopening its whole design
- **Chose:** Add an explicit severity note to `garde.md` and keep the already-updated extractor/method backlog clearly separated from the current contract.
- **Why:** `garde` was close to grade already; it mainly needed the severity-pair meaning written down so later verification would not overclaim what `Warn/Info` or `Error/Info` means.
- **Alternatives considered:**
  - Leave severity-pair interpretation implicit — rejected because the earlier review already identified that as a verification ambiguity.

## Architectural Notes
- The planning split is now much cleaner:
  - family plans describe what the family owns, how it discovers that scope, and how it fails closed
  - shared architecture docs and hardening docs describe cross-family testing and implementation discipline
- Hook planning now explicitly mirrors the Rust family planning style instead of staying an old-style checklist
- `release`, `deps`, and `test` now explicitly document their mixed-scope nature so future verification does not quietly assume repo-root-only behavior
- `libarch` now defines:
  - what counts as a library package
  - what the layered shape actually is
  - how thresholds are measured
  - what each planned rule individually means

## Information Sources
- `.plans/todo/checks/hooks/shared.md`
- `.plans/todo/checks/hooks/rs.md`
- `.plans/todo/checks/rs/release.md`
- `.plans/todo/checks/rs/deps.md`
- `.plans/todo/checks/rs/test.md`
- `.plans/todo/checks/rs/toolchain.md`
- `.plans/todo/checks/rs/fmt.md`
- `.plans/todo/checks/rs/garde.md`
- `.plans/todo/checks/rs/libarch.md`
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/{mod.rs,facts.rs,hook_shared_01_pre_commit_exists.rs,hook_shared_03_modular_directory_inventory.rs,hook_shared_05_pre_commit_executable.rs,hook_shared_06_script_stats_inventory.rs,hook_shared_07_modular_scripts_inventory.rs,hook_shared_18_executable_command_context_only.rs,hook_shared_21_no_fail_open_wrappers.rs}`
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/{mod.rs,facts.rs,hook_rs_01_fmt_step_present.rs,hook_rs_02_clippy_step_present.rs,hook_rs_06_required_tools_installed.rs,hook_rs_07_duplication_tool_is_cargo_dupes.rs,hook_rs_08_guardrail_validate_staged_present.rs,hook_rs_16_config_changes_trigger_validation.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/release/{mod.rs,facts.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/deps/facts.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/test/facts.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/{rs_toolchain_config_01_channel_components.rs,rs_toolchain_config_02_msrv_consistency.rs}`
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/rs_fmt_config_01_settings.rs`
- agent findings during this session for hook/shared, hook/rs, and the weaker Rust family plans
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md`
- `.worklogs/2026-03-24-112000-finalize-cargo-plan-contract.md`

## Open Questions / Future Considerations
- The hook plans are now much tighter, but they still have implementation lag in the same way some other families do. This work only raised the contract quality.
- `libarch` is now far more explicit, but it is still a planned family with no implementation. The first implementation pass should validate whether any classification/detail assumptions need refinement.
- The two untracked agent-brief files created earlier in the session (`20-cargo-agent-brief.md`, `21-deps-agent-brief.md`) were intentionally left out of this commit because they are handoff utilities, not part of the plan-tightening pass.

## Key Files for Context
- `.plans/todo/checks/hooks/shared.md` — tightened hook-structure contract, now much closer to verification-grade
- `.plans/todo/checks/hooks/rs.md` — tightened Rust hook contract, including command-normalization and rule ownership
- `.plans/todo/checks/rs/release.md` — now explicitly documents mixed repo/crate/edge scope and fail-closed behavior
- `.plans/todo/checks/rs/deps.md` — now explicitly documents per-crate vs per-root ownership and validation-root policy ownership
- `.plans/todo/checks/rs/test.md` — now explicitly documents multi-root discovery and shared validation-root inputs
- `.plans/todo/checks/rs/toolchain.md` — now freezes the remaining channel/MSRV semantics more tightly
- `.plans/todo/checks/rs/fmt.md` — now freezes the exact `RS-FMT-CONFIG-01` owned setting surface
- `.plans/todo/checks/rs/garde.md` — now clarifies severity-pair semantics for later verification
- `.plans/todo/checks/rs/libarch.md` — upgraded from design note toward a true family contract
- `.worklogs/2026-03-24-082050-reconcile-rust-plan-contracts.md` — earlier Rust plan reconciliation pass that set up this follow-up
- `.worklogs/2026-03-24-112000-finalize-cargo-plan-contract.md` — earlier cargo-specific contract tightening from the same day

## Next Steps / Continuation Plan
1. If the user wants the planning layer frozen, review whether the remaining uncommitted handoff utility files (`20-cargo-agent-brief.md`, `21-deps-agent-brief.md`) should be committed separately or left out.
2. Keep using the now-tightened family plans as the source of truth when reviewing agent implementation work, especially for hook families and future `libarch`.
3. When implementation work resumes, distinguish clearly between:
   - plan quality being good enough
   - implementation matching the plan
   The user explicitly wants those kept separate.
