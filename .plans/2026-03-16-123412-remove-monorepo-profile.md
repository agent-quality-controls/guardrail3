# Remove monorepo profile

**Date:** 2026-03-16 12:34
**Task:** Remove the "monorepo" profile since init is now split into `rs init` and `ts init`

## Goal
Eliminate the monorepo profile entirely. Users who need monorepo support run both init commands separately.

## Approach

### Code changes
1. `src/commands/generate.rs:388` — remove `|| profile == "monorepo"`
2. `src/commands/init.rs:83,111` — remove `|| profile == "monorepo"` checks
3. `src/commands/init.rs:225` — remove the `"monorepo"` match arm from `generate_rs_config_content`
4. `src/modules/clippy.rs:192` — update doc comment to remove monorepo mention
5. `src/modules/pre_commit.rs:287` — update doc comment
6. `src/modules/release.rs:21` — update comment about monorepos
7. `src/rs/validate/clippy_coverage.rs:100` — update comment
8. `src/discover.rs:27,214,216` — these are about monorepo *structure* detection, not the profile. Keep as-is.
9. `CLAUDE.md` — remove monorepo row from profiles table, update config example, add note about running both init commands

### Files NOT changed
- `src/discover.rs` — references monorepo as a directory structure pattern, not as a profile
- `tests/` — will update after cargo test to see what breaks
