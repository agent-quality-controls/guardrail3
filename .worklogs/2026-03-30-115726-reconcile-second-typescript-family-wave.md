# Reconcile Second TypeScript Family Wave

**Date:** 2026-03-30 11:57
**Scope:** `.plans/by_family/ts/code.md`, `.plans/by_family/ts/content.md`, `.plans/by_family/ts/css.md`, `.plans/by_family/ts/hexarch.md`, `.plans/by_family/ts/i18n.md`, `.plans/by_family/ts/jscpd.md`, `.plans/by_family/ts/libarch.md`, `.plans/by_family/ts/seo.md`

## Summary
Reconciled the second TypeScript family wave in the new by-family planning tree by turning placeholder files into actual family summaries with explicit rule inventories, current code mapping, and implementation-vs-plan notes. This wave covered the source/architecture/content families that were still the most likely to drift because their current code is mixed across grouped TS validator files.

## Context & Problem
After the first TypeScript family wave established the new planning format for config/tooling families, the remaining high-risk docs were the ones where old planning intent and current grouped TS validator code diverged most:

- `ts/code`
- `ts/hexarch`
- `ts/content`
- `ts/css`
- `ts/i18n`
- `ts/jscpd`
- `ts/libarch`
- `ts/seo`

These families had placeholder by-family files, but they still lacked the concrete rule lists the user asked for: every rule, what it should do, and what it is for. Several of them also had obvious mixed ownership in code, especially:
- `jscpd_check.rs` carrying content rules
- `eslint_audit.rs` carrying TS hexarch config-side architecture rules
- `package_deps.rs` carrying CSS package presence
- `i18n_check.rs` overloading one rule ID for multiple checks

## Decisions Made

### Reconcile from live code first, old plan second
- **Chose:** Derive rule inventories from current `apps/guardrail3/crates/app/ts/validate/**` code wherever code already exists, and only use the old TS ledgers to fill future/planned rules.
- **Why:** The old TS plan files often describe broader or cleaner family boundaries than the current grouped validator actually implements. The by-family tree needs to be honest about both the intended family contract and the current runtime.
- **Alternatives considered:**
  - Copy old TS ledger bullets into the new files as-is — rejected because it would hide real implementation gaps and mixed ownership.
  - Treat the current grouped validator as the whole final contract — rejected because some families are explicitly still planning-led (`libarch`, `seo`, much of `content`).

### Keep mixed ownership explicit instead of pretending the family split already exists
- **Chose:** Add “mixed code mapping” / “reconciliation notes” sections where current code is split across the wrong runtime files.
- **Why:** The goal of the by-family tree is to become the current source of truth. It must say clearly where current implementation is still in the wrong place.
- **Alternatives considered:**
  - Hide mixed ownership and describe only the target family — rejected because that would make the planning docs lie about the actual current codebase.

### Use family-native placeholder rule IDs only for truly planning-led families
- **Chose:** For families without a real runtime yet (`content`, `libarch`, `seo`), define provisional family-native rule identifiers in the by-family file.
- **Why:** The user asked for full rule lists per family, but those families do not have concrete runtime IDs yet. The docs still need a coherent family-shaped rule inventory.
- **Alternatives considered:**
  - Leave those families without rule IDs until implementation exists — rejected because the by-family tree would stay too vague to guide real work.
  - Reuse unrelated mixed runtime IDs from other families — rejected because that would normalize the very ownership drift we are trying to clean up.

## Architectural Notes
This wave sharpened the TypeScript family boundaries:

- `ts/code` now explicitly owns source suppressions, `process.env`, `any`, file size, comment directives, and currently also installed-tree banned package scanning.
- `ts/hexarch` now explicitly owns service/extension structure plus boundary-config enforcement split across source and ESLint config.
- `ts/content` is documented as mostly planning-led, with the only live checks still trapped inside `jscpd_check.rs`.
- `ts/css` now distinguishes config/rule semantics from package-presence spillover.
- `ts/i18n` is documented honestly as one overloaded live rule (`T-TOOL-12`) that should later split into family-native rules.
- `ts/jscpd` now clearly separates true duplication-policy rules from the misplaced content checks (`T60`, `T61`).
- `ts/libarch` and `ts/seo` are now explicit planning-led families with concrete target rule inventories rather than vague placeholders.

The most important structural outcome is that the by-family docs now make mixed ownership visible instead of silently inheriting the grouped TS validator layout.

## Information Sources
- `.plans/by_family/ts/*.md`
- `.plans/todo/checks/ts/*.md`
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs`
- `apps/guardrail3/crates/app/ts/validate/stylelint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/i18n_check.rs`
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs`
- `apps/guardrail3/crates/app/ts/validate/mod.rs`
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md`
- `.worklogs/2026-03-30-115405-reconcile-first-typescript-family-wave.md`
- `.worklogs/2026-03-30-115528-reconcile-first-ts-family-wave.md`

## Open Questions / Future Considerations
- `ts/code` still needs a decision on whether installed-tree banned package scanning (`T59`) remains code-owned or moves to `ts/package`.
- `ts/hexarch` still has a live boundary ambiguity around route-wrapper enforcement currently represented as `T50` on the ESLint side.
- `ts/content`, `ts/libarch`, and `ts/seo` still need real runtime families; their new rule inventories are planning-led rather than implementation-led.
- `ts/i18n` should split the overloaded `T-TOOL-12` into multiple family-native rule IDs once the family gets its own runtime surface.

## Key Files for Context
- `.plans/by_family/ts/README.md` — current TS family index and authority order during the TS cutover.
- `.plans/by_family/ts/code.md` — example of a family reconciled directly against current code.
- `.plans/by_family/ts/hexarch.md` — example of a family with mixed source/config architecture ownership.
- `.plans/by_family/ts/content.md` — example of a mostly planning-led family with misplaced live checks.
- `.plans/by_family/ts/jscpd.md` — example where current code and family ownership are clearly misaligned.
- `apps/guardrail3/crates/app/ts/validate/source_scan.rs` — current TS source-rule runtime.
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs` — current TS hexarch source-side runtime.
- `apps/guardrail3/crates/app/ts/validate/eslint_audit.rs` — current TS hexarch config-side runtime.
- `apps/guardrail3/crates/app/ts/validate/jscpd_check.rs` — mixed duplication/content runtime that still needs to split.
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md` — first TS cutover index creation.
- `.worklogs/2026-03-30-115405-reconcile-first-typescript-family-wave.md` — first TS family wave context.

## Next Steps / Continuation Plan
1. Reconcile the remaining high-value TS family still lagging behind the live runtime: `ts/eslint`, including all concrete rule IDs and the current split against `ts/hexarch`.
2. Add superseded banners to the older `.plans/todo/checks/ts/*.md` files only after each corresponding by-family file is complete enough to stand on its own.
3. Once the core TS families are reconciled, create a TS shared architecture index comparable to `apps/guardrail3/crates/app/rs/README.md`, so by-family docs can point to a common TS architecture contract instead of only the grouped validator layout.
