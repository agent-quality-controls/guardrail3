# Adversarial Audit: TypeScript Architecture, Hex Arch, and Boundary Enforcement

**Scope:** `ts_arch_checks.rs`, `ts_code_analysis.rs`, `ts_comment_checks.rs`, `eslint_audit.rs`, `stylelint_check.rs`, `test_checks.rs`, `i18n_check.rs`, plus supporting files `eslint_check.rs`, `eslint_rule_infra.rs`, `source_scan.rs`, `mod.rs`

---

## CRITICAL: Import Boundary Checks Use String Matching, Not AST

**File:** `ts_arch_checks.rs` lines 370-414 (`check_file_imports`)

The import boundary checker (`T-ARCH-02`) uses line-by-line string matching via `extract_import_path`, not tree-sitter AST parsing. This is inconsistent with the project philosophy ("AST-based scanning only") and creates multiple bypass vectors:

### GAP-TS-ARCH-01: Dynamic `import()` expressions are invisible

`extract_import_path` only matches `from '...'`, `from "..."`, `require('...')`, `require("...")`. It completely misses:
```ts
const adapters = await import('../adapters/db');
const { repo } = await import(`@adapters/${name}`);
```

An agent could use dynamic import to bypass every boundary rule. This is the single largest hole in the architecture enforcement.

### GAP-TS-ARCH-02: Template literal imports are invisible

```ts
const path = '../adapters/db';
const mod = require(path);         // not caught (no literal in require)
const mod2 = await import(path);   // not caught
```

Variable indirection makes imports invisible to the string matcher.

### GAP-TS-ARCH-03: Multi-line imports are invisible

```ts
import {
  something
} from
  '../adapters/db';
```

`extract_import_path` scans one line at a time. If `from '...'` is on a different line than `import`, it won't be matched because the function looks for `from '` within a single line. The line containing `'../adapters/db';` does contain `from` followed by a newline -- but the actual `from '../adapters/db'` is split across two lines, so `extract_import_path` on the second line would actually find it. However, the comment filter on line 384 checks `trimmed.starts_with("//")` -- if the line starts with the path, it would pass. **Verdict: partially caught, but fragile.** A well-formatted codebase usually has `from` on the same line, but nothing enforces this.

### GAP-TS-ARCH-04: Re-export chains create laundering paths

```ts
// adapters/db/index.ts
export { UserRepo } from './user-repo';

// application/services/index.ts  (the "laundry" file)
export { UserRepo } from '../../adapters/db';  // <-- THIS is caught

// domain/user.ts
import { UserRepo } from '../application/services';  // NOT caught -- looks like app layer
```

The boundary check only examines the immediate import target. If `application/services/index.ts` re-exports from adapters, domain can transitively access adapter code through application without a boundary violation. The check needs transitive import analysis or barrel-export content scanning.

### GAP-TS-ARCH-05: Comment-skipping logic is incomplete

Line 384:
```rust
if trimmed.starts_with("//") || trimmed.starts_with('*') || trimmed.starts_with("/*") {
    continue;
}
```

This misses:
- Multi-line block comments: a line inside `/* ... */` that doesn't start with `*` is not recognized as a comment
- Lines where a comment follows code: `import { x } from 'y'; // comment` -- this is correctly parsed but the check wouldn't be affected. Fine.
- **The real problem:** This is hand-rolled comment detection, not AST-based. The project uses tree-sitter for comment checks in `ts_comment_checks.rs` but not here.

---

## CRITICAL: Hex Arch Structure Check Is Too Shallow

**File:** `ts_arch_checks.rs` lines 100-155

### GAP-TS-ARCH-06: Only checks `domain/` and `adapters/` existence -- missing `ports/` and `application/`

`check_single_app_structure` only verifies `src/modules/domain/` and `src/modules/adapters/` exist. But the layer enum defines FOUR layers: Domain, Ports, Application, Adapters. The import boundary rules reference all four. If `ports/` or `application/` don't exist, the hex arch is structurally incomplete but no warning is emitted.

An agent could put all code in `domain/` and `adapters/` with no ports layer, making the dependency inversion pattern impossible.

### GAP-TS-ARCH-07: Empty directories pass the check

