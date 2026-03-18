# Per-app type context for TS architecture checks

**Date:** 2026-03-17 15:49
**Task:** Use per-app type context to filter which TS apps get architecture checks

## Goal
Architecture checks (hex arch structure, import boundaries) should only run on service-type apps, not content or library apps. Resolve app type from guardrail3.toml config and use it to filter.

## Approach

1. `ts_arch_checks.rs`: Make `discover_ts_apps` pub(super), add `check_hex_arch_structure_for_apps` and `check_import_boundaries_for_apps`, make `collect_module_ts_files` and `check_file_imports` pub(super) if needed
2. `mod.rs`: Add `config` param to `run()`, add `resolve_app_contexts()`, use per-app filtering for arch checks
3. `main.rs`: Pass config to `ts::validate::run()`
4. `commands/validate.rs`: Pass config to `ts::validate::run()`

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/ts_arch_checks.rs`
- `apps/guardrail3/src/app/ts/validate/mod.rs`
- `apps/guardrail3/src/main.rs`
- `apps/guardrail3/src/commands/validate.rs`
