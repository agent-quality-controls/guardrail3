# Fix rs init: only create guardrail3.toml + smart workspace discovery

**Date:** 2026-03-17 21:10
**Task:** Two fixes to rs init command

## Goal
1. `rs init` should ONLY create guardrail3.toml — no local/*.toml, release-plz.toml, cliff.toml (those belong in `generate`)
2. `rs init` should discover workspace members like `ts init` does, showing per-crate config

## Approach

### Step 1: Remove scaffold_local_dir, scaffold_release_files, print_rs_summary
These functions are no longer needed. Init only creates guardrail3.toml.

### Step 2: Simplify run_rs
- Remove calls to scaffold_local_dir and scaffold_release_files
- Remove skipped tracking
- Simplify to: scaffold_config + simple summary + "run generate" message
- Clean up dry_run path to only show guardrail3.toml

### Step 3: Update generate_rs_config_content for smart discovery
- Add project_path parameter
- Use detect_project to discover workspace members
- Generate per-crate [rust.crates.X] sections with profile/layer
- Generate workspace-level [rust.checks]
- Remove [local] section (that's generate's job)

### Step 4: Update call sites
- scaffold_config and dry_run path both call generate_rs_config_content

## Files to Modify
- `apps/guardrail3/src/commands/init.rs` — all changes in this one file
