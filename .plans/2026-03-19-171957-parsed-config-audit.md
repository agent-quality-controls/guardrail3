# Parsed config audit: guardrail3 baselines vs steady-parent

**Date:** 2026-03-19 17:19
**Task:** Parse every config file on both sides and diff them to find gaps

## Approach
For each config type, parse the actual file and extract structured data, then compare.

### 1. ESLint — use `eslint --print-config` to get resolved rules from steady-parent
Run from steady-parent: `pnpm exec eslint --print-config apps/web/src/app/page.tsx`
This gives the FULL resolved rule set (including presets like strictTypeChecked).
Compare every rule against what guardrail3 checks in eslint_check.rs.

### 2. tsconfig — use `tsc --showConfig` to get resolved settings
Run from steady-parent: `cd apps/web && pnpm exec tsc --showConfig`
Compare every compilerOption against what guardrail3 checks.

### 3. npmrc — parse line by line
Both files are simple key=value. Parse and diff.

### 4. jscpd — parse JSON
Both are JSON. Parse and diff every field.

### 5. package.json — parse JSON
Extract devDependencies, scripts, engines, pnpm.overrides from steady-parent.
Compare against guardrail3's required lists.
