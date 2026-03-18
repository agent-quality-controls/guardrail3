# Add auto-detection of TypeScript app type

**Date:** 2026-03-17 19:20
**Task:** Add auto_detect_app_type function to infer Service/Content/Library from directory structure and package.json deps

## Goal
When no explicit type is configured in guardrail3.toml for a TS app, auto-detect it from signals in the app directory before falling back to Service default.

## Approach

### Changes to mod.rs
1. Add `auto_detect_app_type(fs, app_path) -> Option<TsAppType>` function with signals:
   - `src/modules/domain` dir exists → Service
   - `content` dir exists → Content
   - package.json deps: backend frameworks → Service, content tools → Content, next without hex arch → Content
2. Update `resolve_app_contexts` to use config > auto-detect > default chain
3. Update `has_content_app` to also scan discovered apps for content signals, changing its signature to take `fs` and `root`

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/mod.rs` — all three changes
