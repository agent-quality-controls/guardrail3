# Extract inline tests from src/ to tests/ — batch 1 (7 files)

**Date:** 2026-03-16 18:45
**Task:** Move `#[cfg(test)] mod tests` blocks from 7 source files to `apps/guardrail3/tests/unit/` as integration-style tests.

## Goal
Remove all `#[cfg(test)]` blocks from the 7 listed source files. Tests become standalone files in `tests/unit/` that import via `use guardrail3::...`. Private items needed by tests get promoted to `pub(crate)`.

## Approach

### Visibility changes needed
- `ast_visitors` module: change from `mod` to `pub(crate) mod` in validate/mod.rs
- `deny_inventory` module: change from `mod` to `pub(crate) mod` in validate/mod.rs
- `ast_helpers` pub(super) items used by ast_visitors tests: `parse_file` is already `pub`, OK
- `ast_visitors::IgnoreVisitor`: already `pub` struct with `pub` fields — accessible if module is pub(crate)
- `discover` module functions: `detect_project` and `ProjectInfo` are already `pub`

### Files to create in tests/unit/
1. `allow_checks_test.rs` — tests from allow_checks.rs
2. `ast_helpers_test.rs` — tests from ast_helpers.rs
3. `ast_visitors_test.rs` — tests from ast_visitors.rs
4. `code_quality_checks_test.rs` — tests from code_quality_checks.rs
5. `deny_inventory_test.rs` — tests from deny_inventory.rs
6. `dependency_allowlist_test.rs` — tests from dependency_allowlist.rs
7. `discover_test.rs` — tests from discover.rs

### Import path mapping
- `super::*` in allow_checks → `guardrail3::app::rs::validate::allow_checks::*`
- `super::*` in ast_helpers → `guardrail3::app::rs::validate::ast_helpers::*`
- `super::super::ast_helpers::parse_file` in ast_visitors → `guardrail3::app::rs::validate::ast_helpers::parse_file`
- `super::*` in code_quality_checks → `guardrail3::app::rs::validate::code_quality_checks::*`
- `super::*` in deny_inventory → `guardrail3::app::rs::validate::deny_inventory::*`
- `super::*` in dependency_allowlist → `guardrail3::app::rs::validate::dependency_allowlist::*`
- `super::*` in discover → `guardrail3::app::discover::*`

### Steps
1. Create `tests/unit/` directory and `tests/unit/mod.rs` (or individual files declared in a test harness)
2. For each file: extract test block, rewrite imports, write to tests/unit/
3. For each source file: remove the `#[cfg(test)] mod tests { ... }` block
4. Make visibility changes in mod.rs files
5. Run `cargo test` to verify

## Risks
- `pub(crate)` visibility changes needed for private modules
- Tests using `crate::adapters::outbound::fs::RealFileSystem` (discover tests) — need to check if that's accessible
- `FileSystem` trait impl `StubFs` in dependency_allowlist tests — need to recreate in test file
