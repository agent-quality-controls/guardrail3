# Extract inline tests to tests/unit/ — batch 2 (8 files)

**Date:** 2026-03-16 18:44
**Task:** Move all `#[cfg(test)]` blocks from 8 src/ files to `apps/guardrail3/tests/unit/`

## Goal
Remove all inline test modules from 8 validate source files, placing them into integration-style test files under `tests/unit/`. Private functions used by tests get `pub(crate)`.

## Approach

### Files with `_tests.rs` split (4 files)
These already have separate test files referenced via `#[path = "..._tests.rs"]`:
1. `garde_checks.rs` — remove `#[cfg(test)] #[path] mod tests;`, delete `garde_checks_tests.rs`, create `tests/unit/test_garde_checks.rs`
2. `hex_arch_checks.rs` — same pattern
3. `release_crate_deps.rs` — same pattern
4. `release_repo_checks.rs` — same pattern

### Files with inline tests (4 files)
These have `#[cfg(test)] mod tests { ... }` inline:
5. `release_bin_checks.rs` — extract inline tests
6. `release_checks.rs` — extract inline tests
7. `release_crate_checks.rs` — extract inline tests
8. `source_scan.rs` — extract inline tests

### For each file
1. Cut `#[cfg(test)]` block from src file
2. Create test file with `use guardrail3::...` imports replacing `use super::*`
3. Make private functions `pub(crate)` if tests need them
4. Register test file as `mod` in a tests/unit/mod.rs or as standalone

### Visibility changes needed
- `garde_checks.rs`: `check_garde_dependency`, `content_has_garde_dependency`, `check_reqwest_json_ban_from_table`, `check_ban_presence`, `extract_ban_paths`, `is_input_boundary_derive`, `count_unvalidated_input_structs`, constants `EXPECTED_SERDE_METHOD_BANS`, `EXPECTED_AXUM_TYPE_BANS`, `INPUT_BOUNDARY_DERIVES`
- `hex_arch_checks.rs`: `Layer`, `layer_from_config`, `layer_from_path`, `contains_segment`, `resolve_layer`, `normalize_path`, `is_service_internal`, `check_hex_arch_structure`, `check_dependency_flow`, `check_library_service_boundary`, `check_unconfigured_members`, `CrateLayerMap`, types
- `release_bin_checks.rs`: `check_binary_release_workflow`, `check_binary_linux_target`, `check_binstall_metadata`
- `release_checks.rs`: `discover_crates`, `CrateInfo`
- `release_crate_checks.rs`: `check_required_string_field`, `check_license`, `check_readme`, `check_readme_quality`, `check_version`
- `release_crate_deps.rs`: already mostly pub, plus helpers
- `release_repo_checks.rs`: `check_license_file`, `check_release_plz_toml`, `check_cliff_toml`, `check_workflow_contains`, `check_semver_checks_installed`
- `source_scan.rs`: `filter_non_comment_lines`, `strip_string_literals`, `strip_inline_block_comments`

## Files to Modify
- 8 src files: remove `#[cfg(test)]` blocks, make private items `pub(crate)`
- 4 `_tests.rs` files: delete after moving content
- Create `apps/guardrail3/tests/unit/` directory + mod.rs + 8 test files
