# Reconcile Second-Wave TypeScript Family Plans

**Date:** 2026-03-30 11:57
**Scope:** `.plans/by_family/ts/code.md`, `.plans/by_family/ts/hexarch.md`, `.plans/by_family/ts/content.md`, `.plans/by_family/ts/css.md`, `.plans/by_family/ts/i18n.md`, `.plans/by_family/ts/jscpd.md`, `.plans/by_family/ts/libarch.md`, `.plans/by_family/ts/seo.md`

## Summary
Reconciled the second wave of TypeScript family plan files from placeholders into real family summaries with rule inventories, live code mapping, and explicit planning-vs-implementation boundaries. This wave covers the main source/architecture families (`code`, `hexarch`) plus several planning-heavy or mixed-surface families (`content`, `css`, `i18n`, `jscpd`, `libarch`, `seo`).

## Context & Problem
After the first-wave TS family reconciliation, the remaining TS family files were still placeholders even though several already had enough live code or clear planning intent to turn into usable family plans.

The problem with leaving them as placeholders was different by family:
- `code` and `hexarch` already had real live rule surfaces in code, but the by-family docs did not say what those rules actually were
- `content`, `libarch`, and `seo` are still planning-led, but they needed explicit rule inventories rather than vague future-shape notes
- `css`, `i18n`, and `jscpd` already had mixed or partially implemented live rule surfaces, but their family boundaries were still hidden by the grouped validator

This pass made those distinctions explicit.

## Decisions Made

### Reconcile `code` and `hexarch` before finishing the smaller families
- **Chose:** Expand `ts/code` and `ts/hexarch` first in the second wave.
- **Why:** They are the core TypeScript source- and architecture-policy families, and other family boundaries depend on getting those two right.
- **Alternatives considered:**
  - Leave them for later while filling only easy config families — rejected because they are the main TS ownership anchors.
  - Try to reconcile all remaining families in alphabetical order — rejected because that would bury the highest-leverage boundary work.

### Keep planning-heavy families honest about being planned
- **Chose:** Give `content`, `libarch`, and `seo` concrete rule inventories, but label them clearly as planned or planning-led where there is no dedicated runtime yet.
- **Why:** The by-family docs need to say what the family is supposed to own without pretending it is already implemented.
- **Alternatives considered:**
  - Keep those files as vague placeholders until code exists — rejected because the user explicitly asked for rule lists with purpose and behavior.

### Preserve mixed-runtime spillover explicitly instead of hiding it
- **Chose:** For `css`, `i18n`, and `jscpd`, explicitly document the mixed-current-code situation:
  - CSS package presence still lives in `package_deps.rs`
  - i18n overloads one id across multiple checks
  - jscpd still contains content-specific rules that do not really belong to the duplication family
- **Why:** The family plans should make ownership debt visible so the future split has a concrete target.
- **Alternatives considered:**
  - Pretend the current runtime already matches the family boundaries — rejected because that would recreate the shadow-spec problem.

## Architectural Notes
This wave made the TypeScript family split much clearer:

- `ts/code`
  - now explicitly owns the comment-suppression, `process.env`, `any`, file-length, and coverage/IDE-directive rule surface
- `ts/hexarch`
  - now explicitly owns service/extension structure, import boundaries, and ESLint boundary-config checks, with route-wrapper ownership still called out as ambiguous
- `ts/content`
  - now has a concrete planned rule set instead of only broad content-pipeline prose
- `ts/css`
  - now separates stylelint config/rule semantics from package-presence spillover
- `ts/i18n`
  - now documents the current overloaded `T-TOOL-12` reality instead of implying a cleaner split than exists
- `ts/jscpd`
  - now explicitly calls out the content-rule leakage in `jscpd_check.rs`
- `ts/libarch`
  - now has a concrete planned architecture inventory instead of only broad design notes
- `ts/seo`
  - now has a concrete planned SEO ownership surface rather than a pure placeholder

The common pattern is that each file now says which rules exist or are planned, what they are for, what current code owns them, and where family boundaries are still muddy.

## Information Sources
- `.plans/by_family/ts/*.md`
- `.plans/todo/checks/ts/code.md`
- `.plans/todo/checks/ts/hexarch.md`
- `.plans/todo/checks/ts/content.md`
- `.plans/todo/checks/ts/css.md`
- `.plans/todo/checks/ts/i18n.md`
- `.plans/todo/checks/ts/jscpd.md`
- `.plans/todo/checks/ts/libarch.md`
- `.plans/todo/checks/ts/seo.md`
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_code_analysis.rs`
- `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs`
- `apps/guardrail3/crates/app/ts/validate/stylelint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- `apps/guardrail3/crates/app/ts/validate/i18n_check.rs`
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
- `apps/guardrail3/crates/app/ts/validate/mod.rs`
- `.worklogs/2026-03-30-115529-reconcile-first-wave-ts-family-plans.md`

## Open Questions / Future Considerations
- `ts/code` still owns `T59` in code, but that may belong with `ts/package` or a future TS dependency-policy family instead of staying a source-scan rule.
- `ts/hexarch` still has route-wrapper enforcement ambiguity with `ts/eslint` (`T50`).
- `ts/content` still needs a real decision on how much app discovery it owns versus deferring to future `ts/arch`.
- `ts/i18n` should probably split `T-TOOL-12` into multiple family-native ids later.
- `ts/jscpd` should shed content-specific rules before its final family contract is considered clean.
- `ts/libarch` and `ts/seo` are still planning-led and will need future runtime design, not just doc work.

## Key Files for Context
- `.plans/by_family/ts/code.md` — current TS source-rule family summary and live source-scan rule inventory.
- `.plans/by_family/ts/hexarch.md` — current TS service/extension architecture family summary and boundary split with ESLint.
- `.plans/by_family/ts/content.md` — planning-led content family with explicit future rule slots.
- `.plans/by_family/ts/css.md` — CSS family summary with stylelint/package split called out.
- `.plans/by_family/ts/i18n.md` — i18n family summary documenting the overloaded single-rule runtime.
- `.plans/by_family/ts/jscpd.md` — duplication family summary with content spillover made explicit.
- `.plans/by_family/ts/libarch.md` — planned TS package-architecture family summary.
- `.plans/by_family/ts/seo.md` — planned TS SEO family summary.
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs` — current `ts/code` orchestrator and mixed source-scan rule surface.
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs` — current `ts/hexarch` source-side implementation root.
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs` — current config-side boundary audit still mixed into `ts/hexarch`.
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs` — current duplication file that still incorrectly carries content checks.

## Next Steps / Continuation Plan
1. Reconcile the remaining tool/config families:
   - `spelling`
   - `typecov`
   - `size`
2. Decide whether `tests` belongs with the already reconciled first-wave slice or needs another pass for package/tool-presence split.
3. After the remaining TS families are reconciled, start adding superseded banners to the old `.plans/todo/checks/ts/*.md` files one family at a time.
4. Once the TS family split is clearer, decide whether the repo needs a shared TypeScript architecture README analogous to Rust’s shared `rs/README.md`.