`dir_exists_via_probe` checks for marker files or `dir.is_dir()`. An empty `src/modules/domain/` directory passes the structure check, but an empty domain layer is architecturally meaningless. There is no check that each zone actually contains TypeScript files.

### GAP-TS-ARCH-08: `dir_exists_via_probe` marker list is incomplete

Only checks: `index.ts`, `index.tsx`, `mod.ts`, `types.ts`. If a domain directory has files but none match these markers AND `dir.is_dir()` returns false (e.g., in a virtual filesystem during testing), the directory is falsely reported as missing. The fallback to `dir.is_dir()` uses raw `std::path` instead of the `FileSystem` trait, breaking the port abstraction.

---

## HIGH: ESLint Audit Checks Are Pure String Contains

**File:** `eslint_audit.rs`

### GAP-TS-ARCH-09: Zone definition check is extremely loose

Line 29-31:
```rust
let has_zones = content.contains("element-types")
    || content.contains("domain")
        && (content.contains("commands") || content.contains("adapters"));
```

This matches ANY occurrence of these strings anywhere in the file, including comments, string literals, or completely unrelated config. A comment `// TODO: add element-types` passes. The word "domain" in a comment about documentation passes if "adapters" also appears anywhere.

**Operator precedence issue:** `||` has lower precedence than `&&`, so this reads as:
```
has_zones = contains("element-types") || (contains("domain") && (contains("commands") || contains("adapters")))
```
This is probably intentional but could accidentally match on `// eslint-plugin-boundaries element-types docs` in a comment.

### GAP-TS-ARCH-10: No verification that zones map to actual directory structure

T36 checks that zone definitions EXIST in ESLint config but doesn't verify they match the actual project structure. An ESLint config could define zones for `domain`, `ports`, `adapters`, `application` but the project could have completely different directory names. The boundary enforcement would be useless.

### GAP-TS-ARCH-11: No verification of rule severity/error level

T37 checks `content.contains("boundaries/element-types")` but doesn't check if the rule is set to `"error"`. A config with `"boundaries/element-types": "off"` passes the check. Same for T38 and T39.

### GAP-TS-ARCH-12: `check_entry_point` (T38) only checks string presence

`boundaries/entry-point` could appear in a comment or be set to `"off"`. No verification that it actually restricts deep imports. This is the barrel export enforcement check, and it's trivially bypassable by having the rule name present but disabled.

---

## HIGH: ESLint Rule Checking Infrastructure Weaknesses

**File:** `eslint_rule_infra.rs`, `eslint_check.rs`

### GAP-TS-ARCH-13: Rule presence check is substring match, not structural

`check_eslint_rule_presence` uses `content.contains(rule_name)`. This means:
- `no-unused-vars` matches a comment `// we don't use no-unused-vars`
- `no-console` matches `no-console-log` (if such a rule existed)
- A rule appearing in a disabled override (`"no-console": "off"`) counts as "configured"

The check cannot distinguish between a rule being configured as `"error"` vs `"off"` vs appearing in a comment.

### GAP-TS-ARCH-14: Rule value check has a first-number-wins bug

`check_rule_value` (line 250) finds the rule name, then extracts the first number within 5 lines. For:
```js
// max-lines was previously 200
"max-lines": ["error", { max: 500 }],
```
It would extract `200` from the comment line (line containing `max-lines`) and report "pass" because `200 <= 400`. The comment filter on line 290 skips lines starting with `//` but the rule-name-containing line itself has the comment embedded.

Actually re-reading: line 256 `if !line.contains(rule_name)` finds the comment line, then checks lines i through i+5. The comment line itself is checked first. `extract_number_from_line` skips lines starting with `//` (line 290), so the comment-only line would be skipped. But a line like `"max-lines": ["error", // threshold from 2024`, where the rule name and a comment are on the same line, could extract a year number like `2024`.

### GAP-TS-ARCH-15: No check that ESLint rules are set to "error" severity

For most rules (T40-T48, T60-T83), `check_eslint_rule_presence` only checks that the rule name appears in the file. A rule set to `"warn"` or `"off"` passes. T7 separately inventories `"off"`/`"warn"` lines, but this is just info-level -- it doesn't cross-reference with the presence checks to flag "rule T40 no-floating-promises is present but set to off."

---

