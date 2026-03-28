# Stabilize RS-CARGO Root Policy

**Date:** 2026-03-28 12:22
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/adapters/outbound/tool-runner/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/crates/assertions/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/arch/test_support/Cargo.toml`, `apps/guardrail3/crates/domain/project-tree/Cargo.toml`, `apps/guardrail3/crates/domain/validation-model/Cargo.toml`, `apps/guardrail3/crates/ports/outbound/traits/Cargo.toml`, `apps/guardrail3/crates/shared/fs/Cargo.toml`, `apps/guardrail3/crates/domain/modules/mod.rs`, `apps/guardrail3/crates/adapters/outbound/fs/mod.rs`, `apps/guardrail3/crates/adapters/outbound/tool-runner/mod.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs`, `apps/guardrail3/crates/app/rs/ast/src/ast_visitors.rs`, `apps/guardrail3/crates/app/rs/ast/src/extra_visitors.rs`, `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`, `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`

## Summary
Brought `RS-CARGO` to zero on the live `apps/guardrail3` root by completing the workspace policy root metadata and member lint inheritance requirements. Enabling the missing workspace lint baseline also surfaced real compile debt (`missing_debug_implementations`), so this checkpoint includes the necessary public-type `Debug` derives to keep the workspace buildable under the now-enforced policy.

## Context & Problem
The post-hexarch baseline showed `RS-CARGO` as the highest-leverage family to fix first: only 11 live errors, all concentrated in the app-root workspace manifest and a bounded set of member manifests. The missing workspace policy pieces were:

- no workspace-root edition declaration for the policy root
- missing `missing_debug_implementations` in `[workspace.lints.rust]`
- missing `disallowed_macros = "deny"` in `[workspace.lints.clippy]`
- several member manifests missing `[lints] workspace = true`

Once `missing_debug_implementations` was enforced, workspace builds started failing on exported helper structs that had never been required to derive `Debug`. That compile fallout needed to be fixed in the same checkpoint; otherwise `RS-CARGO` would validate cleanly while leaving the repo unable to build under its own lint policy.

## Decisions Made

### Complete the workspace-root policy instead of weakening `RS-CARGO`
- **Chose:** Add the missing root metadata and lint entries directly in `apps/guardrail3/Cargo.toml`.
- **Why:** The family plan already expects these invariants, and the goal of the sweep is to make the repo comply with the guardrails rather than soften the guardrails for the current tree.
- **Alternatives considered:**
  - Remove `missing_debug_implementations` from the cargo baseline — rejected because `RS-CARGO-01` explicitly owns the lint completeness contract.
  - Relax `RS-CARGO-11` back out of the root baseline — rejected because macro bans need the cargo-side enforcement switch to be meaningful.

### Fix member inheritance at the manifests, not by special-casing paths
- **Chose:** Add `[lints] workspace = true` to the eight member crates that were missing it.
- **Why:** `RS-CARGO-04` is correctly enforcing a manifest-side invariant. These crates are ordinary workspace members and should inherit policy the same way as the rest of the graph.
- **Alternatives considered:**
  - Exempt family support crates or low-level domain crates from the rule — rejected because it would reintroduce inconsistent manifest semantics inside one workspace.
  - Hoist those crates out of the workspace — rejected because it would fight the newly-restored single-workspace app boundary.

### Treat compile fallout from `missing_debug_implementations` as real debt
- **Chose:** Add `#[derive(Debug)]` to exported types that became illegal once the workspace lint baseline was actually enforced.
- **Why:** The policy change is intentional. The code needed to meet it. These are simple data/visitor/tool structs where `Debug` is appropriate and low-risk.
- **Alternatives considered:**
  - Add targeted `#[allow(missing_debug_implementations)]` at crate/module level — rejected because that would immediately undermine the point of turning the lint on.
  - Avoid enabling the lint until later — rejected because it would leave `RS-CARGO` red and block the rest of the family sweep.

## Architectural Notes
This checkpoint makes the app-root workspace manifest the real, enforceable source of lint policy again. That matters beyond `cargo` itself:

- `RS-CARGO-11` now makes macro bans enforceable for downstream policy families.
- member manifests are structurally consistent, which reduces hidden per-crate policy drift.
- the workspace build now runs under the same lint baseline the guardrails expect, instead of a weaker accidental baseline.

The `Debug` derive fixes are intentionally narrow: they were only added to exported types where the workspace lint now requires them. No crate-local allow escape hatches were introduced.

## Information Sources
- `.plans/todo/checks/rs/cargo.md`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_05_workspace_metadata.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_11_disallowed_macros_deny.rs`
- live validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family cargo --inventory --format json`
- verification commands:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-cargo --lib`
  - `cargo build --manifest-path apps/guardrail3/Cargo.toml -p guardrail3`
  - `cargo metadata --manifest-path apps/guardrail3/Cargo.toml --no-deps --format-version 1`

## Open Questions / Future Considerations
- The new lint baseline may expose more exported-type `Debug` gaps as additional families are touched. That is acceptable and should be fixed case-by-case rather than backing the lint out.
- `RS-CLIPPY` and `RS-GARDE` are the next natural follow-ons because they directly consume the now-correct cargo-side policy surface.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — app-root workspace policy root
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints.rs` — cargo lint completeness rule
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_04_lint_inheritance.rs` — member inheritance rule
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_05_workspace_metadata.rs` — edition policy rule
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_11_disallowed_macros_deny.rs` — cargo-side macro-enforcement switch
- `.worklogs/2026-03-28-110213-finish-hexarch-workspace-boundary.md` — prior checkpoint that restored valid app-root workspace ownership

## Next Steps / Continuation Plan
1. Move to `RS-CLIPPY` next and fix the root `clippy.toml` policy gaps now that cargo-side enforcement is correct.
2. Re-run `RS-GARDE` immediately after `RS-CLIPPY`, because many garde warnings are downstream of missing clippy bans.
3. Save the giant `RS-TEST`, `RS-CODE`, and `RS-RELEASE` fallout buckets for after the policy-root families are stable.
