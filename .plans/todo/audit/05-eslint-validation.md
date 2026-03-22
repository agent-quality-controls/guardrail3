# Adversarial Audit: ESLint Validation (T1-T8, T36-T51, T60-T83, T-ESLP-*)

**Auditor:** Adversarial agent
**Files audited:**
- `eslint_check.rs` — T1-T8, T36-T51, T60-T83, T-ESLP-13/14
- `eslint_plugin_checks.rs` — T-ESLP-01 through T-ESLP-12
- `eslint_rule_infra.rs` — `check_eslint_rule`, `check_eslint_rule_presence`, `check_rule_value`
- `eslint_audit.rs` — T36-T39
- `eslint.config.mjs` (actual config) — ground truth

---

## CRITICAL: Flat Config Override Blindness (affects ALL checks)

### F01: Later config blocks can disable required rules — UNDETECTED

**Severity: CRITICAL**

ESLint flat config is an array of config objects. Later objects override earlier ones. The entire validation system uses `content.contains(rule_name)` which checks if a rule name appears ANYWHERE in the file. It does NOT check what the rule is set to.

**Attack vector:** Add a config block at the end of the file:
```js
// At the end of eslint.config.mjs
{
  files: ["**/*.ts", "**/*.tsx"],
  rules: {
    "@typescript-eslint/no-floating-promises": "off",
    "@typescript-eslint/no-explicit-any": "off",
    // ... disable everything
  }
}
```

