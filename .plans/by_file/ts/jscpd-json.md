# .jscpd.json

## Location

**Where jscpd looks:** In the path argument (defaults to CWD). No walk-up. Can specify `--config` flag, or `"jscpd"` key in package.json.

**In steady-parent:**
- Root `.jscpd.json` (28 lines) — threshold 10, format ["typescript", "rust"], 18 ignore patterns
- `apps/validator-rust/.jscpd.json` (9 lines) — threshold 5, language ["rust"], 2 ignore patterns

## Contents (root, verified)

```json
{
  "$schema": "...",
  "threshold": 10,              ← PROJECT (guardrail3 default is 0, but see below)
  "minTokens": 50,              ← GUARDRAIL
  "reporters": ["consoleFull"], ← GUARDRAIL
  "ignore": [                   ← MIXED (guardrail base patterns + project-specific)
    "**/node_modules/**",       ← guardrail
    "**/.next/**",              ← guardrail
    "**/dist/**",               ← guardrail
    "**/target/**",             ← guardrail
    "**/components/ui/**",      ← guardrail (shadcn)
    "**/components/pro-blocks/**", ← project
    "**/drizzle/**",            ← project
    "**/__tests__/**",          ← guardrail
    "**/__mocks__/**",          ← guardrail
    "**/test/**",               ← guardrail
    "**/*.generated.*",         ← guardrail
    "**/coverage/**",           ← guardrail
    "**/.plans/**",             ← guardrail
    "**/.worklogs/**",          ← guardrail
    "**/lib/utils.ts",          ← project
    "**/golden/**",             ← guardrail (test fixtures)
    "**/legacy/**",             ← project
    "content/**",               ← project
    "packages/validation/**"    ← project
  ],
  "format": ["typescript", "rust"], ← PROJECT (which languages to check)
}
```

## Category: Merge-managed

**guardrail3 manages:**
- `minTokens`: ensure present (default 50)
- `reporters`: ensure present
- `ignore` array: ensure guardrail base patterns present, LEAVE user's extra patterns
- `$schema`: ensure present

**User owns:**
- `threshold`: project choice. guardrail3 default is 0 but this is impractical without project-specific ignore patterns. WARN if looser than 0 but don't force.
- `format`/`language`: project choice (which languages to scan)
- Extra ignore patterns beyond guardrail base
- `minLines`, `absolute`, and other jscpd options

## Algorithm

```
1. Parse as JSON (serde_json)
2. threshold: LEAVE (validate warns if > guardrail baseline)
3. minTokens: if missing ADD 50, if present LEAVE
4. reporters: if missing ADD ["consoleFull"], if present LEAVE
5. ignore array: for each guardrail pattern, if missing ADD. User patterns: LEAVE.
6. All other keys: LEAVE
7. Write back as formatted JSON
```

**guardrail base ignore patterns (11):**
`**/node_modules/**`, `**/.next/**`, `**/dist/**`, `**/target/**`, `**/components/ui/**`, `**/__tests__/**`, `**/__mocks__/**`, `**/test/**`, `**/*.generated.*`, `**/coverage/**`, `**/.plans/**`, `**/.worklogs/**`, `**/golden/**`

## Per-app .jscpd.json

`apps/validator-rust/.jscpd.json` exists with completely different config (Rust only, threshold 5, different ignores). This is a per-app choice. guardrail3 treats per-app jscpd as validate-only — don't touch, don't generate.

## Edge cases

1. **ignore array dedup:** If guardrail pattern already present, don't add duplicate. Match by exact string.
2. **JSON comments:** .jscpd.json is standard JSON (not JSONC). No comment preservation issue.
3. **format vs language:** Older jscpd uses `format`, newer uses `language`. Both work. Don't change what the user has.
