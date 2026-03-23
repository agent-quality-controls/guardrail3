# HOOK-TS — TypeScript-specific hook step checker (7 rules)

**Input:** .githooks/pre-commit script content (pattern matching in executable lines)
**Current code:** `hook_script_checks.rs` (H-TOOL-*, H-CSS-01), `tool_checks.rs` (H12)

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| HOOK-TS-01 | H-TOOL-01 | Warn | cspell spell-checking step present | Implemented |
| HOOK-TS-02 | H-TOOL-02 | Warn | Merge conflict marker check step present | Implemented |
| HOOK-TS-03 | H-TOOL-03 | Warn | Lockfile integrity check (frozen-lockfile) step present | Implemented |
| HOOK-TS-04 | H-TOOL-04 | Warn | Prettier format check step present | Implemented |
| HOOK-TS-05 | H-TOOL-05 | Warn | pnpm audit vulnerability scan step present | Implemented |
| HOOK-TS-06 | H-CSS-01 | Warn | Stylelint CSS lint step present (content profile) | Implemented |
| HOOK-TS-07 | H12 (partial) | Warn | Duplication tool: jscpd for TS projects (not cargo-dupes) | Implemented |
