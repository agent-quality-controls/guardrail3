# .githooks/pre-commit

## Location

One per repo. Configured via `git config core.hooksPath .githooks`. No per-app hooks (git limitation — one hooks directory per repo).

**In steady-parent:** `.githooks/pre-commit` (283 lines). Monorepo-aware with per-app Rust loops, per-app TS type checking, project-specific checks.

## Contents (verified)

**Guardrail steps (~40%, ~110 lines):**
- Secret scanning (gitleaks) — hard fail if not installed
- File size check (1MB)
- cargo fmt --check (per Rust app)
- cargo clippy -D warnings (per Rust app)
- cargo deny check (per Rust app)
- cargo machete (per Rust app)
- cargo test --workspace (per Rust app)
- Structural health (500 lines, 20 use statements, no crate-wide allow)
- ESLint with --max-warnings 0
- Guardrail tamper detection (#[allow] without reason, eslint-disable without reason)

**Project-specific steps (~60%, ~173 lines):**
- Migration consistency check (project-specific migrations)
- Content-constraints staleness check
- Per-app TS type checking loop (project knows which apps to typecheck)
- Stylelint on CSS files
- jscpd --threshold 10 (matches project's .jscpd.json)
- Specific tamper detection patterns (config relaxation patterns)
- Monorepo-aware iteration loops (`for app in apps/*/`)
- File staging and selective checking logic

## Category: Scaffold-once

**Critical limitation of scaffold-once:** Guardrail steps can NEVER be updated in existing hooks. If guardrail3 adds a new check, the user's hook doesn't get it.

**Alternative (better but more complex): Source-based model**
guardrail3 generates checks to `.guardrail3/generated/pre-commit-checks.sh`. User's `.githooks/pre-commit` sources it:
```bash
source .guardrail3/generated/pre-commit-checks.sh
# project-specific checks below
```
This lets guardrail3 update its checks without touching the user's hook.

## Algorithm

### Scaffold-once model (current plan):
```
1. If .githooks/pre-commit exists: do NOT touch
2. If missing: generate starter hook with guardrail steps
3. Set core.hooksPath if not already set
```

### Source-based model (future):
```
1. ALWAYS regenerate .guardrail3/generated/pre-commit-checks.sh
2. If .githooks/pre-commit doesn't exist: scaffold starter that sources the generated checks
3. If .githooks/pre-commit exists without source line: validate warns "hook doesn't source guardrail checks"
```

## Edge cases

1. **Hook not executable:** After scaffold, set chmod 755. Check permissions on validate.
2. **core.hooksPath not set:** Generate warns + sets it. But user might use a different hooks mechanism (husky, lefthook).
3. **User uses husky/lefthook:** These tools manage hooks differently. guardrail3 should detect them and NOT scaffold a conflicting hook. Check for `.husky/`, `lefthook.yml`, `.lintstagedrc`.
