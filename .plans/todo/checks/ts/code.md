# TS-CODE — TypeScript source scan checker

**Input:** `*.ts`, `*.tsx`, `*.mjs`
**Parser:** tree-sitter TypeScript/TSX + structured source scanning
**Current code:** `app/ts/validate/source_scan.rs`, `ts_comment_checks.rs`, `ts_code_analysis.rs`, `ast_helpers.rs`
**Owned root:** TS package/app root

## Owns

- source-level comment suppression checks
- direct `process.env` access
- `any` usage inventory/policy
- file-length and other local source-shape limits
- banned package presence in `node_modules` if kept as source-scan policy
- generic source-string/source-pattern policy that is not specific to i18n, SEO, or architecture

## Does not own

- architecture/import boundary enforcement
  - that belongs to `ts/hexarch`
- test-specific quality rules
  - that belongs to `ts/tests`
- i18n/message completeness
  - that belongs to `ts/i18n`
- translation-surface correctness in i18n-enabled apps
  - that belongs to `ts/i18n`

## Contract direction

This is the direct TS analogue of Rust `rs/code`:
- local source assertions
- AST-backed where possible
- no project-architecture ownership