guardrail3 sees `no-floating-promises` in the file (from the first block where it's `"error"`) and reports PASS. But ESLint applies the LAST matching config, which sets it to `"off"`.

**Impact:** Every single rule check (T2-T5, T40-T48, T60-T83, all T-ESLP-* rule groups) is vulnerable. An agent can silently disable any rule by adding a later override block.

**How the real config already does this legitimately:** The test file override block (line 260-271) sets `max-lines: "off"`, `complexity: "off"`, etc. guardrail3's T7 check inventories `"off"` lines, but does NOT cross-reference them against the required rules. So the test relaxation is invisible to the rule presence checks.

### F02: Comments containing rule names produce false passes

**Severity: HIGH**

`content.contains("no-floating-promises")` matches:
- `// TODO: add no-floating-promises later` — commented out, not configured
- `// Removed no-floating-promises because it was too strict` — explicitly removed
- `/* "@typescript-eslint/no-floating-promises": "error", */` — commented-out config

No check strips comments before matching.

**Impact:** All presence checks (T40-T48, T60-T83, all T-ESLP-* rule groups). A rule can be completely absent from active config while appearing in a comment.

### F03: String literals in banned-import messages produce false passes

**Severity: MEDIUM**

The actual `eslint.config.mjs` contains messages like:
```js
{ name: "moment", message: "Use date-fns or Intl — moment is dead and 300KB" }
```

If a check searched for `"moment"` or `"date-fns"`, it would match inside the message string of a banned-import rule, not in an actual rule configuration. This doesn't affect current checks (they search for specific rule names like `no-floating-promises`), but the pattern is fragile.

---

## CRITICAL: Value Checks (T2-T4)

### F04: `max-lines` expected value mismatch — checker expects 400, actual config is 300

**Severity: CRITICAL (wrong threshold)**

`eslint_check.rs` line 62: `check_eslint_rule(... "max-lines", Some("400"), ...)`

The actual `eslint.config.mjs` line 200: `"max-lines": ["error", { max: 300, ... }]`

The checker expects 400 and uses `<=` comparison (actual <= expected means "stricter, pass"). Since 300 <= 400, this PASSES. But the checker's intent is to verify the config matches the template. If someone changes max-lines to 399, it still passes — but the template says 400, and the actual says 300. **The expected value is wrong.** It should be 300 to match the actual config, or the design should be clarified.

### F05: `check_rule_value` extracts the FIRST number near the rule name — wrong for object configs

**Severity: HIGH**

`extract_number_from_line` returns the first number it finds on a line (or within 5 lines of the rule name).

For the actual config:
```js
"max-lines": ["error", { max: 300, skipBlankLines: true, skipComments: true }],
```

The function scans line-by-line looking for a number. It will find `300` on the line containing `max: 300`. But consider:
```js
"max-lines": ["error", { skipBlankLines: true, max: 9999 }],
```
If `skipBlankLines` and `max` are on different lines, it might find the wrong number first. More dangerously:
```js
// Max lines should be 300 for production
"max-lines": ["error", { max: 9999 }],
```
The comment `300` appears first (though comment lines are skipped). The real risk: any number appearing within 5 lines of the rule name could be mistakenly matched.

### F06: `check_rule_value` returns FIRST match — if rule appears multiple times, only checks the first occurrence

**Severity: HIGH**

The function iterates lines and returns on the first rule name match (line 256-278). If a rule appears in multiple config blocks:
```js
// Block 1: strict
"max-lines": ["error", { max: 300 }],
// Block 2: override for specific files
"max-lines": "off",
```
Only Block 1 is checked. Block 2 (which overrides to "off") is ignored.

### F07: Number extraction skips the rule severity number

**Severity: LOW (partially mitigated)**

ESLint uses numeric severity: `0` = off, `1` = warn, `2` = error. If a rule is configured as:
```js
"max-lines": [2, { max: 300 }]
```
The `2` and `300` are both numbers. `extract_number_from_line` returns the FIRST number found. If `[2, { max: 300 }]` is on one line, `2` would be extracted, and `2 <= 400` passes — for the WRONG reason. In practice this works by accident because `2` is always <= any reasonable threshold, but the logic is unsound.

---

## Boundary Enforcement (T6, T36-T39)

### F08: T6 false positive on the word "boundaries" in ANY context

**Severity: HIGH**

`eslint_check.rs` line 97: `content.contains("boundaries") || content.contains("eslint-plugin-boundaries")`

The word "boundaries" appears in comments, messages, or even other strings. A file with:
```js
// TODO: add boundaries enforcement later
```
passes T6.

### F09: T36 zone definitions — operator precedence bug

**Severity: CRITICAL (logic bug)**

`eslint_audit.rs` line 29-31:
```rust
let has_zones = content.contains("element-types")
    || content.contains("domain")
        && (content.contains("commands") || content.contains("adapters"));
```

Due to Rust operator precedence, `&&` binds tighter than `||`. This evaluates as:
```rust
content.contains("element-types")
    || (content.contains("domain") && (content.contains("commands") || content.contains("adapters")))
```

This is actually correct for the intended logic. BUT: the word "domain" appears in comments, rule messages, and CLAUDE.md references. The word "adapters" similarly. Any ESLint config that mentions these words in comments passes T36 even with zero boundary zones configured.

### F10: T37-T39 — same substring matching problem

**Severity: HIGH**

- T37 checks `content.contains("boundaries/element-types")` — more specific, less likely to false-match
- T38 checks `content.contains("boundaries/entry-point")` — same
- T39 checks `content.contains("boundaries/external")` — same

These are better than T36 but still match comments. A commented-out block passes.

### F11: T6 + T36-T39 don't verify rules are set to "error"

**Severity: HIGH**

The boundaries plugin could be imported and configured, but with all rules set to `"warn"` or `"off"`. The checks only verify the string exists, not the severity. Someone could write:
```js
"boundaries/element-types": "off",
```
and T37 passes because the string `boundaries/element-types` is present.

---

## Preset Checks (T-ESLP-13, T-ESLP-14)

### F12: Comment bypass for presets

**Severity: HIGH**

`eslint_check.rs` line 128: `content.contains("strictTypeChecked")`

Matches:
- `// strictTypeChecked` — commented out
- `// Removed strictTypeChecked` — explicitly removed
- `"// ...tseslint.configs.strictTypeChecked"` — inside a string
- `// TODO: re-enable strictTypeChecked` — aspirational comment

Same for `stylisticTypeChecked` on line 159.

### F13: Spread operator not verified

**Severity: MEDIUM**

The actual config uses `...tseslint.configs.strictTypeChecked` (spread into the array). The check only verifies the string `strictTypeChecked` exists. Someone could write:
```js
const x = tseslint.configs.strictTypeChecked; // assigned but never used
```
and pass.

---

## Route Wrapper Enforcement (T50)

### F14: Only checks string existence, not rule structure

**Severity: HIGH**

`eslint_check.rs` line 342: `content.contains("withBody") || content.contains("withRoute")`

The actual config (lines 296-327) uses `no-restricted-syntax` with specific AST selectors (`ExportNamedDeclaration > FunctionDeclaration`, etc.) to ban raw route exports. T50 only checks if the strings `withBody`/`withRoute` appear anywhere — which they do in the MESSAGE strings, not in the selector patterns.

So even if someone removed all the `no-restricted-syntax` rules and just left a comment:
```js
// We used to enforce withBody/withRoute wrappers
```
T50 passes.

### F15: Does not verify all four wrapper variants

**Severity: MEDIUM**

The actual config bans raw exports and requires `withBody/withRoute/withPublicBody/withPublicRoute`. T50 only checks for `withBody` and `withRoute`. If the public variants were removed, T50 still passes.

### F16: Does not verify the AST selectors are correct

**Severity: MEDIUM**

The actual config uses four AST selectors to catch different export patterns:
1. `ExportNamedDeclaration > FunctionDeclaration`
2. `ExportNamedDeclaration > VariableDeclaration > VariableDeclarator > ArrowFunctionExpression`
3. `ExportNamedDeclaration > VariableDeclaration > VariableDeclarator > FunctionExpression`
4. `ExportNamedDeclaration[declaration=null] > ExportSpecifier`

T50 doesn't verify any of these. Someone could remove selectors 2-4 and only ban function declarations, letting arrow functions through.

---

## process.env Ban (T51)

### F17: Only checks string existence, not rule structure

**Severity: HIGH**

`eslint_check.rs` line 376: `content.contains("process.env")`

The actual config uses `no-restricted-syntax` with AST selector `MemberExpression[object.name='process'][property.name='env']`. T51 only checks if the string "process.env" appears in the file. This matches:
- The MESSAGE text: `"Use env from @/lib/env instead of process.env directly"` (line 285)
- A comment mentioning process.env
- Even the `ignores` line: `"**/lib/env.ts"` (no, this doesn't contain "process.env")

Since the message string contains `process.env`, T51 will ALWAYS pass as long as the message exists, even if the actual AST selector rule is removed.

### F18: Does not verify the env.ts exemption

**Severity: MEDIUM**

The actual config exempts `**/lib/env.ts` from the process.env ban (line 277). This is correct — env.ts is the centralized env module. But T51 doesn't verify this exemption exists and is narrow. An agent could expand the exemption to `**/*.ts` and T51 would still pass.

---

## Plugin Checks (T-ESLP-01 through T-ESLP-12)

### F19: `find_missing_rules` uses `content.contains` — same comment/override blindness

**Severity: HIGH**

`eslint_plugin_checks.rs` line 112: `!content.contains(**rule)` — every rule in every group (UNICORN_DISABLED, UNICORN_EXTRA, REGEXP_EXTRA, SONARJS_RULES, REACT_EXTRA, BUILTIN_RULES, TEST_RELAXATION_RULES) is checked with substring matching. All vulnerable to F01 and F02.

### F20: UNICORN_DISABLED rules — checked for presence, not for "off" setting

**Severity: HIGH**

T-ESLP-02 checks that disabled unicorn rules are present. The intent is that these rules should be set to `"off"`. But `find_missing_rules` only checks if the rule NAME appears in the file. A config where `"unicorn/no-null": "error"` passes T-ESLP-02 — the rule is present, but it's NOT disabled.

### F21: T-ESLP-04 regexp marker "flat/recommended" is too generic

**Severity: MEDIUM**

`eslint_plugin_checks.rs` line 236: checks for `"regexp"` AND `"flat/recommended"`. But `flat/recommended` also appears in the unicorn import check (line 321). If unicorn's `flat/recommended` is present but regexp is missing, the check could pass because both strings exist (from different plugins).

Actually looking more carefully: the check requires BOTH `"regexp"` AND `"flat/recommended"`. Since `"regexp"` is specific, this is less likely to false-match. But if someone writes a comment `// regexp: consider using flat/recommended`, it passes.

### F22: T-ESLP-07 jsx-a11y strict — "strict" is too generic

**Severity: HIGH**

`eslint_plugin_checks.rs` line 414: `content.contains("strict")`.

The word "strict" appears in multiple places in the actual config:
- `strictTypeChecked` (line 136)
- `"@typescript-eslint/strict-boolean-expressions"` (line 218)

So T-ESLP-07 will ALWAYS pass in any config that has `strictTypeChecked` or `strict-boolean-expressions`, even without jsx-a11y. The check requires BOTH `jsxA11y`/`jsx-a11y` AND `strict`, so the first condition gates it — but if someone writes `// jsxA11y: consider adding strict mode`, both conditions match from a comment.

### F23: T-ESLP-11 test relaxation — doesn't verify rules are set to "off"

**Severity: MEDIUM**

The check verifies the TEST_RELAXATION_RULES names appear somewhere in the file. But these rules (like `max-params`, `sonarjs/cognitive-complexity`) also appear in the MAIN config where they're set to `"error"`. The check just confirms the rule names exist — it doesn't verify they're relaxed (set to `"off"` or `"warn"`) specifically in the test override section.

### F24: T-ESLP-12 tailwind-ban — denyList check uses generic string

**Severity: LOW**

The word "denyList" is checked but could appear in any context. Low risk since `tailwind-ban` is a more specific gate.

---

## Rule Presence Checks (T40-T48, T60-T83)

### F25: Rules provided by presets are NOT explicitly configured — false negatives possible

**Severity: MEDIUM**

`strictTypeChecked` preset provides many rules (e.g., `no-floating-promises`, `await-thenable`, `no-explicit-any`). The actual config ALSO explicitly configures these rules. But if a project relies solely on the preset without explicit rule entries, the presence checks (T40, T60, T61, etc.) would FAIL even though the rules are active via the preset.

This isn't a bug per se — guardrail3 wants explicit configuration — but it's a design decision that could surprise users.

### F26: No check for rule SEVERITY

**Severity: HIGH**

All presence checks confirm the rule name exists. None verify the severity is `"error"`. A rule set to `"warn"` passes all checks. T7 inventories `"warn"` lines but doesn't cross-reference against required rules.

### F27: Prefix matching causes false positives

**Severity: MEDIUM**

`content.contains("no-empty")` matches both `no-empty` and `no-empty-function`, `no-empty-pattern`, etc. Similarly:
- `"no-cycle"` matches `"no-cycle-imports"` (if such a rule existed)
- `"no-shadow"` matches `"no-shadowed-variable"`
- `"strict"` matches `"strictTypeChecked"`, `"strict-boolean-expressions"`, `"use strict"`

In the current rule set, the most problematic is `"no-empty"` (T83) which would match `"@typescript-eslint/no-empty-function"` or `"@typescript-eslint/no-empty-interface"`.

---

## Missing Checks — Rules in Actual Config Not Validated

### F28: `no-restricted-syntax` rules not validated

**Severity: HIGH**

The actual config uses `no-restricted-syntax` for two critical purposes:
1. Banning raw route exports (requiring withBody/withRoute wrappers)
2. Banning `process.env` direct access

T50 and T51 only check for string presence, not that `no-restricted-syntax` is configured with the correct selectors. There is no check that `no-restricted-syntax` itself exists as a configured rule.

### F29: `import-x/max-dependencies` checked as T46 but with wrong rule name

**Severity: MEDIUM**

T46 checks for `"max-dependencies"`. The actual config uses `"import-x/max-dependencies"`. Since `content.contains("max-dependencies")` matches the substring within `"import-x/max-dependencies"`, this works — but it would also match a comment containing `max-dependencies` without the plugin prefix.

### F30: `import-x/no-cycle` checked as T45 but with wrong rule name

**Severity: MEDIUM**

T45 checks for `"no-cycle"`. Actual config uses `"import-x/no-cycle"`. Same substring matching issue as F29.

### F31: No check for `max-depth`, `max-params`, `max-nested-callbacks`

**Severity: MEDIUM**

The actual config doesn't have these rules, but they're in the BUILTIN_RULES list in `eslint_plugin_checks.rs` (lines 80-82). So T-ESLP-10 will report them as missing — which is correct. But the actual config doesn't include them, meaning the real project would fail T-ESLP-10. This suggests either the expected rules list is aspirational or the actual config needs updating.

### F32: No check for landing-specific import restrictions

**Severity: LOW**

The actual config (lines 330-356) has landing-specific `no-restricted-imports` that bans `@project/spec`, `@project/validator-types`, and `**/content/**`. No guardrail3 check verifies this exists.

### F33: No check for `.mjs` file rules

**Severity: LOW**

The actual config (lines 504-529) applies rules to `.mjs` files and has a special higher line limit for `eslint.config.mjs` itself. No guardrail3 check verifies these blocks exist.

### F34: No check for `disableTypeChecked` on .mjs files

**Severity: LOW**

The actual config (lines 148-149) disables type-checked rules for `.mjs` files. This is important because `.mjs` files can't have a tsconfig. No check verifies this.

### F35: No check for `parserOptions.project: true`

**Severity: MEDIUM**

Type-checked rules require `parserOptions: { project: true }`. The actual config (lines 141-143) sets this. If removed, all type-checked rules silently become no-ops. No guardrail3 check verifies this.

### F36: No check for global ignores pattern

**Severity: LOW**

The actual config (lines 121-133) has critical global ignores for generated code, shadcn UI, etc. No check verifies these exist or are correct.

---

## Test Quality Issues

### F37: Tests don't cover comment bypass

**Severity: HIGH**

`eslint_plugin_checks_tests.rs` creates synthetic content by joining rule name strings. No test verifies behavior when rules appear only in comments. No test verifies the override blindness (F01).

### F38: Tests don't cover value checking

**Severity: HIGH**

No test in the test file covers `check_rule_value`, `extract_number_from_line`, or any T2-T4 value validation logic.

### F39: `test_core_plugins_all_pass` creates unrealistic content

**Severity: MEDIUM**

The test joins rule names with newlines — no actual ESLint config syntax. It doesn't test that the checker works on real config syntax like `"@typescript-eslint/no-floating-promises": "error"`.

---

## Summary Table

| ID | Severity | Check(s) Affected | Issue |
|----|----------|-------------------|-------|
| F01 | CRITICAL | ALL | Later config blocks can disable rules undetected |
| F02 | HIGH | ALL presence checks | Comments containing rule names produce false passes |
| F04 | CRITICAL | T2 | Expected value (400) doesn't match actual config (300) |
| F05 | HIGH | T2-T4 | First-number extraction is fragile for object configs |
| F06 | HIGH | T2-T4 | Only checks first occurrence of rule in file |
| F09 | CRITICAL | T36 | Operator precedence + generic word matching |
| F11 | HIGH | T6, T36-T39 | Doesn't verify rules are "error" not "off" |
| F12 | HIGH | T-ESLP-13/14 | Comment bypass for preset checks |
| F14 | HIGH | T50 | Matches message text, not rule structure |
| F17 | HIGH | T51 | Always passes because "process.env" is in message text |
| F19 | HIGH | T-ESLP-02..10 | All plugin group checks use `content.contains` |
| F20 | HIGH | T-ESLP-02 | Disabled rules checked for presence, not "off" state |
| F22 | HIGH | T-ESLP-07 | "strict" matches strictTypeChecked |
| F26 | HIGH | T40-T48, T60-T83 | No severity verification — "warn" passes as "error" |
| F28 | HIGH | T50, T51 | no-restricted-syntax rules not structurally validated |
| F35 | MEDIUM | All type-checked | parserOptions.project not verified |
| F37 | HIGH | Tests | No comment bypass test coverage |
| F38 | HIGH | Tests | No value checking test coverage |

### Root Cause

Every finding traces back to one design decision: **all ESLint config validation uses raw string matching (`content.contains()`) on the entire file content.** This approach cannot distinguish:
- Active config vs. comments
- First occurrence vs. later override
- Rule presence vs. rule severity
- Rule name in config vs. rule name in message text
- Spread presets vs. explicit rules

The CLAUDE.md acknowledges this: "ESLint rules checked by pattern matching. guardrail3 greps `eslint.config.mjs` for rule names. It checks ~35 key rules individually but cannot detect if a rule's configuration (options, severity) was changed — only presence/absence." However, the actual impact is worse than stated — it's not just severity/options that are missed, but comments, overrides, and message-text false positives.

### Suggested Fix Priority

1. **Strip comments before matching** — eliminates F02, F08, F09, F12, F22
2. **Parse the last config block for each rule to detect overrides** — addresses F01
3. **Verify rules are not set to "off"/"warn" where "error" is required** — addresses F11, F20, F26
4. **Fix T2 expected value to match actual config (300 not 400)** — addresses F04
5. **Add `parserOptions.project` check** — addresses F35
6. **Structural validation for no-restricted-syntax** — addresses F14, F17, F28
