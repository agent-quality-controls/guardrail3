# Add TypeScript Family Placeholders

**Date:** 2026-03-30 11:43
**Scope:** `.plans/by_family/ts/**`

## Summary
Created placeholder family files for the full canonical TypeScript family set under `.plans/by_family/ts` and upgraded the TS index README from a pure placeholder into a transitional family index. Each family file now points at the actual grouped TS validator code and the old detailed family ledger that still carries most of the rule-level detail.

## Context & Problem
After consolidating Rust family planning under `.plans/by_family/rs`, the user asked to begin the same process for TypeScript and, as a first step, create placeholder files for every TS family that exists in the current planning surface.

The TypeScript material was still split across:
- `.plans/todo/checks/ts/*.md`
- `.plans/by_file/ts/*.md`
- `.plans/by_file/tools/**`
- `.plans/per-app-config-design/**`
- grouped legacy implementation under `apps/guardrail3/crates/app/ts/validate/**`

Before doing deeper reconciliation, the missing piece was a stable family-indexed landing surface equivalent to the Rust one.

## Decisions Made

### Use the canonical family set from the old TS index
- **Chose:** Create placeholders for all 18 families listed in `.plans/todo/checks/ts/README.md`.
- **Why:** That file already defines the intended TS family inventory, so using it avoids inventing a new set during the transition.
- **Alternatives considered:**
  - Infer families only from current code files — rejected because several TS families are planned but not yet implemented as distinct runtime files.
  - Wait to add placeholders until every family is fully reconciled — rejected because the user explicitly wanted the new tree assembled family-by-family starting now.

### Keep placeholders thin but grounded in actual code
- **Chose:** For each family, record status, implementation roots, current detailed ledger, and the next reconciliation focus.
- **Why:** The placeholders should be enough to orient future cleanup without pretending the TS family cutover is already complete.
- **Alternatives considered:**
  - Copy the full old plan content into the new files — rejected because it would duplicate drift instead of reducing it.
  - Make the files empty stubs — rejected because they would not help reconcile old docs against the actual implementation.

### Treat the new TS tree as an index, not yet the full authority
- **Chose:** Update `.plans/by_family/ts/README.md` to make the transition-state authority order explicit.
- **Why:** Unlike Rust, TypeScript is not reconciled yet, so the old `.plans/todo/checks/ts/*.md` ledgers still need to remain the detailed source.
- **Alternatives considered:**
  - Immediately demote all old TS ledgers with superseded banners — rejected because the new TS files are only placeholders at this stage.

## Architectural Notes
The new TS tree now mirrors Rust structurally:
- one family index under `.plans/by_family/ts/README.md`
- one file per family under `.plans/by_family/ts/*.md`

But unlike Rust, TypeScript is still in a transitional authority state:
- live code under `apps/guardrail3/crates/app/ts/validate/**`
- `.plans/by_family/ts/**` for family indexing and reconciliation
- `.plans/todo/checks/ts/*.md` for detailed rule ledgers
- `.plans/by_file/**` and `.plans/per-app-config-design/**` as research/design support

This keeps the next TS reconciliation steps incremental instead of requiring a single large rewrite.

## Information Sources
- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/*.md`
- `.plans/by_file/ts/*.md`
- `.plans/per-app-config-design/02-typescript-config-scoping.md`
- `apps/guardrail3/crates/app/ts/validate/**`
- `.worklogs/2026-03-30-111654-create-by-family-rust-plan-surface.md`

## Open Questions / Future Considerations
- None of the old TS family ledgers have been demoted yet; that should wait until each family placeholder is reconciled into a stronger current summary.
- There is no family-local README layer for TS yet, unlike Rust.
- Several TS families are still purely planned (`arch`, `libarch`, `seo`) or only partially implemented (`content`, `fmt`, `spelling`, `typecov`, `size`).
- Hook/deploy TS-adjacent docs remain out of this family cutover for now.

## Key Files for Context
- `.plans/by_family/ts/README.md` — TS family index and transition-state authority order.
- `.plans/todo/checks/ts/README.md` — old canonical TS family set and detailed planning context.
- `.plans/by_family/ts/eslint.md` — example of a current family contract with substantial grouped implementation.
- `.plans/by_family/ts/arch.md` — example of a planned family with no cohesive runtime yet.
- `apps/guardrail3/crates/app/ts/validate/mod.rs` — current grouped TS validator entrypoint.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs` — example of mixed ownership that several future TS families need to split.
- `.worklogs/2026-03-30-111654-create-by-family-rust-plan-surface.md` — prior Rust cutover work that set the same indexing pattern.

## Next Steps / Continuation Plan
1. Pick one TS family at a time and reconcile its placeholder against live code plus the old detailed ledger, starting with the highest-leverage architectural families: `hexarch`, `arch`, `code`, `eslint`, `package`.
2. As each TS family is reconciled, strengthen its `.plans/by_family/ts/<family>.md` file and then add a superseded banner to the corresponding `.plans/todo/checks/ts/<family>.md`.
3. Once several core TS families are reconciled, add a TS shared architecture index comparable to the Rust shared README, so family placeholders can stop pointing directly into the old grouped validator.
