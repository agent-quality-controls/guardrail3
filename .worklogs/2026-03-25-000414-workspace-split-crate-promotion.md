# Promote guardrail3 workspace members and Rust family crates

**Date:** 2026-03-25 00:04
**Scope:** `Cargo.toml`, `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/lib.rs`, `apps/guardrail3/crates/main.rs`, `apps/guardrail3/crates/domain/{config,report,modules,project-tree,validation-model}`, `apps/guardrail3/crates/ports/outbound/traits`, `apps/guardrail3/crates/shared/fs`, `apps/guardrail3/crates/adapters/outbound/{fs,report,tool-runner}`, `apps/guardrail3/crates/app/{core,hooks,rs}`, `apps/guardrail3/crates/app/rs/families/*`

## Summary
Converted the pseudo-crate layout inside `apps/guardrail3` into a real inner workspace with concrete shared/domain crates, real Rust family crates, and promoted runtime/adapter crates. The root package remains as the product facade, but it now re-exports real crate owners instead of owning most of the Rust validation stack directly.

## Context & Problem
`apps/guardrail3` was laid out like many crates in the filesystem, but Cargo still treated most of it as one package. That left the root crate carrying runtime orchestration, families, shared fs behavior, tool traits, and adapter code behind module paths instead of real crate boundaries. The user asked to keep splitting the architecture and explicitly allowed temporary breakage to enforce the intended ownership first and repair compile fallout second.

## Decisions Made

### Make the inner workspace real before polishing the facade
- **Chose:** Promote the actual workspace members and fix compile fallout afterward.
- **Why:** The main problem was fake ownership. Real crate boundaries had to exist before import cleanup or test-topology work could mean anything.
- **Alternatives considered:**
  - Keep shrinking the root facade without creating new members — rejected because it preserves the monolith behind re-exports.
  - Split all remaining surfaces in one pass including CLI and TS — rejected because the dependency blast radius would have been harder to recover from cleanly.

### Promote the missing shared owners that family crates depended on
- **Chose:** Make `domain/modules`, `app/core`, `domain/config`, `domain/report`, `domain/project-tree`, `domain/validation-model`, `shared/fs`, and normalized outbound traits real crates.
- **Why:** The family crates were blocked on real shared owners for config/report types, project discovery, module data, filesystem access, and family identity.
- **Alternatives considered:**
  - Let new family crates keep depending on root-only modules — rejected because it would harden the wrong boundary.
  - Defer `domain/modules` until later — rejected because `clippy`/`deny` generation and family facts already depend on it.

### Use crate-local compatibility shims inside family/runtime crates
- **Chose:** Add narrow shim modules inside the promoted crates so existing source under `app/rs/checks/**` could still compile with old `crate::domain::*`, `crate::app::core::*`, `crate::app::rs::checks::hooks::shell`, and legacy AST-helper paths.
- **Why:** This preserved momentum while still moving ownership to real crates. It also kept the write set smaller than rewriting every family source file immediately.
- **Alternatives considered:**
  - Rewrite every family import to direct crate paths in one pass — rejected because it would explode the patch size and risk.
  - Keep the families in the root crate until a perfect import rewrite existed — rejected because the user explicitly wanted the real split first.

### Move duplicate-override logic out of inbound CLI
- **Chose:** Move `deduplicated_override` ownership into `domain/modules`.
- **Why:** `domain/modules` was blocked from becoming a real crate because it still reached back into CLI generation code.
- **Alternatives considered:**
  - Keep a reverse dependency from `domain/modules` into inbound CLI — rejected because shared/domain code must not depend on adapters.
  - Copy the helper into every caller — rejected because it would duplicate behavior and drift.

