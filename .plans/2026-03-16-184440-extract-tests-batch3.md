# Extract inline tests to tests/unit/ — batch 3

**Date:** 2026-03-16 18:44
**Task:** Move #[cfg(test)] blocks from 8 src/ files to tests/unit/

## Goal
Remove all inline test modules from 8 source files, create corresponding test files in tests/unit/, make private functions pub(crate) where needed.

## Files
1. rs/validate/structure_checks.rs — tests call pub fns only, straightforward
2. rs/validate/test_checks.rs — tests call private fns (check_cargo_mutants_installed, check_mutants_toml, check_mutants_profile, has_mutants_profile, check_tests_exist, content_has_test, file_has_cfg_test_module)
3. rs/validate/test_quality_checks.rs — tests call private fns (count_pub_fns, count_test_fns, check_test_coverage_inventory, check_integration_tests, check_ignore_without_reason, find_ignore_without_reason, check_mutation_hook, has_rs_files_in_dir)
4. ts/validate/ast_helpers.rs — tests call pub fns only
5. ts/validate/source_scan.rs — tests call private fns (check_process_env, check_any_types, check_file_length, check_comment_pattern)
6. ts/validate/test_checks.rs — tests call private fns (check_stryker_config, check_test_files_exist, check_test_runner_config) + pub fns
7. ts/validate/ts_arch_checks.rs — has separate _tests.rs file with #[path] attr, tests call private fns (check_single_app_structure, check_file_imports, layer_from_path, extract_import_path, resolve_relative, TsLayer)
8. ts/validate/ts_code_analysis.rs — tests call pub fns only

## Approach
- Make needed private fns pub(crate)
- Create tests/unit/ directory
- Create one test file per source file
- Remove #[cfg(test)] blocks from source files
- For ts_arch_checks: also remove the _tests.rs file and #[path] attr
