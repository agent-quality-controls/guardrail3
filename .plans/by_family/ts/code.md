# TS-CODE

Status: current family contract, legacy-grouped implementation.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
- `apps/guardrail3/crates/app/ts/validate/ts_code_analysis.rs`
- `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/code.md` as the detailed family ledger until the cutover is complete

Current state:

- source scanning exists, but still lives in the old grouped TS validator
- no dedicated family workspace/README yet

Rule inventory:

- `T23` — block-level `eslint-disable` without reason is forbidden.
  - What it should do: error on block `eslint-disable` comments that do not explain themselves.
  - What it is for: broad lint suppression is too dangerous to leave undocumented.
- `T24` — block-level `eslint-disable` with reason is inventoried.
  - What it should do: inventory block suppressions that include a reason.
  - What it is for: documented suppressions still need audit visibility.
- `T25` — line-level `eslint-disable` without reason is forbidden.
  - What it should do: error on `eslint-disable-next-line` / `eslint-disable-line` comments without a reason.
  - What it is for: local suppressions should still justify themselves.
- `T26` — line-level `eslint-disable` with reason is inventoried.
  - What it should do: inventory line-level suppressions with reasons.
  - What it is for: documented local suppressions are still drift-prone and should stay visible.
- `T27` — `@ts-ignore` is forbidden.
  - What it should do: error on `@ts-ignore`.
  - What it is for: it suppresses type errors too broadly and does not fail when the underlying error disappears.
- `T28` — undocumented `@ts-expect-error` is warned.
  - What it should do: warn when `@ts-expect-error` has no reason.
  - What it is for: `@ts-expect-error` is safer than `@ts-ignore`, but it still needs rationale.
- `T29` — documented `@ts-expect-error` is inventoried.
  - What it should do: inventory `@ts-expect-error` comments with explanations.
  - What it is for: even justified type suppressions should remain auditable.
- `T30` — direct `process.env` access is forbidden.
  - What it should do: error on direct `process.env` reads outside allowed env/config files, with inventory-only behavior when explicitly eslint-suppressed.
  - What it is for: environment access should flow through a central validated config surface.
- `T31` — `any` usage is inventoried.
  - What it should do: detect `: any` and `as any`.
  - What it is for: this keeps type escapes visible even before deciding whether they become hard errors.
- `T32` — oversized files are forbidden.
  - What it should do: error on files above the effective-line threshold.
  - What it is for: large files are hard to reason about and resist clean refactoring.
- `T34` — JetBrains `noinspection` directives are inventoried.
  - What it should do: inventory IDE-only suppression comments.
  - What it is for: IDE-local suppression policy should not quietly live in source control.
- `T35` — coverage ignore directives are inventoried.
  - What it should do: inventory `istanbul ignore`, `c8 ignore`, and similar directives.
  - What it is for: hidden coverage gaps should remain visible even when intentionally accepted.
- `T59` — banned packages in `node_modules` are forbidden.
  - What it should do: error when banned direct or transitive packages are present under `node_modules`.
  - What it is for: this extends dependency policy into the actually installed tree, not only manifests.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/source_scan.rs`
  - orchestrates `T23`..`T35` and `T59`
  - skips `T23`..`T31` for test files
- `apps/guardrail3/crates/app/ts/validate/ts_comment_checks.rs`
  - implements `T23`..`T29`
- `apps/guardrail3/crates/app/ts/validate/ts_code_analysis.rs`
  - provides AST helpers for `T30`, `T31`, and test-method detection
- `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`
  - provides tree-sitter parsing/comment extraction shared by the family

Current doc/code reconciliation notes:

- the old ledger is broadly aligned with the live code
- the family currently skips most source-quality checks for test files; that boundary should stay explicit when reconciling against `ts/tests`
- `T59` blurs package-policy and source-scan ownership; the family currently owns it in code, but it should be re-evaluated against `ts/package`

Historical/supplemental references:

- `.plans/todo/checks/ts/code.md`

Next planning focus:

- separate generic TS source rules from architecture, tests, i18n, and content concerns
- decide whether `T31` (`any` usage) stays inventory-only or becomes a stricter policy rule
