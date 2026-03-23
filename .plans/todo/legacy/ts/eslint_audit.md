# ESLint Config Checker — Adversarial Audit

Audited: `eslint_check.rs`, `eslint_audit.rs`, `eslint_plugin_checks.rs`, `eslint_parser.rs`, `eslint_rule_infra.rs`
Current coverage: 63 rules (TS-ESLINT-01 through TS-ESLINT-63)

---

## HIGH: Severity validation is presence-only for value rules

**What:** `check_eslint_rule` with `expected_value: None` delegates to `check_eslint_rule_presence`, which only verifies `severity == "error"`. But the 3 value rules (T2 max-lines, T3 max-lines-per-function, T4 complexity) check the numeric value without also verifying severity is "error". A config with `["warn", 400]` passes T2 because 400 <= 400, but the rule won't fail the build.

**Fix:** `check_eslint_rule` should verify severity is "error" AND the numeric value is correct. Both conditions must hold.

**Priority:** HIGH — a rule set to "warn" provides zero enforcement in CI.

---

## HIGH: Numeric threshold gaming via absence

**What:** `check_rule_value` returns `false` when `numeric_value` is `None` for a numeric check. But the parser's `extract_first_number` skips values <= 2. A rule like `["error", { max: 1 }]` would have `numeric_value: None` (1 is skipped as a potential severity alias), causing the check to report a value mismatch even though 1 is stricter than 400. Conversely, `["error"]` without any numeric config also fails — but `max-lines` with no number means "no limit", which should absolutely fail.

The real problem: there is no way to distinguish "rule has no numeric config" (permissive — should fail) from "parser couldn't extract the number" (bug — should warn). Both return `None`.

**Fix:** The parser should return a tri-state: `Configured(u32)`, `NotConfigured`, `ParseFailed`. `NotConfigured` means the rule is present but has no threshold — should be an error for rules that need one. `ParseFailed` should be a warning about parser limitations.

**Priority:** HIGH — false negatives on rules with no threshold configured.

---

## HIGH: Missing React hooks rules

**What:** Neither `react-hooks/exhaustive-deps` nor `react-hooks/rules-of-hooks` appear anywhere in the checked rules. These are the two most critical React rules — `rules-of-hooks` prevents hooks in conditionals/loops (causes runtime crashes), `exhaustive-deps` prevents stale closures (causes subtle data bugs).

The `REACT_EXTRA` list has 10 React rules but omits the hooks plugin entirely.

**Fix:** Add `react-hooks/rules-of-hooks` and `react-hooks/exhaustive-deps` to a new check or to `REACT_EXTRA`. Both must be severity "error".

**Priority:** HIGH — hooks violations cause runtime crashes and are the #1 source of React bugs.

---

## HIGH: Missing Next.js rules (content profile)

**What:** No Next.js-specific rules are checked. For content-profile projects using Next.js, the following are critical:
- `@next/next/no-img-element` — forces `next/image` for automatic optimization
- `@next/next/no-head-element` — forces `next/head` component
- `@next/next/no-html-link-for-pages` — forces `next/link` for client-side navigation
- `@next/next/no-sync-scripts` — prevents synchronous scripts blocking render

The content-profile checks (`check_content_plugins`) handle jsx-a11y and tailwind-ban but nothing Next.js-specific.

**Fix:** Add a `check_nextjs_rules` function in `eslint_plugin_checks.rs` gated on content profile, checking for `@next/next/*` rules.

**Priority:** HIGH — Next.js without its ESLint plugin loses image optimization, routing, and performance guardrails.

---

## MEDIUM: No detection of `eslint-disable` in the config file itself

**What:** The source scan checks (T23-T26 in `source_scan.rs`) catch `eslint-disable` in source files. But nobody checks whether `eslint.config.mjs` itself contains `eslint-disable` comments, which would silently disable rules in the config file that configures the rules — a meta-bypass.

