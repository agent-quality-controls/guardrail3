# Fix hex arch layer check and add set -e hook validation

**Date:** 2026-03-19 18:26
**Task:** Fix two gaps: T-ARCH-01 only checks 2/4 hex layers, and no set -e validation in hooks

## Goal
1. T-ARCH-01 in `check_single_app_structure` should also check for `src/modules/application` layer (the steady-parent template structure uses `application/commands/`, not bare `ports/`)
2. Add H-SAFE-01 check that pre-commit hook contains `set -e` or `set -euo pipefail`

## Approach

### Step 1: Fix check_single_app_structure in ts_arch_checks.rs
- Add `has_application` probe for `modules_dir.join("application")`
- Add "application" to the missing layers list when absent
- Update the message to mention all three required layers

### Step 2: Add check_set_e_safety to hook_script_checks.rs
- New function `check_set_e_safety(content, results)` following H-TOOL pattern
- Check ID: H-SAFE-01, Severity: Warn
- Pattern: content contains "set -e" or "set -euo pipefail"
- Export from hook_script_checks, import+call in hook_checks.rs

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/ts_arch_checks.rs` — add application layer check
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs` — add check_set_e_safety function
- `apps/guardrail3/src/app/hooks/hook_checks.rs` — import and wire check_set_e_safety
