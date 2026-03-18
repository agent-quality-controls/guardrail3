# Wire T-TOOL check modules into TS validate orchestrator

**Date:** 2026-03-17 18:32
**Task:** Add T-TOOL-01..12 and H-TOOL-01..05 checks to mod.rs orchestrator, update help_gen.rs docs, update CLAUDE.md check count.

## Goal
Wire existing `tool_config_checks` and `i18n_check` modules into the TS validation pipeline, and update documentation to reflect 17 new checks.

## Approach

### Step-by-step plan
1. **mod.rs** — Add `mod tool_config_checks;` declaration (i18n_check already declared). Add three new sections in `run()`: additional tool packages, tool configurations, and i18n completeness (content-gated).
2. **help_gen.rs** — Add ADDITIONAL TOOLS and CONTENT TOOLS sections after ESLINT PLUGIN CHECKS.
3. **CLAUDE.md** — Update total check count from 162 to 179 (162 + 17).

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/mod.rs` — wire checks into orchestrator
- `apps/guardrail3/src/help_gen.rs` — add check category docs
- `CLAUDE.md` — update total check count
