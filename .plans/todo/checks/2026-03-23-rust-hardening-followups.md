# Rust Hardening Follow-ups From Archived Audit Sweep

**Date:** 2026-03-23
**Scope:** Cross-family Rust hardening backlog extracted from archived `tests_guardrails.md` and `audit/`

## Purpose

The top-level audit backlog and test-guardrail notes have been reviewed against the current Rust-only codebase and active `checks/rs` plans.

Most of that material is now historical and belongs in `legacy/`.

This file keeps only the still-live Rust/shared follow-up work that did **not** already have clear ownership in an active family plan.

## Live follow-up items

### 1. Rust mutation-hook contract is still under-specified

- The active `RS-TEST-08` / hook planning only proves coarse hook presence.
- The older mutation-hook note was stricter and still has value for Rust:
  - scoped `cargo mutants --in-diff`
  - explicit timeout/fail behavior
  - surviving mutants block commit
- This should be pulled into:
  - `checks/rs/test.md`
  - `checks/hooks/shared.md`
  - `checks/hooks/rs.md`

### 2. Self-validation / negative-testing backlog is still largely unowned

- The old test/self-validation audit still leaves one real meta-quality gap:
  - guardrail3 should have explicit negative/self-validation coverage goals
  - mutation-resistance hardening of per-rule tests should be an intentional later phase, not accidental
- This is a project-level hardening backlog item, not a single-family rule.

### 3. Canonical drift protection for fmt/toolchain is weaker than cargo/deny

- `rs/fmt` and `rs/toolchain` still rely on hardcoded canonical expectations without an explicit active-plan note or consistency-test requirement tying them to generated modules.
- Add a later hardening pass similar in spirit to the stronger cargo/deny canonical-drift handling.

### 4. Whole-type `#[garde(skip)]` ownership is still unclear

- Current `rs/code` planning/implementation clearly owns field-level `#[garde(skip)]`.
- Old source-scan audit identified whole-struct/type `#[garde(skip)]` as a bypass surface.
- That broader ownership needs to be made explicit, likely in `rs/code` and/or `rs/garde`.

### 5. Finish removing raw filesystem/direct-`exists()` behavior from surviving legacy Rust paths

- Multiple audit batches converged on the same residual debt:
  - there are still old `app/rs/validate/*` paths using direct filesystem checks and legacy bypass-prone patterns
- The new families already use `ProjectTree`, but the migration-closure criterion is not written down clearly.
- Active improvement target:
  - either finish migrating the remaining legacy Rust validation paths away from direct fs access
  - or explicitly retire/delete them

### 6. Deny generation still appears to drift from effective profile selection

- Per-agent audit finding:
  - `deny.toml` generation still uses workspace-level profile in places where per-app/per-root effective profile should drive the baseline.
- This is generator/checker contract debt and should be fixed or explicitly planned.

### 7. Hook generation still looks inconsistent around `workspace_root`

- Full generate path and narrower generate/install paths do not appear to share one consistent `workspace_root` contract.
- This is a live Rust/shared hook generation bug, not historical noise.

### 8. Embedded module registry completeness is still incomplete

- At least one Rust-relevant generated module (`DENY_BANS_LIBRARY_IO`) is still missing from registry exposure (`list-modules` / `show-module` surface).
- This should become an explicit generator/module-system backlog item.

### 9. CLI/reporting domain routing is stale relative to the current Rust family model

- User-facing validate/report routing still reflects the older coarse domain split more than the actual Rust family inventory.
- This includes stale category/domain treatment around Rust families and hook-related flags.
- Needs an explicit reconciliation pass so the CLI contract matches the current Rust-only architecture.

### 10. Shared hook prerequisite-tool diagnostics are still incomplete

- Audit found one narrow but real gap:
  - explicit prerequisite diagnostics for `git` / `cargo` in hook validation/generation paths are not clearly owned in current hook plans
- This belongs in shared/Rust hook planning rather than in archived audit notes.

### 11. Repo-wide hardening rule about semantic parsing vs raw substring matching should remain explicit

- Archived TS/source/hook audits all converged on one principle that still matters for Rust/shared work:
  - semantic rules should prefer AST/structured parsing
  - raw substring matching should be limited to comments/inventory surfaces only
- Some active family plans already say this, but the repo-wide closure criterion is still only implicit.

## Archived source material this file replaces

- `.plans/todo/tests_guardrails.md`
- `.plans/todo/audit/`

Those notes should remain available only as historical/adversarial reference after archival.
