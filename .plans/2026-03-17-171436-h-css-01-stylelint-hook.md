# Add H-CSS-01 stylelint hook check and update pre-commit template

**Date:** 2026-03-17 17:14
**Task:** Add H-CSS-01 check for stylelint in pre-commit hook + update PRE_COMMIT_BASE template with CSS detection and stylelint step.

## Goal
Pre-commit hooks should lint CSS files with stylelint. Add a validation check (H-CSS-01) that detects whether the hook has stylelint configured, and update the generated pre-commit template to include a CSS/stylelint section.

## Approach

### Step-by-step plan
1. Add `check_stylelint_hook` function to `hook_script_checks.rs` — it fits with the other script content checks there
2. Wire it into `hook_checks.rs` in `check_hook_structure` — only when `has_typescript` is true (CSS is web-project-only)
3. Update `pre_commit.rs` PRE_COMMIT_BASE: add `CSS_CHANGED` detection line after `RUST_CHANGED`, add stylelint section between ESLint and Rust checks

## Files to Modify
- `apps/guardrail3/src/app/hooks/hook_script_checks.rs` — add `check_stylelint_hook` function
- `apps/guardrail3/src/app/hooks/hook_checks.rs` — wire H-CSS-01 into check_hook_structure, import the new function
- `apps/guardrail3/src/domain/modules/pre_commit.rs` — add CSS_CHANGED detection + stylelint section to PRE_COMMIT_BASE