### Promote outbound adapters and Rust runtime after family split stabilized
- **Chose:** Promote `adapters/outbound/fs`, `adapters/outbound/report`, `app/rs` runtime, and `app/hooks` once the family crates compiled.
- **Why:** These were the next biggest real root-owned execution paths. Moving them reduced root ownership of the live Rust path rather than just adding more helper crates.
- **Alternatives considered:**
  - Promote inbound CLI first — rejected because it has the highest fan-in and would have mixed structural split work with command-surface cleanup.
  - Promote TS next — rejected because TS is not the active direction and would not improve the active Rust architecture first.

## Architectural Notes
- `apps/guardrail3/Cargo.toml` is now the real inner workspace root with concrete members for the Rust substrate, Rust family crates, runtime, hooks, and outbound adapters.
- The repo-root `Cargo.toml` excludes `apps/guardrail3`, which is required for the nested workspace to be legal.
- The root `guardrail3` crate still exists, but it now re-exports real crate owners for:
  - `domain::{config,modules,report}`
  - `app::{core,hooks,rs::runtime}`
  - `adapters::outbound::{fs,report}`
- Rust family crates under `crates/app/rs/families/*` are real workspace members.
- `app/rs/runtime` is now a real crate root at `crates/app/rs/runtime.rs`, with internal shim modules to the family crates and shared owners.
- `app/hooks` no longer depends on the old `app/rs/validate::run_hook_report`; the Rust hook report path is assembled directly from the split hook family crates.

## Information Sources
- `AGENTS.md`
- `.plans/todo/check_review/test_hardening/30-workspace-split-phase1-agent-brief.md`
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`
- recent worklogs:
  - `.worklogs/2026-03-24-151825-add-generate-helpers-profile-test.md`
  - `.worklogs/2026-03-24-151738-harden-rust-hexarch-family.md`
  - `.worklogs/2026-03-24-151655-migrate-rust-deps-family-tests.md`
  - `.worklogs/2026-03-24-151611-harden-rust-garde-family.md`
  - `.worklogs/2026-03-24-151528-harden-rust-code-family.md`
- local compile feedback from:
  - `cargo fmt --manifest-path apps/guardrail3/Cargo.toml --all`
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace --lib`

## Open Questions / Future Considerations
- `adapters/inbound/cli` is still root-owned and is now the highest fan-in remaining product surface.
- `app/rs/validate` still exists as a large legacy stack and still owns the AST helper substrate used by the `code` and `test` family crates via shims.
- `app/ts` remains root-owned legacy validation code and is not yet isolated.
- Root tests are still a separate problem; this commit improves ownership, not test-target isolation.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — inner workspace members and dependency wiring
- `Cargo.toml` — repo-root exclusion that makes the inner workspace legal
- `apps/guardrail3/crates/lib.rs` — current thin product facade/re-export surface
- `apps/guardrail3/crates/app/rs/Cargo.toml` — real Rust runtime crate manifest
- `apps/guardrail3/crates/app/rs/runtime.rs` — Rust runtime crate root and family wiring
- `apps/guardrail3/crates/app/hooks/Cargo.toml` — real hooks crate manifest
- `apps/guardrail3/crates/app/hooks/validate.rs` — hook report path no longer routed through legacy Rust validate
- `apps/guardrail3/crates/app/rs/checks/rs/mod.rs` — Rust family re-exports to real crates
- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs` — hook family re-exports to real crates
- `apps/guardrail3/crates/domain/modules/mod.rs` — shared override-dedup owner after removing CLI backedge
- `apps/guardrail3/crates/ports/outbound/traits/mod.rs` — normalized portable outbound trait surface
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs` — real `ProjectTree` crate owner
- `apps/guardrail3/crates/domain/validation-model/src/lib.rs` — real validation-model crate owner

## Next Steps / Continuation Plan
1. Promote `adapters/inbound/cli` into a real crate and make `main.rs` a thin binary wrapper over that crate.
2. Split the remaining legacy Rust validate substrate by extracting the AST-helper/shell-parser ownership out of `app/rs/validate`, so `code` and `test` families stop depending on compatibility shims.
3. Decide whether `app/ts` should become a real legacy crate boundary or remain deferred because TS is not the active roadmap.