**Fix:** After parsing, scan `config.raw_content` for `eslint-disable` patterns. Any match is an error: the config file must not suppress its own linting.

**Priority:** MEDIUM — unlikely to happen accidentally, but a deliberate agent bypass vector.

---

## MEDIUM: Missing import order/sorting rules

**What:** No check verifies that `import-x/order` (or `import/order`) is configured. Without it, imports are randomly ordered, making diffs noisy and merge conflicts frequent. The current checks verify `no-cycle` (T45) and `max-dependencies` (T46) from import plugins but not ordering.

Additionally, `import-x/no-duplicates` is not checked — duplicate imports of the same module compile fine but waste bundle size and confuse readers.

**Fix:** Add checks for `import-x/order` (or `import/order`) and `import-x/no-duplicates`.

**Priority:** MEDIUM — not a correctness issue, but a significant DX and merge-conflict issue.

---

## MEDIUM: Missing testing rules

**What:** No vitest/jest/testing-library rules are checked. Critical testing rules:
- `vitest/no-focused-tests` (or `jest/no-focused-tests`) — `.only` left in test files skips all other tests in CI
- `vitest/no-disabled-tests` — `.skip` hides failing tests
- `testing-library/no-debugging-utils` — `screen.debug()` left in tests
- `testing-library/no-wait-for-empty-callback` — empty `waitFor()` is always a bug
- `testing-library/prefer-screen-queries` — enforces consistent query patterns

The `TEST_RELAXATION_RULES` list covers what rules to relax for tests, but not what test-specific rules should be enabled.

**Fix:** Add a check for test plugin presence (vitest or jest) and key rules. Could be a new plugin group like `T-ESLP-16`.

**Priority:** MEDIUM — `.only` in CI is a silent test-suite killer.

---

## MEDIUM: `check_file_overrides` uses raw string matching, not AST

**What:** T8 scans raw content line-by-line for `files:` or `files =` patterns. This catches string literals in comments, import statements, or unrelated code. For example, a comment `// TODO: handle files: migration` would trigger a false positive.

The tree-sitter parser already identifies objects with `files:` properties (via `object_has_test_files`). The override detection should use the same structural approach.

**Fix:** During AST walking, collect all config objects that have a `files:` property and report those as overrides. Remove the raw-line scan.

**Priority:** MEDIUM — false positives in reporting, not false negatives in enforcement.

---

## MEDIUM: No detection of conflicting rules

**What:** Certain ESLint rule pairs conflict:
- `@typescript-eslint/no-unused-vars` + `no-unused-vars` (base rule must be off when TS version is on)
- `@typescript-eslint/no-shadow` + `no-shadow` (same conflict)
- `@typescript-eslint/no-use-before-define` + `no-use-before-define` (same pattern)
- `promise-function-async` + `require-await` can fight: one demands async, the other demands await inside async

When both the base ESLint rule and the `@typescript-eslint/` version are set to "error", TypeScript files get double-reported and the base rule produces false positives on type-only constructs.

**Fix:** After parsing all rules, check for known conflicting pairs. If `@typescript-eslint/X` is present, verify that `X` (base version) is "off".

**Priority:** MEDIUM — causes confusing double-errors and false positives in CI output.

---

## MEDIUM: Rule deprecation / removal detection

**What:** typescript-eslint v8 renamed/removed several rules:
- `@typescript-eslint/no-throw-literal` -> `@typescript-eslint/only-throw-error` (v8)
- `@typescript-eslint/no-var-requires` -> removed (covered by `verbatimModuleSyntax`)
- `@typescript-eslint/ban-types` -> `@typescript-eslint/no-restricted-types` (v8)
- `@typescript-eslint/no-empty-function` -> scoped differently in v8

The checker currently looks for `no-throw-literal` (T82) which is the old name. If a project uses typescript-eslint v8, they'd have `only-throw-error` and T82 would report it as missing.