## HIGH: Import Boundary Layer Detection Gaps

**File:** `ts_arch_checks.rs`

### GAP-TS-ARCH-16: Windows path separators break layer detection

`layer_from_path` (line 191) splits on `/`. On Windows (or with paths containing `\`), no layers would be detected, and ALL import boundary checks would silently pass.

### GAP-TS-ARCH-17: `@domain/...` alias prefix matching is too broad

Lines 232-243: `import_path.starts_with("@domain")` matches `@domain-utils/something`, `@domains/entity`, etc. No `/` or end-of-string check after the layer name.

### GAP-TS-ARCH-18: No support for tsconfig path aliases beyond `@/modules/`, `~/modules/`, `@domain/`, etc.

If a project uses custom aliases like `#domain/`, `$lib/domain/`, or `@app/modules/domain/`, these are completely invisible to the boundary checker. The checker has a hardcoded set of alias patterns with no extensibility.

### GAP-TS-ARCH-19: Files outside `modules/` directory are not checked

`collect_module_ts_files` (line 347) only includes files containing `/modules/` in their path. Files in `src/utils/`, `src/config/`, `src/lib/`, or `src/pages/` (Next.js) are never checked for boundary violations. An agent could place code in `src/helpers/` that imports from any layer without detection.

### GAP-TS-ARCH-20: Test files excluded from boundary checking

Line 347: `!is_ts_test_file(&path_str)` excludes test files. While tests legitimately need to import from multiple layers, this means test files could contain production helper code that violates boundaries and never be flagged.

---

## MEDIUM: Test Quality Checks Are Superficial

**File:** `test_checks.rs`

### GAP-TS-ARCH-21: No test-to-source ratio or coverage threshold check

The only structural test check is "at least one test file exists" (T-TEST-02). There is no check for:
- Minimum test file count relative to source file count
- Coverage configuration (e.g., coverageThreshold in vitest/jest config)
- Whether test files actually contain assertions (an empty test file passes)

### GAP-TS-ARCH-22: No check for test file naming convention consistency

`is_test_file` accepts both `.test.ts` and `.spec.ts`. There's no check that a project uses ONE convention consistently. Mixed conventions indicate disorganized test strategy.

### GAP-TS-ARCH-23: No check for co-location or test directory structure

No verification that test files are co-located with source files or in a consistent test directory. Tests could be scattered randomly.

### GAP-TS-ARCH-24: `.skip()` reason check is trivially bypassable

Line 218: `line_text.contains("// reason")` -- any comment containing the word "reason" passes. `// reason: TODO` or `// no good reason` both pass.

### GAP-TS-ARCH-25: No check for assertion count in test files

A test file with `it('should work', () => {})` (no assertions) passes all checks. There's no minimum assertion density requirement.

### GAP-TS-ARCH-26: `test.todo()` not detected

Tree-sitter checks for `.skip()` and `.only()` but not `.todo()`. Committed `test.todo()` calls represent unimplemented test cases that silently reduce coverage.

### GAP-TS-ARCH-27: Stryker config existence != mutation testing actually runs

T-TEST-01 checks for config file existence but doesn't verify Stryker is in devDependencies, has a test command in package.json scripts, or is run in CI.

---

## MEDIUM: i18n Check Gaps

**File:** `i18n_check.rs`

### GAP-TS-ARCH-28: Only top-level keys compared

Line 134: `obj.keys().cloned().collect()` -- only top-level keys are compared. Nested keys are not recursively checked. If `en.json` has `{"auth": {"login": "Login"}}` and `fr.json` has `{"auth": {}}`, the check passes because both have the top-level key `auth`.

### GAP-TS-ARCH-29: No check for hardcoded user-visible strings in source

The i18n check only validates locale file consistency. It doesn't scan source files for hardcoded strings that should be i18n keys (e.g., JSX text content like `<p>Welcome</p>` instead of `<p>{t('welcome')}</p>`).

### GAP-TS-ARCH-30: No check for missing i18n function imports

No verification that components actually import and use the i18n translation function (`useTranslations`, `useIntl`, `t()`, etc.).

### GAP-TS-ARCH-31: Silent skip when no i18n library detected

If an i18n library is not in package.json but hardcoded strings exist, the check silently returns. A content app with no i18n setup at all gets zero warnings about missing internationalization.

---

## MEDIUM: Stylelint Check Gaps

**File:** `stylelint_check.rs`

### GAP-TS-ARCH-32: All checks use `content.contains()` string matching

Every stylelint check uses substring matching on the config file content. A commented-out rule passes. A rule name in a string literal passes. Same fundamental weakness as ESLint checks.

### GAP-TS-ARCH-33: No check for stylelint being in devDependencies

The check looks for config files and their content but never verifies that `stylelint` itself is installed as a dependency.

### GAP-TS-ARCH-34: Architecture exceptions check doesn't verify rules are actually disabled

Line 176: `content.contains(rule)` checks that the rule name appears, but doesn't verify it's set to `null` (disabled). The rule could be set to `"error"` and still pass.

### GAP-TS-ARCH-35: Missing `stylelint.config.cjs` and `stylelint.config.ts` from config file list

`STYLELINT_CONFIG_FILES` doesn't include `.cjs` or `.ts` variants, or `package.json` `stylelint` field. A project using these formats would get a false "config not found" error.

---

## MEDIUM: Comment Check Gaps

**File:** `ts_comment_checks.rs`

### GAP-TS-ARCH-36: `eslint-disable` reason detection is bypassable

Line 49: `text.contains("-- ")` checks for the double-dash separator. But `/* eslint-disable no-console --*/` (no space after dash-dash before closing) could pass or fail depending on exact formatting. More importantly, `-- x` (single character "reason") passes.

### GAP-TS-ARCH-37: Both `@ts-ignore` and `@ts-expect-error` trigger on same comment

Lines 169 and 187: If a comment contains both strings (e.g., `// @ts-ignore -- should use @ts-expect-error`), both T27 and T28/T29 fire, producing duplicate/confusing results.

---

## MEDIUM: Code Analysis (tree-sitter) Gaps

**File:** `ts_code_analysis.rs`

### GAP-TS-ARCH-38: `any` detection misses function parameter default types

`find_any_types` looks for `type_annotation`, `as_expression`, and bare `predefined_type`. But `any` in generic type parameters like `Promise<any>`, `Array<any>`, `Map<string, any>` may or may not be caught depending on tree-sitter's AST structure. The `has_predefined_any_child` recursive check should catch nested `any` in type annotations, but only if the outer node is a `type_annotation` or `as_expression`. For `type X = Promise<any>`, the `predefined_type` case handles it via the bare-node path. **Likely works but not explicitly tested for all patterns.**

### GAP-TS-ARCH-39: `process.env` detection misses destructured access

```ts
const { NODE_ENV } = process.env;
```

This IS a member expression `process.env` in the AST, so it IS caught. Good.

But:
```ts
const env = process.env;  // caught
env.NODE_ENV;              // NOT caught -- "env" is not "process.env"
```

Variable aliasing defeats the check, same as with imports.

### GAP-TS-ARCH-40: Test method detection only covers flat calls

`collect_test_method_calls` only matches `object.method()` where object is one of `test`, `describe`, `it`, `beforeEach`, `afterEach`. It misses:
- `suite()` (Mocha)
- `expect.skip()` pattern
- Nested describes: `describe.concurrent.skip()` (chained methods)
- Custom test wrappers: `myTest.skip()` where `myTest = test.extend({...})`

---

## MEDIUM: Monorepo Handling Gaps

### GAP-TS-ARCH-41: ESLint config checked only at project root

`eslint_check.rs` line 9: `path.join("eslint.config.mjs")` -- only checks the root-level ESLint config. In a monorepo with per-app ESLint configs (`apps/web/eslint.config.mjs`), per-app configs are never validated. An app could have its own ESLint config that disables all boundary rules.

### GAP-TS-ARCH-42: Stylelint, test runner, and Stryker configs checked only at root

Same issue: `test_checks.rs` checks for `vitest.config.ts` at the root path, not per-app. Monorepo apps with their own test configs are not discovered.

### GAP-TS-ARCH-43: App discovery only looks at `apps/` directory

`discover_ts_apps` only looks under `<root>/apps/`. Monorepos using `packages/`, `services/`, `libs/`, or nested workspace globs are invisible.

---

## LOW: Source Scan Exclusion Gaps

### GAP-TS-ARCH-44: `.mjs` files skip process.env check entirely

`source_scan.rs` line 129-131: All `.mjs` files are exempt from process.env checks. A determined agent could rename a TypeScript file to `.mjs` to evade the check. The intent is to exempt config files, but the exemption is overbroad.

### GAP-TS-ARCH-45: `is_excluded_ts_dir` doesn't exclude `build/`, `out/`, `.turbo/`, `.vercel/`

Missing common build/cache directories. Scanning these wastes time and could produce false positives from transpiled code.

### GAP-TS-ARCH-46: Test fixtures excluded by path convention only

Line 110 in `source_scan.rs` and line 179 in `test_checks.rs`: `path_str.contains("tests/fixtures/")`. An agent could place bypass code in `test/fixtures/` (singular `test`) or `__fixtures__/` and it would be scanned. Inconsistent with `is_ts_test_file` which checks `__tests__/` (with underscores).

---

## LOW: Escape Hatch Opportunities

### GAP-TS-ARCH-47: Wrapper files / re-exports can launder imports

As detailed in GAP-TS-ARCH-04, barrel files in intermediate layers can re-export from forbidden layers, creating a laundering path. The boundary checker only sees the immediate import target, not transitive dependencies.

### GAP-TS-ARCH-48: Deep imports into zones are not checked by guardrail3 itself

The ESLint `boundaries/entry-point` rule is supposed to enforce barrel-only imports, but guardrail3 only checks that this rule EXISTS in the config (T38). If the rule is misconfigured, disabled, or has exceptions, guardrail3's own import boundary check (`T-ARCH-02`) doesn't enforce barrel-only imports -- it only checks layer direction.

### GAP-TS-ARCH-49: No enforcement of index.ts barrel pattern

There is no check that each zone (`domain/`, `ports/`, `adapters/`, `application/`) has an `index.ts` barrel file. Without barrels, the `boundaries/entry-point` ESLint rule has nothing to enforce.

---

## DESIGN: Missing Architectural Patterns Not Enforced

### GAP-TS-ARCH-50: No dependency injection verification

Hex arch requires dependency injection at boundaries (adapters implement port interfaces). There is no check that:
- Port interfaces exist (TypeScript interfaces in `ports/`)
- Adapter classes implement port interfaces
- Domain code only references port interfaces, not concrete implementations

### GAP-TS-ARCH-51: No check for shared/common module patterns

Many hex arch implementations have a `shared/` or `common/` module for cross-cutting concerns. The layer system doesn't account for this -- files in `src/modules/shared/` get `layer_from_path` returning `None` and are completely unmonitored.

### GAP-TS-ARCH-52: No enforcement of single-direction data flow

Hex arch should enforce that data transformations happen at boundaries (DTOs in adapters, domain models in domain). There's no check for domain types leaking into adapter interfaces or vice versa.

### GAP-TS-ARCH-53: No check for `any` in exported function signatures specifically

T31 reports all `any` usage at info level. But `any` in an exported function signature (public API boundary) is far more dangerous than `any` in a local variable. There's no severity differentiation.

### GAP-TS-ARCH-54: No verification that ESLint is actually run in CI/pre-commit

Guardrail3 checks that ESLint is configured but doesn't verify it's actually executed. A perfectly configured ESLint that's never run provides zero protection.

---

## Summary by Severity

| Severity | Count | Key Gaps |
|----------|-------|----------|
| CRITICAL | 2 | Import boundary uses string matching not AST (GAP-01-05); hex arch structure check incomplete (GAP-06-08) |
| HIGH | 4 | ESLint audit is pure string contains (GAP-09-12); rule checking can't verify severity (GAP-13-15); layer detection fragile (GAP-16-20) |
| MEDIUM | 7 | Test checks superficial (GAP-21-27); i18n only top-level keys (GAP-28-31); stylelint string matching (GAP-32-35); comment check edge cases (GAP-36-37); code analysis aliasing (GAP-38-40); monorepo (GAP-41-43) |
| LOW | 3 | Source scan exclusions (GAP-44-46); escape hatches (GAP-47-49) |
| DESIGN | 5 | Missing arch patterns (GAP-50-54) |
