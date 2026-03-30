# Reconcile First TypeScript Family Wave

**Date:** 2026-03-30 11:54
**Scope:** `.plans/by_family/ts/{eslint,fmt,npmrc,package,size,spelling,tsconfig,typecov}.md`

## Summary
Turned the first wave of TypeScript family placeholders into actual reconciled family plans by adding explicit rule inventories, current implementation mappings, and implementation-status notes. This focused on the easy tool/config families first, plus `ts/eslint`, because those had enough code and plan material to reconcile without waiting for the full TS family migration.

## Context & Problem
The earlier TS cutover only created placeholders under `.plans/by_family/ts/**`. The user then asked to go family by family, aggregate prior plan material, and make each family plan contain all of the rules with descriptions of what they do and what they are for.

TypeScript is in a different state from Rust:
- no shared TS README yet
- grouped implementation still lives under `apps/guardrail3/crates/app/ts/validate/**`
- old detailed ledgers still live under `.plans/todo/checks/ts/*.md`

That meant the new by-family files needed to become more than placeholders, but not pretend the old ledgers were already obsolete.

## Decisions Made

### Reconcile the easy TS families first
- **Chose:** Start with `ts/npmrc`, `ts/tsconfig`, `ts/package`, `ts/fmt`, `ts/spelling`, `ts/typecov`, `ts/size`, and `ts/eslint`.
- **Why:** These families already have narrow enough current code surfaces or code-discoverable rule IDs to reconcile cleanly.
- **Alternatives considered:**
  - Start with `ts/arch` or `ts/hexarch` first — rejected for this slice because those families still have more cross-family boundary ambiguity.
  - Wait for every sub-agent result before editing anything — rejected because several of the easy families were already deterministically recoverable from local code plus the old ledgers.

### Make the new by-family files code-led where docs understate the live rule surface
- **Chose:** Extract live check IDs and current behavior directly from the TS validator code and use that to populate the family plans.
- **Why:** Some old TS ledgers are directionally correct on scope but incomplete on actual rule inventory, especially `ts/eslint`.
- **Alternatives considered:**
  - Copy the old ledgers into the new files with minimal edits — rejected because that would preserve the existing drift.

### Keep planned-only rules explicit rather than pretending they are implemented
- **Chose:** For partially implemented tool families (`fmt`, `spelling`, `typecov`, `size`), distinguish implemented package/config/script checks from planned contract pieces.
- **Why:** The old ledgers promise more than the current runtime enforces. The reconciled family docs need to make that visible instead of overstating current coverage.
- **Alternatives considered:**
  - Trim the family contract down to only what the current code emits — rejected because the user asked to preserve the intended family rule surface, not just today’s runtime output.

## Architectural Notes
This pass establishes a transition pattern for TypeScript families:

- the by-family file is the current planning/status summary
- the old TS ledger remains the detailed rule/history document until the family is fully reconciled
- the live validator code is authoritative when the old ledger and code disagree on concrete emitted rule IDs

The most important structural observation from this slice:
- `ts/npmrc` and `ts/tsconfig` are already close to real family-shaped runtimes
- `ts/package` has a solid root policy core, but still shares some tool/package ownership with sibling families
- `ts/fmt`, `ts/spelling`, `ts/typecov`, and `ts/size` are still mostly mixed-tool implementations
- `ts/eslint` has a much larger live rule surface than the old family ledger shows

## Information Sources
- `.plans/by_family/ts/*.md`
- `.plans/todo/checks/ts/{eslint,fmt,npmrc,package,size,spelling,tsconfig,typecov}.md`
- `.plans/by_file/ts/{eslint-config-mjs,npmrc,tsconfig}.md`
- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_plugin_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_parser.rs`
- `apps/guardrail3/crates/app/ts/validate/eslint_rule_infra.rs`
- `apps/guardrail3/crates/app/ts/validate/npmrc_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_check.rs`
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
- `apps/guardrail3/crates/app/ts/validate/tool_config_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs`
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md`

## Open Questions / Future Considerations
- `ts/eslint` and `ts/hexarch` still need a precise ownership split for boundary-related checks like `T6` and route-wrapper enforcement.
- `ts/tsconfig` currently overloads `T9` across existence, parseability, and several core strictness checks.
- `ts/package` currently applies most rules only at the package-manager root and gives per-app manifests a narrower rule subset.
- Tool families still need exact config-root ownership decisions before their old ledgers can be demoted.

## Key Files for Context
- `.plans/by_family/ts/README.md` — TS family index and current transition-state authority order.
- `.plans/by_family/ts/eslint.md` — code-led example of a reconciled TS family with a large live rule surface.
- `.plans/by_family/ts/tsconfig.md` — reconciled config family with overloaded rule-id notes.
- `.plans/by_family/ts/npmrc.md` — narrow TS family already close to its final split.
- `.plans/by_family/ts/package.md` — root-policy family with mixed sibling ownership still visible.
- `apps/guardrail3/crates/app/ts/validate/eslint_check.rs` — live ESLint rule inventory and current boundary overlap.
- `apps/guardrail3/crates/app/ts/validate/package_deps.rs` — shared TS tool/package ownership hotspot.
- `apps/guardrail3/crates/app/ts/validate/tsconfig_check.rs` — current TS config rule inventory and overloaded IDs.
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md` — prior TS cutover checkpoint that created the placeholder family tree.

## Next Steps / Continuation Plan
1. Reconcile the next high-leverage architecture/source families: `ts/arch`, `ts/hexarch`, `ts/code`, and `ts/tests`.
2. Reconcile the remaining mixed tool/config families: `ts/css`, `ts/jscpd`, `ts/content`, `ts/i18n`, `ts/seo`, and `ts/libarch`.
3. After each family file is strong enough, add superseded banners to the matching `.plans/todo/checks/ts/*.md` files the same way Rust now works.
4. Once several core TS families are reconciled, add a shared TS architecture index so by-family files can point to a stable shared contract instead of only the grouped validator.
