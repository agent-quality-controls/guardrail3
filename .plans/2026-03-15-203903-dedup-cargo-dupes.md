# Deduplicate all 11 cargo-dupes groups

**Date:** 2026-03-15 20:39
**Task:** Fix 11 exact duplicate groups reported by `cargo-dupes --exclude-tests`

## Goal
0 exact duplicate groups from `cargo-dupes --exclude-tests`. All 139+ tests pass. Clean clippy.

## Approach

### Group 1 + 10 (walkdir filter_entry closure — 6 members in rs/validate)
Create `pub fn is_excluded_dir(entry: &walkdir::DirEntry) -> bool` in `src/rs/validate/source_scan.rs`.
Replace all 6 closures in: test_quality_checks.rs (4), release_checks.rs (1), test_checks.rs (1).

### Group 2 (TS walkdir filter — 3 members)
Create `pub fn is_excluded_ts_dir(entry: &walkdir::DirEntry) -> bool` in `src/ts/validate/source_scan.rs`.
Replace closures in: ts/validate/test_checks.rs (2), ts/validate/source_scan.rs (1).

### Group 3 (check_release/publish/registry workflow — 3 similar fns)
Extract `check_workflow_contains(workflows, pattern, check_id, found_title, found_msg, missing_title, missing_msg, results)` in release_repo_checks.rs.

### Group 4 + 7 (read_workflow_files duplicated)
Move `read_workflow_files` to a shared location. Make the one in release_repo_checks.rs `pub` and import from release_bin_checks.rs.

### Group 5 (check_description / check_repository similar)
Extract `check_required_string_field(pkg, field_name, check_id, name, label, file, results)`.

### Group 6 (check_cargo_mutants_installed / check_semver_checks_installed)
Extract `check_tool_installed(tool_name, check_id, install_cmd, results)`.

### Group 8 (check_serde_method_bans_from_table / check_axum_type_bans_from_table)
These are thin wrappers calling check_ban_presence with different args. Inline them or keep as-is. They're 10 lines each — extract a macro or generic wrapper.

### Group 9 (check_claude_md / check_cliff_toml)
Extract `check_file_exists_at_root(root, filename, check_id, found_title, missing_title, results)`.

### Group 11 (collect_rs_files duplicated)
Make source_scan::collect_rs_files `pub` and import from garde_checks.

## Files to Modify
- `src/rs/validate/source_scan.rs` — add `pub fn is_excluded_dir`, make `collect_rs_files` pub
- `src/rs/validate/test_quality_checks.rs` — use shared filter
- `src/rs/validate/test_checks.rs` — use shared filter + extract tool check helper
- `src/rs/validate/release_checks.rs` — use shared filter
- `src/rs/validate/release_repo_checks.rs` — extract workflow helper, make read_workflow_files pub, extract file exists helper, extract tool check
- `src/rs/validate/release_bin_checks.rs` — import read_workflow_files
- `src/rs/validate/release_crate_checks.rs` — extract required field helper
- `src/rs/validate/code_quality_checks.rs` — use shared file exists helper
- `src/rs/validate/garde_checks.rs` — import collect_rs_files, inline ban wrappers
- `src/ts/validate/source_scan.rs` — add pub fn is_excluded_ts_dir
- `src/ts/validate/test_checks.rs` — use shared TS filter
