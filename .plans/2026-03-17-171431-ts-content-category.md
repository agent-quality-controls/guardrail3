# Add `content` boolean field to TypeScript check categories

**Date:** 2026-03-17 17:14
**Task:** Add a `content` category to `TsCheckCategories` and `TsChecksConfig`

## Goal
TypeScript check categories gain a `content` field that controls whether content-related checks run, resolved per-app-type from config.

## Approach

1. **config/types.rs** — Add `content: Option<bool>` to `TsChecksConfig`
2. **report.rs** — Add `content: bool` to `TsCheckCategories`, update Default (all true), update `TsAppType::default_categories` per-type
3. **main.rs** — Add `cfg_content` resolution in `build_ts_categories`, include `content` in both CLI and config branches
4. **commands/validate.rs** — Same changes to `build_ts_categories`

## Files to Modify
- `apps/guardrail3/src/domain/config/types.rs`
- `apps/guardrail3/src/domain/report.rs`
- `apps/guardrail3/src/main.rs`
- `apps/guardrail3/src/commands/validate.rs`
