# Fix SCOPE-01, SCOPE-02, DISC-01

**Date:** 2026-03-19 18:26
**Task:** Fix three bugs in validate.rs and discover.rs

## Goal
Fix staged file detection missing renames, dirty file detection missing untracked files, and TS detection triggering on pure JS projects.

## Approach

### SCOPE-01: --staged misses renamed files
- File: `apps/guardrail3/src/commands/validate.rs`, line 140
- Change `--diff-filter=ACM` to `--diff-filter=ACMR`

### SCOPE-02: --dirty misses untracked files
- File: `apps/guardrail3/src/commands/validate.rs`, function `git_dirty_files`
- Add `git ls-files --others --exclude-standard` command and merge results

### DISC-01: Any package.json triggers TS detection
- File: `apps/guardrail3/src/app/discover.rs`, function `detect_typescript`
- After finding package.json, also require tsconfig.json OR typescript in deps/devDeps
- Read and parse package.json to check for typescript dependency

## Files to Modify
- `apps/guardrail3/src/commands/validate.rs` — SCOPE-01 and SCOPE-02
- `apps/guardrail3/src/app/discover.rs` — DISC-01
