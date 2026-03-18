# Fix RS generate to use actual filesystem paths, not config keys

**Date:** 2026-03-18 00:43
**Task:** RS generate creates files at wrong paths. Config key `validator-rust` ≠ directory path `apps/validator-rust/`.

## Problem

Init discovers workspace members at `apps/validator-rust/crates/...`, strips to `validator-rust` for the config key. Generate uses the config key as a directory path — creating `validator-rust/clippy.toml` instead of `apps/validator-rust/clippy.toml`.

steady-parent structure:
- Root `Cargo.toml` — virtual workspace with `members = ["packages/*"]`, excludes `apps/*`
- `apps/validator-rust/Cargo.toml` — nested workspace with `members = ["crates/*"]`
- `apps/substack-publisher/Cargo.toml` — another nested workspace
- Existing clippy.toml at `apps/validator-rust/clippy.toml`, deny.toml at `apps/validator-rust/deny.toml`

## Approach

### Step 1: Generate runs discovery to resolve app names → actual paths

In `generate_rust_files()`, run `detect_project()` to get workspace info. Build a mapping:
- For each app key in `[rust.apps.*]`, find the matching workspace whose root contains the app name
- The app's actual path = that workspace root relative to project root

For steady-parent:
- `validator-rust` → `apps/validator-rust`
- `substack-publisher` → `apps/substack-publisher`

### Step 2: Per-workspace file generation

Each Rust workspace needs its own set of workspace-level files:
- `clippy.toml` — at workspace root
- `deny.toml` — at workspace root (cargo-deny runs per workspace)
- `rustfmt.toml` — at workspace root or project root (inherited)
- `rust-toolchain.toml` — project root only (one toolchain for all)

For steady-parent this means:
- `apps/validator-rust/clippy.toml` (per-app)
- `apps/validator-rust/deny.toml` (per-workspace)
- `apps/substack-publisher/clippy.toml` (per-app)
- `apps/substack-publisher/deny.toml` (per-workspace)
- `rustfmt.toml` at root (shared)
- `rust-toolchain.toml` at root (shared)
- `release-plz.toml`, `cliff.toml` at root (shared)

### Step 3: Per-crate clippy.toml stays the same but with correct path prefix

Currently: `{root_prefix}{crate_path}/clippy.toml` where crate_path is the config key.
Fixed: `{resolved_app_path}/clippy.toml` where resolved_app_path is from discovery.

If there are per-crate overrides within an app (e.g., composition-root vs pure), those sub-crate clippy.toml files need the full path too.

## Files to Modify

- `apps/guardrail3/src/commands/generate.rs` — `generate_rust_files()` to run discovery, resolve paths
- `apps/guardrail3/src/commands/diff.rs` — same resolution for dry-run

## Key Decision

- Generate discovers actual paths at runtime rather than storing paths in config. This means the config stays clean (`[rust.apps.validator-rust]`) and paths are always correct even if the project structure changes.
