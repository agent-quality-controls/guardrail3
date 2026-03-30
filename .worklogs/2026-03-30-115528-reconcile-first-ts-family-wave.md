# Reconcile First TS Family Wave

**Date:** 2026-03-30 11:55
**Scope:** `.plans/by_family/ts/arch.md`, `.plans/by_family/ts/npmrc.md`, `.plans/by_family/ts/tsconfig.md`, `.plans/by_family/ts/package.md`, `.plans/by_family/ts/fmt.md`, `.plans/by_family/ts/tests.md`

## Summary
Reconciled the first TypeScript family wave from placeholders into usable by-family ledgers. The updated files now carry explicit rule inventories, current code mapping, implementation status, and doc/code reconciliation notes instead of only pointing back to the old TS plan tree.

## Context & Problem
The previous step created placeholder files for every TypeScript family under `.plans/by_family/ts`, but those files were still too thin to serve as current family plans. The user asked to go family by family and make each family plan list all of its rules with a description of what each rule should do and what it is for.

The immediate challenge was that TS family detail still lives in three places at once:
- old family ledgers under `.plans/todo/checks/ts/*.md`
- grouped validator code under `apps/guardrail3/crates/app/ts/validate/**`
- the new placeholder by-family docs

So the first real reconciliation step was to pick the lowest-ambiguity families and collapse those three sources into one current family summary.

## Decisions Made

### Start with the low-ambiguity TS families
- **Chose:** Reconcile `arch`, `npmrc`, `tsconfig`, `package`, `fmt`, and `tests` first.
- **Why:** These families already had either a narrow current implementation (`npmrc`, `tsconfig`, `tests`, `package`) or a clear planning gap (`arch`, `fmt`) that could be documented honestly without pretending the code was further along than it is.
- **Alternatives considered:**
  - Start with `eslint` or `hexarch` first — rejected for this first wave because those families are broader and still have more mixed ownership with sibling TS families.
  - Keep the placeholder files and only gather agent notes — rejected because the user explicitly wanted the family plan itself to carry the rule inventory.

### Treat current grouped rule IDs as real inventory even when they are poorly scoped
- **Chose:** Document legacy rule ids like `T9`, `T11`, `T15`, `T-TOOL-04`, and `T-PKG-01` as the current live inventory where code already emits them.
- **Why:** The goal was to make the by-family files truthful about current behavior, not to silently invent cleaned-up ids before the TS validator is actually refactored.
- **Alternatives considered:**
  - Rewrite all current TS rules into new family-scoped ids immediately — rejected because that would create a shadow contract with no matching implementation.
  - Ignore the live ids and summarize only the conceptual policy — rejected because the user asked for all rules in a list.

### Be explicit where the family contract is broader than the current runtime
- **Chose:** For families like `fmt` and `arch`, document planned rules separately from the smaller current implementation.
- **Why:** Some TS families still exist more clearly in planning than in code. The plan files need to say that plainly instead of pretending they are fully implemented.
- **Alternatives considered:**
  - Shrink the family contract to only what current code already does — rejected because it would erase important intended family boundaries.
  - Keep broad contract language without implementation status — rejected because it would hide the current gap and recreate planning drift.

## Architectural Notes
This wave established the by-family format that should be repeated for the rest of the TS families:

- status
- implementation roots
- current source of truth
- current state
- rule inventory
- current implementation mapping
- implementation/reconciliation notes
- historical references
- next planning focus

That makes the new by-family files more than just indexes; they are now current family summaries that bridge planning and code honestly.

Key family outcomes:
- `ts/npmrc` is already close to a clean family-shaped runtime.
- `ts/tsconfig` has a real rule inventory in code, but `T9` is overloaded.
- `ts/package` is substantial, but root vs local manifest policy is still uneven.
- `ts/fmt` remains mostly planned; only package presence is currently implemented.
- `ts/tests` is already cohesive in code, but the old contract is broader than the runtime.
- `ts/arch` remains a planned missing family, and the doc now says that plainly.

## Information Sources
- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/arch.md`
- `.plans/todo/checks/ts/npmrc.md`
- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/package.md`
- `.plans/todo/checks/ts/fmt.md`
- `.plans/todo/checks/ts/tests.md`
- `apps/guardrail3/crates/app/ts/validate/mod.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_arch_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/test_checks.rs`
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md`

## Open Questions / Future Considerations
- `eslint` and `hexarch` still need careful reconciliation because their current code and intended ownership are more tangled.
- `ts/fmt` likely needs actual implementation work before its by-family file can become authoritative instead of transitional.
- `ts/package` needs a decision on whether local app/package manifest existence should be a concrete family rule or remain part of future root ownership.
- `ts/tests` needs a contract decision on which planned rules are mandatory versus just future quality ideas.

## Key Files for Context
- `.plans/by_family/ts/README.md` — TS family index and current transition-state authority order.
- `.plans/by_family/ts/npmrc.md` — first clean specimen where the family plan and live code are already close.
- `.plans/by_family/ts/tsconfig.md` — example of a family with a dense live rule inventory that needed expansion from the old high-level plan.
- `.plans/by_family/ts/package.md` — example of a family with real ownership drift between current code and intended boundaries.
- `.plans/by_family/ts/fmt.md` — example of a family whose intended contract is much larger than the current runtime.
- `.plans/by_family/ts/tests.md` — example of a family that is cohesive in code but narrower than the old plan.
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs` — current live `tsconfig` rule inventory.
- `apps/guardrail3/crates/app/ts/validate/package_check.rs` — current live `package` rule inventory.
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md` — prior TS cutover step that established the placeholder tree.

## Next Steps / Continuation Plan
1. Reconcile the next TS family wave with the same format:
   - `eslint`
   - `code`
   - `hexarch`
   - `i18n`
   - `jscpd`
   - `css`
2. After that, do the tool/config families:
   - `spelling`
   - `typecov`
   - `size`
3. Finish with the remaining more-planned families:
   - `content`
   - `libarch`
   - `seo`
4. As each family becomes strong enough, add a superseded banner to the corresponding `.plans/todo/checks/ts/<family>.md` file the same way Rust was migrated.
