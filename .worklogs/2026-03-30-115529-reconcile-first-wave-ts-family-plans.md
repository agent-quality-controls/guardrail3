# Reconcile First-Wave TypeScript Family Plans

**Date:** 2026-03-30 11:55
**Scope:** `.plans/by_family/ts/arch.md`, `.plans/by_family/ts/eslint.md`, `.plans/by_family/ts/tsconfig.md`, `.plans/by_family/ts/npmrc.md`, `.plans/by_family/ts/package.md`, `.plans/by_family/ts/fmt.md`, `.plans/by_family/ts/tests.md`

## Summary
Upgraded the first wave of TypeScript family files from placeholder summaries into actual family plans with rule inventories, live code mapping, and reconciliation notes. The new content is code-led: it cross-checks the old TS ledgers against the current grouped validator under `apps/guardrail3/crates/app/ts/validate/**` instead of trusting older docs at face value.

## Context & Problem
The user asked to go family by family on the new TypeScript planning surface and fill out each family plan so it contains the rules, what each rule should do, and what each rule is for. The earlier placeholder pass created one file per TS family, but those files were intentionally thin and still depended heavily on the old `.plans/todo/checks/ts/*.md` ledgers.

The immediate risk was that the old ledgers were not precise enough:
- some families like `ts/eslint`, `ts/tsconfig`, `ts/package`, and `ts/tests` already have richer live rule surfaces in code than the old ledgers describe
- some families like `ts/fmt` and `ts/arch` still have mostly planned contracts rather than fully distinct implementation
- some grouped code mixes ownership across families (`package_deps.rs`, `tool_config_checks.rs`, `mod.rs`)

So the first reconciliation pass had to be grounded in live code rather than doc inheritance.

## Decisions Made

### Reconcile the first TS wave from live code first
- **Chose:** Fill out `arch`, `eslint`, `tsconfig`, `npmrc`, `package`, `fmt`, and `tests` first.
- **Why:** This wave covers the root/ownership layer plus the highest-leverage config/tool families and one clean source-quality family specimen.
- **Alternatives considered:**
  - Start with every TS family at once — rejected because the grouped validator still mixes ownership heavily and would increase drift risk.
  - Only rewrite the old ledgers — rejected because the new `by_family` tree is supposed to become the current surface.

### Use actual validator IDs where they exist, and explicit planned rule slots where they do not
- **Chose:** Preserve live rule IDs like `T1`, `T-ESLP-12`, `T9`, `T-NPMRC-01`, `T55`, `T-TEST-04`, and use planned named slots only where the family has no cohesive implementation yet, such as `TS-ARCH-*`.
- **Why:** This keeps the family plans faithful to current code without inventing fake stability for families that are still mostly design lanes.
- **Alternatives considered:**
  - Invent a fresh family-prefixed TS rule namespace immediately — rejected because it would sever the plan from current code before the migration exists.
  - Keep everything abstract and avoid listing rule IDs — rejected because the user explicitly wanted a per-rule list with purpose and behavior.

### Treat the grouped TS validator as the current implementation source of truth
- **Chose:** Cross-check the plans directly against `apps/guardrail3/crates/app/ts/validate/**`.
- **Why:** The old TS ledgers are still useful, but several are incomplete or too coarse relative to the real runtime.
- **Alternatives considered:**
  - Use only `.plans/todo/checks/ts/*.md` — rejected because that would miss many live rules in `ts/eslint`, `ts/tsconfig`, and `ts/tests`.

## Architectural Notes
The first-wave family split now looks like this:

- `ts/arch`
  - still a planned repo-global family; no cohesive runtime yet
- `ts/eslint`
  - already has a large live rule surface, but it is split across config, plugin, parser, infra, and package-dependency buckets
- `ts/tsconfig`
  - already behaves like a family in code, but still hangs off the grouped validator
- `ts/npmrc`
  - already family-shaped in code and one of the cleanest TS specimens
- `ts/package`
  - real manifest-policy family, but still mixed with sibling tool/package ownership questions
- `ts/fmt`
  - mostly a contract today, with only package presence implemented distinctly
- `ts/tests`
  - already cohesive in code, but the planned contract is broader than the live rule surface

The main structural theme is that TypeScript still has the old grouped-validator problem Rust used to have. These files now make that explicit family by family instead of hiding it behind generic placeholders.

## Information Sources
- `.plans/by_family/ts/*.md`
- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/arch.md`
- `.plans/todo/checks/ts/eslint.md`
- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/npmrc.md`
- `.plans/todo/checks/ts/package.md`
- `.plans/todo/checks/ts/fmt.md`
- `.plans/todo/checks/ts/tests.md`
- `.plans/by_file/ts/eslint-config-mjs.md`
- `.plans/by_file/ts/npmrc.md`
- `.plans/by_file/ts/tsconfig.md`
- `.plans/per-app-config-design/02-typescript-config-scoping.md`
- `apps/guardrail3/crates/app/ts/validate/mod.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`
- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/test_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md`

## Open Questions / Future Considerations
- The five spawned family-audit agents were not reliable in this session because the existing thread pool was polluted by older work; this pass therefore relied on live-code reconciliation rather than agent output.
- `ts/code` and `ts/hexarch` are still the most important next families to reconcile because they will clarify the main source-scan versus architecture split.
- `ts/arch` still has no live routed runtime, so its inventory is necessarily planned rather than implemented.
- `ts/fmt` is still mostly a contract and should not be treated as fully enforced today.

## Key Files for Context
- `.plans/by_family/ts/README.md` — current TS family index and transition-state authority order.
- `.plans/by_family/ts/eslint.md` — example of a fully expanded current TS family plan with live rule IDs.
- `.plans/by_family/ts/tsconfig.md` — example of a current TS family whose live rule inventory is richer than the old ledger suggested.
- `.plans/by_family/ts/arch.md` — example of a planned TS family with explicit non-implemented rule slots.
- `apps/guardrail3/crates/app/ts/validate/mod.rs` — grouped TS validator orchestrator and current ownership mixing.
- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs` — largest current TS family rule bucket.
- `apps/guardrail3/crates/app/ts/validate/package_check.rs` — current package-policy family core.
- `apps/guardrail3/crates/app/ts/validate/test_checks.rs` — current cohesive TS tests family implementation.
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md` — placeholder cutover that this pass builds on.

## Next Steps / Continuation Plan
1. Reconcile `ts/code` from `source_scan.rs`, `ts_comment_checks.rs`, `ts_code_analysis.rs`, and `ast_helpers.rs`, including a full rule inventory.
2. Reconcile `ts/hexarch` from `ts_arch_checks.rs` and `eslint_audit.rs`, explicitly separating architecture-owned rules from baseline ESLint-owned rules.
3. Reconcile `ts/tests` further if needed, but keep its current code/plan mismatch explicit until package/tool-presence questions are split out cleanly.
4. Continue the same pattern for `content`, `i18n`, `css`, `jscpd`, `typecov`, `spelling`, `size`, and `seo`.
5. Only after several core TS families are reconciled should the old `.plans/todo/checks/ts/*.md` files start getting superseded banners like Rust.
