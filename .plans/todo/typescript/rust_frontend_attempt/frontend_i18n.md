# RS-FRONTEND-I18N — Rust frontend locale and translation checker

**Input:** Locale config + route manifests + localized content/message artifacts + Rust route helpers
**Parser:** Structured config parsing + generated content/message facts + targeted Rust AST checks
**Current code:** None yet — new family needed for Rust frontend i18n validation

## Scope

This family carries over the locale-correctness part of the old content/frontend planning into Rust-rendered frontend work.

It should validate locale routing, translation/content parity, and the absence of hardcoded locale-path bypasses.

## Rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FRONTEND-I18N-01 | Error | Canonical locale config exists and matches frontend route/content generation | Planned |
| RS-FRONTEND-I18N-02 | Error | No hardcoded locale path literals where typed route/locale helpers should be used | Planned |
| RS-FRONTEND-I18N-03 | Error | Required localized content/message artifacts exist for each configured locale | Planned |
| RS-FRONTEND-I18N-04 | Warn | Locale fallback/default-locale behavior is explicit and centralized | Planned |
| RS-FRONTEND-I18N-05 | Warn | Locale coverage/parity inventory (missing translations, missing localized content) | Planned |
| RS-FRONTEND-I18N-06 | Error | Locale config/content parsing failures fail closed | Planned |

## Notes

- This family is about locale correctness, not generic frontend routing.
- It should be driven by typed locale/route/content facts, not raw string grep except for targeted literal-bypass detection.
