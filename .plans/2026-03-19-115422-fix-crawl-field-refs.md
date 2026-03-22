# Fix CrawlResult field references after Option→Vec migration

**Date:** 2026-03-19 11:54
**Task:** Update all references to renamed CrawlResult fields (Option<PathBuf> → Vec<PathBuf>)

## Goal
Fix compilation errors in project_map.rs and map.rs caused by CrawlResult fields being changed from Option<PathBuf> to Vec<PathBuf> with pluralized names.

## Approach

### project_map.rs
1. Update `RootConfigs` struct: change 9 fields from `Option<PathBuf>` to `Vec<PathBuf>`
2. Update `RustScopeConfigs` struct: change `rust_toolchain` to use Vec pattern
3. Fix `find_rust_configs_at()`: change rust_toolchain from `.as_ref()` to `.iter().find()`
4. Fix `read_pnpm_patterns()`: change from `&crawl.pnpm_workspace` to `.first()`
5. Fix `build_root_configs()`: change all `.clone()` on singular fields to use Vec patterns

### map.rs
1. Fix all `rc.field.is_some()` → `!rc.field.is_empty()` or use `.first()`
2. Fix all `if let Some(p) = &rc.field` → `if let Some(p) = rc.field.first()`
3. Fix `configs.rust_toolchain.is_some()` references

## Files to Modify
- `apps/guardrail3/src/app/project_map.rs`
- `apps/guardrail3/src/commands/map.rs`
