# Remove RS-CODE-14 Duplicate Clippy Scan

**Date:** 2026-03-27 16:36
**Scope:** `apps/guardrail3/crates/app/rs/families/code`, `apps/guardrail3/crates/app/rs/families/cargo/README.md`, `.plans/todo/checks/rs/code.md`

## Summary
Removed `RS-CODE-14`, which duplicated Clippy's `unwrap_used` and `expect_used` lint coverage in the source-scanning family. Updated the `code` and `cargo` family docs to make the ownership split explicit: Cargo/Clippy policy owns those lints, not `RS-CODE`.

## Context & Problem
During the `RS-CODE` attack review, the repo still showed a very large `RS-CODE-14` bucket. The detector itself was working, but the user explicitly rejected keeping source-analysis for behavior that should be delegated to proper Rust tooling. The code family was duplicating Clippy capability instead of enforcing that the correct lint baseline is configured in Cargo policy.

The existing `cargo` family already requires `clippy::unwrap_used` and `clippy::expect_used` in its expected lint baseline. That made `RS-CODE-14` redundant and architecturally wrong.

## Decisions Made

### Remove `RS-CODE-14` instead of splitting it into more code rules
- **Chose:** delete the entire `RS-CODE-14` runtime rule, assertions module, and sidecar tests.
- **Why:** raw unwrap/expect detection belongs to Clippy, and this repo already treats Cargo lint tables as the source of truth for required Clippy policy.
- **Alternatives considered:**
  - Split `RS-CODE-14` into three source rules (`unwrap`, `expect outside tests`, `weak expect message in tests`) — rejected because the first two duplicate Clippy and violate the repo's “delegate to proper tools” rule.
  - Keep `RS-CODE-14` as-is and merely downgrade severity — rejected because the problem was ownership, not tuning.

### Keep ownership documentation with `RS-CARGO`
- **Chose:** document in `cargo/README.md` that required Clippy lint baseline includes `clippy::unwrap_used` and `clippy::expect_used`.
- **Why:** the enforcement surface is Cargo lint tables, so the family docs should say that explicitly.
- **Alternatives considered:**
  - Move the statement into `RS-CLIPPY` docs only — rejected because the actual enforced baseline lives in Cargo lint tables.
  - Leave the ownership implicit in test fixtures only — rejected because the docs would stay misleading.

### Remove dead parser plumbing instead of leaving orphaned AST helpers
- **Chose:** delete the unwrap/expect parser entrypoints and visitor code from `parse.rs` / `parse/visitors.rs`.
- **Why:** leaving dead parsing paths would keep future drift around a deleted rule and risk false assumptions that the detector still exists.
- **Alternatives considered:**
  - Leave the helper in place “for possible future reuse” — rejected because it preserves dead architecture and weakens the signal that ownership moved out of `RS-CODE`.

## Architectural Notes
This change tightens the division of labor across Rust families:

- `RS-CODE` owns custom AST/source policy that Clippy does not express well.
- `RS-CARGO` owns required lint baseline in `Cargo.toml`.
- `RS-CLIPPY` remains the place for `clippy.toml`-specific policy if test allowances or config knobs ever need enforcement.

This is the same architecture already used elsewhere in the repo: do not duplicate tool-native checks in custom AST scanners unless the tool cannot express the rule.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lint_support.rs` — confirmed `unwrap_used` and `expect_used` are already in the required Clippy deny baseline.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — showed `RS-CODE-14` was still wired into runtime execution.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs`
- `apps/guardrail3/crates/app/rs/families/code/README.md`
- `.plans/todo/checks/rs/code.md`
- Prior worklogs:
  - `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md`
  - `.worklogs/2026-03-27-144818-finish-rs-code-stabilization.md`

## Open Questions / Future Considerations
- The repo still has substantial real `RS-CODE` debt in `RS-CODE-03`, `RS-CODE-04`, `RS-CODE-24`, and `RS-CODE-15`. This change intentionally did not reduce those buckets.
- `cargo` family docs now state the unwrap/expect lint ownership explicitly, but the family root itself is not fully `RS-CARGO`-clean yet. That is a separate stabilization task.
- If the team still wants “test `expect(...)` messages must be useful,” that should be a new custom rule with clearly non-Clippy scope rather than reviving raw unwrap/expect scanning.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — code-family orchestrator; shows the deleted rule slot.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs` — runtime parser surface after removal of unwrap/expect helpers.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` — visitor set after deleting the unwrap/expect AST visitor.
- `apps/guardrail3/crates/app/rs/families/code/README.md` — family-level ownership statement for `RS-CODE`.
- `apps/guardrail3/crates/app/rs/families/cargo/README.md` — family-level ownership statement for required Clippy lint baselines.
- `.plans/todo/checks/rs/code.md` — rule inventory and the explicit rejection note moving unwrap/expect out of source scanning.
- `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md` — immediate backstory from the code-family attack pass.

## Next Steps / Continuation Plan
1. Re-run `rs validate . --family code --inventory --format json` and sample the remaining largest buckets (`RS-CODE-04`, `RS-CODE-03`, `RS-CODE-24`, `RS-CODE-15`) to keep separating validator bugs from real repo debt.
2. Decide whether the next stabilization target is `rs/release`, `rs/garde`, or a cargo-family self-compliance pass now that `RS-CODE` no longer duplicates Clippy.
3. If wanted later, define a narrowly-scoped custom rule for low-quality test `expect(...)` messages only, and place it in the correct family after verifying Clippy cannot express it.
