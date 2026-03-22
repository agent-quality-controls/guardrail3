# Wire Crawler Data into TS + Hooks Validation — Phase 2

**Date:** 2026-03-19 20:39
**Task:** Replace hardcoded path construction with crawler data in TS validate and hooks validate modules.

## Goal
TS validate and hooks validate modules currently construct config file paths via `path.join("filename")`. Change them to iterate over paths discovered by the crawler, enabling proper multi-config support for monorepos.

## Approach

### 1. `app/ts/validate/mod.rs` — orchestrator
- Rename `_crawl` to `crawl`
- Pass `&crawl.eslint_configs` to config_files::check
- Pass crawl data through to eslint plugin checks (use first eslint config found)
- No changes to source_scan, arch, or test sections (they don't need crawler data yet)

### 2. `app/ts/validate/config_files.rs` — dispatcher
- Accept `&CrawlResult` parameter
- Pass crawler data to each sub-check

### 3. `app/ts/validate/eslint_check.rs`
- Change `check_eslint_config` to accept `eslint_configs: &[PathBuf]` instead of root path
- Iterate each config, run all checks on each
- If empty, emit T1 error

### 4. `app/ts/validate/tsconfig_check.rs`
- Change `check_tsconfig` to accept `tsconfigs: &[PathBuf]` and `tsconfig_bases: &[PathBuf]`
- Check tsconfig.base.json first (from tsconfig_bases), fall back to tsconfigs
- Iterate all found configs

### 5. `app/ts/validate/npmrc_check.rs`
- Change `check_npmrc` to accept `npmrcs: &[PathBuf]`
- Iterate each, validate all

### 6. `app/ts/validate/package_check.rs`
- Change `check_package_json` to accept `package_jsons: &[PathBuf]`
- Root package.json = first one (sorted, so root comes first)
- Run all existing checks on root

### 7. `app/ts/validate/package_deps.rs`
- Change `check_lint_plugins` and `check_additional_tools` to accept `package_jsons: &[PathBuf]`
- Use root (first) package.json

### 8. `app/ts/validate/jscpd_check.rs`
- Change `check_jscpd` to accept `jscpd_configs: &[PathBuf]`
- If empty, emit T19 warning
- Iterate each config

### 9. `app/hooks/validate.rs` + `hook_checks.rs`
- Change to use `crawl.pre_commit_hooks`
- Pass pre-commit hook paths from crawler instead of hardcoding `.githooks/pre-commit`

## Key decisions
- **Root-first strategy**: For package.json, npmrc, etc., the sorted crawl vectors put root files first. Use the first entry as the "root" config.
- **Iterate all for eslint/tsconfig**: These benefit most from multi-config support.
- **Keep check IDs unchanged**: The semantic meaning doesn't change, just the discovery method.

## Files to modify
- `app/ts/validate/mod.rs` — rename _crawl, pass to config_files
- `app/ts/validate/config_files.rs` — accept CrawlResult, pass fields to checks
- `app/ts/validate/eslint_check.rs` — accept &[PathBuf]
- `app/ts/validate/tsconfig_check.rs` — accept &[PathBuf]
- `app/ts/validate/npmrc_check.rs` — accept &[PathBuf]
- `app/ts/validate/package_check.rs` — accept &[PathBuf]
- `app/ts/validate/package_deps.rs` — accept &[PathBuf]
- `app/ts/validate/jscpd_check.rs` — accept &[PathBuf]
- `app/hooks/validate.rs` — rename _crawl, pass to hook_checks
- `app/hooks/hook_checks.rs` — accept pre-commit paths from crawl
