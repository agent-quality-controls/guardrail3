# Decompose functions with #[allow(clippy::too_many_lines)]

**Date:** 2026-03-16 19:37
**Task:** Split 20 instances of too_many_lines across 18 files into smaller sub-functions

## Goal
Remove all `#[allow(clippy::too_many_lines)]` by decomposing long functions into smaller helpers.

## Approach

For each function, extract logical sections into sub-functions. Most are sequential validation checks.

### Analysis of each instance:

1. **main.rs:main()** - Already dispatches to handle_rs/handle_ts/handle_guide. Only ~20 lines. Just remove the allow.
2. **init.rs:run_rs()** - 108 lines. Split into: scaffold_config, scaffold_local_files, scaffold_release_files, print_summary.
3. **discover.rs:detect_rust()** - ~100 lines. Split into: parse_exclude_dirs, expand_workspace_members.
4. **npmrc_check.rs:check_npmrc()** - ~120 lines. Split into: parse_settings, check_expected_settings, check_extra_settings.
5. **eslint_audit.rs:check()** - ~117 lines. Split into check_zones, check_import_direction, check_entry_point, check_external_deps.
6. **eslint_check.rs:check_eslint_config()** - ~274 lines. Split into check_core_rules, check_line_scans, check_rule_presence, check_route_and_env.
7. **dependency_scan.rs:check_cargo_lock()** - ~90 lines. Split into parse_and_scan_lockfile, report_banned_crates.
8. **toolchain_check.rs:check_toolchain_settings()** - ~122 lines. Split into check_channel, check_components.
9. **deny_bans.rs:check_ban_list()** - ~138 lines. Split into check_multiple_versions, check_highlight, check_deny_list_coverage.
10. **deny_licenses.rs:check_licenses()** - ~114 lines. Split into check_allow_list, check_private_ignore, check_confidence_threshold.
11. **deny_licenses.rs:check_sources()** - ~87 lines. Split into check_unknown_registries, check_allow_git.
12. **rustfmt_check.rs:check_rustfmt_settings()** - ~169 lines. Split into check_string_settings, check_int_settings, check_bool_settings, check_extra_settings.
13. **hook_checks.rs:check_hooks()** - ~211 lines. Split into check_hook_existence, check_hook_structure, check_hook_metadata, check_hook_extras.
14. **hook_checks.rs:check_monolithic_patterns()** - ~113 lines. Data-driven pattern table, loop is the body. Try removing allow to see if clippy complains.
15. **workspace_metadata.rs:check_workspace_metadata()** - ~90 lines. Split into check_edition_version, check_publish, check_release_profile.
16. **mod.rs(rs/validate):run()** - ~174 lines. Orchestrator with clear section blocks. Split into run_code_checks, run_architecture_checks, run_test_checks, run_release_checks.
17. **deny_audit.rs:check_advisory_values()** - ~110 lines. Split into check_unmaintained, check_yanked.
18. **config_files.rs:check_clippy_thresholds()** - ~90 lines. Already data-driven loop. Try removing allow.
19. **release_repo_checks.rs:check_release_plz_toml()** - ~110 lines. Split into validate_plz_structure, check_package_coverage.
20. **cargo_lints.rs:check_lint_level()** - ~99 lines. Match with several arms. Try removing allow.

## Files to Modify
All 18 files listed above.
