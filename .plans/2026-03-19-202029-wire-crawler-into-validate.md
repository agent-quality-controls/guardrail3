# Wire crawler into validate

**Date:** 2026-03-19 20:20
**Task:** Replace hardcoded path discovery in validate with crawler data

## Problem
Validate and crawler are completely disconnected. Three independent discovery systems exist:
1. `crawl.rs` — single efficient walk, classifies all files (used by coverage/map only)
2. `discover.rs` — separate filesystem walk for ProjectInfo
3. `ts_arch_checks::discover_ts_apps()` — yet another walk for TS apps

Every check module hardcodes paths: `workspace_root.join("clippy.toml")`, `path.join("eslint.config.mjs")`, etc. This means validate only checks ROOT-LEVEL configs and misses per-crate/per-app configs in monorepos.

## Goal
Call `crawl()` once at the top of validate, pass `&CrawlResult` down, check EVERY discovered config file against baselines. Uncovered directories become visible.

## Approach — 3 phases

### Phase 1: Thread CrawlResult through (signature changes)
1. `commands/validate.rs::run()` — add `let crawl = crawl::crawl(&abs_path);` at the top
2. Pass `&crawl` to `rs::validate::run()`, `ts::validate::run()`, `hooks::validate::run()`
3. Each orchestrator accepts `&CrawlResult` as a new parameter
4. No check logic changes yet — just plumbing

### Phase 2: Replace hardcoded path construction
Each check module changes from `workspace_root.join("file")` to extracting from crawl data.

**Rust checks:**
- `config_files.rs`: use `crawl.clippy_tomls`, `crawl.rustfmt_tomls`, `crawl.rust_toolchains` filtered by workspace root
- `clippy_coverage.rs`: iterate ALL `crawl.clippy_tomls`, check each one against baselines
- `deny_audit.rs` + `deny_bans.rs`: iterate ALL `crawl.deny_tomls`, check each
- `cargo_lints.rs`: use `crawl.cargo_tomls` for workspace + member Cargo.tomls

**TS checks:**
- `eslint_check.rs`: iterate `crawl.eslint_configs` (may find per-app configs)
- `tsconfig_check.rs`: iterate `crawl.tsconfigs` + `crawl.tsconfig_bases`
- `npmrc_check.rs`: iterate `crawl.npmrcs`
- `package_check.rs`: iterate `crawl.package_jsons`
- `jscpd_check.rs`: use `crawl.jscpd_configs` + `crawl.velite_configs`

**Hooks:**
- `hook_checks.rs`: use `crawl.pre_commit_hooks`

### Phase 3: Add uncovered-directory reporting
Using `crawl.dirs_with_rs` and `crawl.dirs_with_ts`, report source directories that have NO covering config file. Same walk-up resolution the coverage engine uses, but now as validation errors.

## Key design decisions

### Per-file vs per-workspace validation
Currently checks run once per workspace root. After wiring, checks should run PER CONFIG FILE found by the crawler. A monorepo with 6 clippy.toml files gets 6 independent validations.

### Filter by workspace
The crawler finds ALL files in the project tree. RS checks should filter to files within the Rust workspace root. TS checks should filter to files relevant to the TS project. The `ProjectInfo.workspaces` list provides the boundaries.

### Backward compatibility
The orchestrator signatures change (new `&CrawlResult` parameter). All callers (main.rs, tests) need updating. Integration tests that call the binary are unaffected.

## Files to modify

| Phase | File | Change |
|---|---|---|
| 1 | `commands/validate.rs` | Add `crawl()` call, pass down |
| 1 | `app/rs/validate/mod.rs` | Accept `&CrawlResult` |
| 1 | `app/ts/validate/mod.rs` | Accept `&CrawlResult` |
| 1 | `app/hooks/validate.rs` | Accept `&CrawlResult` |
| 1 | `main.rs` | Update call sites |
| 2 | `app/rs/validate/config_files.rs` | Use crawl data |
| 2 | `app/rs/validate/clippy_coverage.rs` | Use crawl data |
| 2 | `app/rs/validate/deny_audit.rs` | Use crawl data |
| 2 | `app/rs/validate/deny_bans.rs` | Use crawl data |
| 2 | `app/rs/validate/cargo_lints.rs` | Use crawl data |
| 2 | `app/ts/validate/eslint_check.rs` | Use crawl data |
| 2 | `app/ts/validate/tsconfig_check.rs` | Use crawl data |
| 2 | `app/ts/validate/npmrc_check.rs` | Use crawl data |
| 2 | `app/ts/validate/package_check.rs` | Use crawl data |
| 2 | `app/ts/validate/jscpd_check.rs` | Use crawl data |
| 2 | `app/hooks/hook_checks.rs` | Use crawl data |
| 3 | `app/rs/validate/mod.rs` | Add uncovered-dir reporting |
| 3 | `app/ts/validate/mod.rs` | Add uncovered-dir reporting |

## Risks
- Signature changes touch many files — parallel agents may conflict
- Integration tests may need crawler mocking or test fixture updates
- Performance: crawl adds ~10ms but eliminates 3 redundant walks
- Some checks use `fs.read_file()` which still needs the FileSystem trait — crawler only finds paths, doesn't cache content
