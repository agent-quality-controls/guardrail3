# RS-FRONTEND-UI — Dioxus UI, accessibility, and styling checker

**Input:** Rust UI/component files + generated markup/template IR + theme/token config + compiled style artifacts
**Parser:** Rust AST + frontend/template extraction + structured style/config parsing
**Current code:** None yet — new family needed for Rust UI semantics

## Scope

This family carries over the *semantic* parts of the old JSX a11y/stylelint/tailwind-ban plans into Rust frontend code.

It should validate markup semantics, accessibility, and design-token discipline in Dioxus-style UI code and generated frontend artifacts, without carrying over JS package/plugin mechanics.

## Rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FRONTEND-UI-01 | Error | Interactive controls have accessible labels | Planned |
| RS-FRONTEND-UI-02 | Error | Clickable non-interactive elements require valid keyboard/focus semantics | Planned |
| RS-FRONTEND-UI-03 | Error | Invalid/redundant ARIA roles and unsupported ARIA props are rejected | Planned |
| RS-FRONTEND-UI-04 | Warn | Heading/landmark/form structure must be semantically valid | Planned |
| RS-FRONTEND-UI-05 | Warn | Images/media/iframes require accessibility metadata (`alt`, captions, titles) | Planned |
| RS-FRONTEND-UI-06 | Warn | No `autofocus`, `accesskey`, or other hostile interaction defaults without explicit exemption | Planned |
| RS-FRONTEND-UI-07 | Error | Raw/unsafe HTML injection is banned unless explicitly audited and documented | Planned |
| RS-FRONTEND-UI-08 | Error | Design tokens/theme APIs must be used instead of raw colors/spacing/typography/z-index magic values | Planned |
| RS-FRONTEND-UI-09 | Warn | No `outline: none` / focus suppression without explicit replacement behavior | Planned |
| RS-FRONTEND-UI-10 | Warn | Reduced-motion handling is required for significant animation/motion patterns | Planned |
| RS-FRONTEND-UI-11 | Warn | No inaccessible text styling patterns (excessive letter spacing, unreadable sizing, justified body text) | Planned |
| RS-FRONTEND-UI-12 | Warn | Meaningful content must not be hidden with unsafe display/visibility patterns | Planned |
| RS-FRONTEND-UI-13 | Warn | Inline style escape hatches or custom ARIA escape hatches require explicit reason/inventory | Planned |
| RS-FRONTEND-UI-14 | Info | UI/component complexity inventory: large components, oversized prop surfaces, duplicate local style systems | Planned |
| RS-FRONTEND-UI-15 | Error | Input failures for UI/template/style parsing fail closed | Planned |

## Notes

- This family should validate semantics, not stylelint/eslint package presence.
- If styling lives partly in generated CSS artifacts and partly in Rust-side declarations, both surfaces should be normalized by the orchestrator and checked here.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| stylelint config/plugin presence | JS toolchain concern, not Rust frontend policy |
| tailwind package/plugin enforcement | Mechanism-specific; keep token/style semantics only |
| generic formatter/style preference rules | Not guardrails |
