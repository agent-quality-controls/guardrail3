# Refactor ProjectInfo to support multiple workspaces

**Date:** 2026-03-17 10:31
**Task:** Change ProjectInfo from flat workspace fields to Vec<RustWorkspace>, update all consumers, fix R-ARCH-04 message, R26 section hints, R-PUB-08 version.workspace support.

## Goal
ProjectInfo uses `workspaces: Vec<RustWorkspace>` instead of flat `cargo_workspace_root`, `workspace_members`, `workspace_member_dirs`. Helper methods provide backward-compat access. All 292 tests pass.

## Approach

### Step 1: Change ProjectInfo struct in discover.rs
- Remove `cargo_workspace_root`, `workspace_members`, `workspace_member_dirs`
- Add `workspaces: Vec<RustWorkspace>` with `RustWorkspace { root: PathBuf, members: Vec<WorkspaceMember> }` and `WorkspaceMember { name: String, dir: String }`
- Add helper methods: `all_member_dirs()`, `all_member_names()`, `primary_workspace_root()`
- Update `detect_rust` to populate RustWorkspace
- Update `discover_nested_workspaces` to add separate RustWorkspace entries
- Update `detect_project` fallback logic

### Step 2: Update all consumers
- `rs/validate/mod.rs` — use `primary_workspace_root()`, `all_member_dirs()`
- `hex_arch_checks.rs` — iterate workspaces for R-ARCH-01/02/03/04
- `cargo_lints.rs` — `all_member_dirs()` for inheritance check
- `config_files.rs` — `all_member_dirs()` for per-crate clippy
- `release_checks.rs` — no change needed (uses walkdir, _project unused)
- `commands/validate.rs` — no direct field access
- Test files: update ProjectInfo construction

### Step 3: Fix R-ARCH-04 message
- Include workspace root in unconfigured crate message
- New detailed message format

### Step 4: Fix R26 section hints
- In `cargo_lints.rs`, when R26 reports missing lint, indicate `[workspace.lints.rust]` vs `[workspace.lints.clippy]`

### Step 5: Fix R-PUB-08 version.workspace = true
- In `release_crate_checks.rs`, check for `version.workspace = true` table form
- Resolve version from workspace root Cargo.toml

## Files to Modify
- `apps/guardrail3/src/app/discover.rs` — struct + detection logic
- `apps/guardrail3/src/app/rs/validate/mod.rs` — workspace_root + member_dirs access
- `apps/guardrail3/src/app/rs/validate/hex_arch_checks.rs` — all 4 check functions
- `apps/guardrail3/src/app/rs/validate/cargo_lints.rs` — R26 section hints
- `apps/guardrail3/src/app/rs/validate/config_files.rs` — member_dirs parameter
- `apps/guardrail3/src/app/rs/validate/release_crate_checks.rs` — R-PUB-08 version.workspace
- `apps/guardrail3/tests/unit/test_hex_arch_checks.rs` — update ProjectInfo construction
- `apps/guardrail3/tests/unit/test_release_checks.rs` — update ProjectInfo construction
- `apps/guardrail3/tests/unit/discover_test.rs` — update assertions