**Fix:** For rules with known renames, check both old and new names. Emit a deprecation warning if the old name is found. The `BUILTIN_RULES` list already has `only-throw-error` but `check_all_eslint_rules` still checks for `no-throw-literal` — these are inconsistent.

**Priority:** MEDIUM — false negatives on projects using typescript-eslint v8+.

---

## MEDIUM: `check_eslint_rule_presence` doesn't verify rules provided by presets

**What:** `strictTypeChecked` preset includes rules like `no-floating-promises`, `no-unsafe-assignment`, `await-thenable`, etc. If these are enabled by the preset but not explicitly listed in the `rules:` block, the checker reports them as missing. This creates noise — the rules ARE active, just provided by the preset spread.

Currently there is no way to know which rules a preset provides (that would require knowing the preset's contents). But the checker could at least note that when `strictTypeChecked` is present, certain rules may be covered implicitly.

**Fix:** When `strictTypeChecked` preset is detected, downgrade missing-rule severity from Error to Warn for rules known to be in that preset, with a message noting they may be covered by the preset. Alternatively, skip those checks entirely when the preset is present.

**Priority:** MEDIUM — false positives causing noise, but not dangerous.

---

## LOW: No validation of `no-restricted-imports` configuration content

**What:** T5 checks that `no-restricted-imports` is present but not what it restricts. An empty `no-restricted-imports: "error"` with no `patterns` or `paths` is useless — it bans nothing. For guardrail purposes, this rule should at minimum ban certain dangerous packages (e.g., `lodash` full import, `moment`, `axios` if `fetch` is preferred).

**Fix:** Check that `no-restricted-imports` has at least one `patterns` or `paths` entry in its configuration. This requires extending the parser to extract array/object options for specific rules.

**Priority:** LOW — the rule being present without content is better caught during template generation review.

---

## LOW: No `no-return-await` or `@typescript-eslint/return-await` check

**What:** `return await` inside a try/catch changes error handling behavior (the await is needed), but outside try/catch it's a pointless extra microtask. `@typescript-eslint/return-await` with `in-try-catch` option handles this correctly. The `BUILTIN_RULES` list is missing this.

**Fix:** Add `@typescript-eslint/return-await` to `BUILTIN_RULES` or a dedicated check.

**Priority:** LOW — minor performance/correctness edge case.

---

## LOW: Boundary check uses raw content fallback too liberally

**What:** `check_zone_definitions` (T36) falls back to checking raw content for strings like "domain" and "adapters". A comment mentioning these words triggers a false pass. The check should prefer the structured `config.rules` data and only use raw content as a secondary signal with lower confidence.

**Fix:** If `boundaries/element-types` is not in `config.rules`, check raw content but emit as Warn (possible but unverified) instead of Info (confirmed).

**Priority:** LOW — false pass is possible but the boundary import check (T6) would still fail if the plugin isn't actually configured.

---

## LOW: `check_test_relaxations` T-ESLP-11 uses `find_missing_rules` which doesn't check test context

**What:** `find_missing_rules` checks if rules exist in `config.rules` at all. But test relaxation rules should specifically exist in a test override block (with `is_test_override: true`). A rule relaxed globally would satisfy the check even though it's not scoped to tests — which is worse than having no relaxation at all.

**Fix:** `find_missing_rules` for test relaxations should check that the rules exist AND are in a test override context. The parser already tracks `is_test_override`.

**Priority:** LOW — edge case where global relaxation would mask the issue.

---

## LOW: No `curly` rule check

**What:** The `curly` rule enforces braces around all control flow blocks (`if`, `else`, `for`, `while`). Without it, adding a second line to an unbraced `if` block is a common source of bugs. This is a basic safety rule missing from all checked lists.

**Fix:** Add `curly` to `BUILTIN_RULES` or the core rule presence checks.

**Priority:** LOW — most formatters (Prettier) handle this, but the ESLint rule catches it earlier.
