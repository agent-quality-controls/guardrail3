# Replace grep-based hook checks with guardrail3 validate

**Date:** 2026-03-17 14:23
**Task:** Remove redundant grep-based tamper detection and structural health from pre-commit hook, replace with guardrail3 validate --staged

## Goal
The pre-commit hook should use guardrail3 itself (AST-based) instead of broken grep patterns. This eliminates false positives and removes code duplication.

## Approach

### What gets removed from the hook
1. **"Guardrail tamper detection" section** (lines 73-131) — eslint-disable, #[allow(, config relaxation grep checks
2. **"Structural health checks" section** (lines 184-231) — file length, use count, crate-wide allow grep checks

### What replaces them
A single `guardrail3 validate --staged` call, placed early (after secret scanning, before language-specific tool checks):

```bash
# --- Guardrail validation (AST-based) ---
echo "Running guardrail3 validation..."
if command -v guardrail3 &> /dev/null; then
    if [ "$RUST_CHANGED" -gt 0 ] || [ "$CARGO_CHANGED" -gt 0 ]; then
        if ! guardrail3 rs validate --staged .; then
            echo "guardrail3 Rust validation failed."
            exit 1
        fi
    fi
    if [ "$TS_CHANGED" -gt 0 ]; then
        if ! guardrail3 ts validate --staged .; then
            echo "guardrail3 TypeScript validation failed."
            exit 1
        fi
    fi
else
    echo "WARNING: guardrail3 not installed. Skipping guardrail validation."
    echo "Install: cargo install guardrail3"
fi
```

Note: needs to be after the stack detection section (RUST_CHANGED/TS_CHANGED vars).

### Files to modify
- `apps/guardrail3/src/domain/modules/pre_commit.rs` — the template
- `.githooks/pre-commit` — the local hook (regenerate via `guardrail3 generate` or edit directly)
