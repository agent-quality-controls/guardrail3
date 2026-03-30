# Reconcile Second TypeScript Family Wave

**Date:** 2026-03-30 11:57
**Scope:** `.plans/by_family/ts/code.md`, `.plans/by_family/ts/content.md`, `.plans/by_family/ts/css.md`, `.plans/by_family/ts/hexarch.md`, `.plans/by_family/ts/i18n.md`, `.plans/by_family/ts/jscpd.md`, `.plans/by_family/ts/libarch.md`, `.plans/by_family/ts/seo.md`

## Summary
Replaced the second-wave TypeScript family placeholders with actual rule inventories, current code mapping, and reconciliation notes for `code`, `content`, `css`, `hexarch`, `i18n`, `jscpd`, `libarch`, and `seo`. The by-family TS tree now covers the full canonical family set with explicit rule lists instead of bare placeholders.

## Context & Problem
After creating the `.plans/by_family/ts` tree and reconciling the cleaner first-wave families, eight TS family files were still only placeholders:
- `code`
- `content`
- `css`
- `hexarch`
- `i18n`
- `jscpd`
- `libarch`
- `seo`

The user asked for a family-by-family planning surface where each family file contains the actual rules with a description of what they should do and what they are for. That meant the placeholders had to become real family summaries even when the underlying TS validator is still grouped and unevenly split.

## Decisions Made

### Reconcile the second wave by family maturity, not by implementation cleanliness
- **Chose:** Treat the remaining families in three buckets:
  - code-led families with current rule IDs (`code`, `css`, `jscpd`, `i18n`, `hexarch`)
  - planning-led families with no cohesive runtime yet (`content`, `libarch`, `seo`)
  - mixed families where live code exists but ownership is still split (`hexarch`, `jscpd`, `css`)
- **Why:** The TS validator is still grouped under `apps/guardrail3/crates/app/ts/validate/**`, so one uniform method would either understate the planned families or overstate the mixed ones.
- **Alternatives considered:**
  - Leave planning-only families as placeholders until code exists — rejected because the user explicitly asked for a rule list per family now.
  - Pretend every family has a clean runtime surface already — rejected because that would create a misleading shadow spec.

### Make mixed-family spillover explicit instead of hiding it
- **Chose:** Call out current spillover in the by-family docs, especially:
  - `jscpd` still carrying content rules `T60` and `T61`
  - `css` package presence still living in `package_deps.rs`
  - `hexarch` relying on `eslint_audit.rs` for `T36`..`T39` and still sharing route-wrapper concerns with `ts/eslint`
- **Why:** The planning surface has to help split the grouped validator later; hiding mixed ownership would make that harder, not easier.
- **Alternatives considered:**
  - Keep only ideal target rules in the new docs — rejected because it would erase the exact boundary mess the family split needs to solve.

### Use current rule IDs where they exist, and explicit intended rule names where they do not
- **Chose:** Keep the real `T*` / `T-STYL-*` / `T-ARCH-*` / `T-TOOL-*` IDs for live code-backed rules, and use `TS-...-*` style names for planning-led families without a cohesive runtime yet.
- **Why:** This keeps the docs faithful to the current code while still giving planning-only families a concrete rule inventory.
- **Alternatives considered:**
  - Rename everything into future family-native IDs immediately — rejected because the code does not reflect that split yet.
  - Mirror the old ledgers verbatim — rejected because many of them still describe broad ownership without a usable per-rule list.

## Architectural Notes
With this change, every canonical TS family file in `.plans/by_family/ts` now has a rule inventory section.

The second-wave families surface three important TS architecture realities:
- `ts/code` and `ts/tests` are already reasonably family-shaped, even if still grouped under the old validator tree.
- `ts/hexarch`, `ts/css`, and `ts/jscpd` have real rules but still leak ownership across adjacent families.
- `ts/content`, `ts/libarch`, and `ts/seo` are still planning-led, but they now have explicit intended rule inventories that can guide future implementation instead of vague family-level bullets.

This makes the by-family tree usable as the current TS planning surface even before the code split is done.

## Information Sources
- `.plans/by_family/ts/*.md`
- `.plans/todo/checks/ts/code.md`
- `.plans/todo/checks/ts/content.md`
- `.plans/todo/checks/ts/css.md`
- `.plans/todo/checks/ts/hexarch.md`
- `.plans/todo/checks/ts/i18n.md`
- `.plans/todo/checks/ts/jscpd.md`
- `.plans/todo/checks/ts/libarch.md`
- `.plans/todo/checks/ts/seo.md`
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
- `apps/guardrail3/crates/app/ts/validate/stylelint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/i18n_check.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs`
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md`

## Open Questions / Future Considerations
- `ts/code` still owns `T59` against installed `node_modules`, which may belong with dependency/package policy instead of source scanning long term.
- `ts/jscpd` still carries content-specific rules that should move to `ts/content`.
- `ts/css` still has package-presence ownership split across `package_deps.rs`.
- `ts/hexarch` still has a real ownership ambiguity with `ts/eslint` around route-wrapper and lint-config boundary rules.
- `ts/i18n` still overloads everything under `T-TOOL-12`.
- `ts/content`, `ts/libarch`, and `ts/seo` still need real runtime implementations later; these docs are now concrete planning contracts, not code summaries.

## Key Files for Context
- `.plans/by_family/ts/code.md` — source-rule family reconciled from actual `T23`..`T35` and `T59` code.
- `.plans/by_family/ts/jscpd.md` — example of a mixed family with explicit spillover into `ts/content`.
- `.plans/by_family/ts/css.md` — example of a config/rule family with package-presence spillover.
- `.plans/by_family/ts/hexarch.md` — example of a current TS architecture family whose rule surface is split across source analysis and ESLint config checks.
- `.plans/by_family/ts/content.md` — planning-led family now expressed as a concrete intended rule inventory.
- `.plans/by_family/ts/libarch.md` — planning-led package architecture family now expressed concretely.
- `.plans/by_family/ts/seo.md` — planning-led public-route/metadata family now expressed concretely.
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs` — live source-scan rule implementation used to reconcile `ts/code`.
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs` — mixed implementation showing why `ts/jscpd` and `ts/content` need a cleaner split.
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs` — live lint-config architecture enforcement that still feeds `ts/hexarch`.

## Next Steps / Continuation Plan
1. Reconcile the remaining first-wave TS files that are already cleaner in code but still need the same format tightened if needed:
   - `arch`
   - `eslint`
   - `fmt`
   - `npmrc`
   - `package`
   - `tsconfig`
2. After that, add superseded banners to `.plans/todo/checks/ts/*.md` one family at a time, starting with the families that now have strong by-family summaries.
3. Decide whether TypeScript needs a shared architecture README, or whether the by-family tree plus the grouped validator code is enough until the implementation split starts.
