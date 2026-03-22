# Adversarial Audit — TS Plan Files (source, jscpd, stylelint, tools, test, arch)

Date: 2026-03-21

---

## 1. TS-SOURCE (13 rules)

### Already covered (confirmed in code)

- `process["env"]` bracket notation: **Covered.** `ts_code_analysis.rs` handles `subscript_expression` alongside `member_expression` in `collect_process_env` and `is_process_env_access`. Both `process.env.X` and `process["env"]` are caught via tree-sitter AST.
- `v8 ignore`: **Covered.** `source_scan.rs` already checks `["istanbul ignore", "c8 ignore", "v8 ignore"]` in the coverage directive check (TS-SOURCE-12 / T35).

### Missing rules — recommend adding

| Candidate ID | What | Bypass it catches | Priority |
|---|---|---|---|
| TS-SOURCE-14 | `@ts-nocheck` at file level | `// @ts-nocheck` disables ALL type checking for the entire file — strictly worse than `@ts-ignore` which is per-line. Currently not caught by any rule. TS-SOURCE-05 catches `@ts-ignore`, TS-SOURCE-06/07 catch `@ts-expect-error`, but `@ts-nocheck` is the nuclear option and is completely undetected. | **High** |
| TS-SOURCE-15 | `as unknown as T` double-cast | Type-safe escape hatch: `x as unknown as TargetType` defeats the type system more aggressively than `as any` (TS-SOURCE-09). The `as any` check catches single-cast but the double-cast through `unknown` is a common workaround that avoids triggering the `any` rule. Tree-sitter can detect nested `as_expression` nodes where the inner type is `unknown`. | **Medium** |
| TS-SOURCE-16 | Expanded test file pattern exclusions | Source checks (file length, suppressions) walk all `.ts`/`.tsx` files. Test infrastructure files (`.stories.tsx`, `*.e2e.ts`, `__mocks__/*.ts`, `*.test.mjs`) may need different thresholds or explicit exclusion tracking. Currently `is_test_file()` only matches `.test.ts(x)` and `.spec.ts(x)`. Files like `__mocks__/foo.ts` or `setup.ts` in test dirs are scanned as production code. Not a bypass per se, but causes false positives on test infrastructure. | **Low** |
| TS-SOURCE-17 | Template literal type assertion | `${expr} as Type` inside tagged template literals — exotic but possible in some TS patterns. Extremely rare in practice. | **Skip** — too niche, not a real-world bypass vector. |

### Verdict

**TS-SOURCE-14 (`@ts-nocheck`) is a clear gap.** It is the most powerful suppression directive in TypeScript and the plan has no rule for it. TS-SOURCE-15 (double-cast) is worth adding but lower urgency since it requires deliberate intent.

---

## 2. TS-JSCPD (10 rules)

### Missing rules — recommend adding

| Candidate ID | What | Why it matters | Priority |
|---|---|---|---|
| TS-JSCPD-09 | `reporters` field validation | If `reporters` is set to `[]` (empty array) or omits `console`, jscpd runs but produces no output — silently disabling the tool. Should warn when reporters is empty or missing. | **Medium** |
| TS-JSCPD-10 | `minLines` / `minTokens` coexistence | If both `minLines` and `minTokens` are set, behavior is confusing — `minTokens` takes precedence in most jscpd versions but `minLines` is silently ignored. Warn when both are present or when `minLines` is used instead of `minTokens` (which TS-JSCPD-05 already requires). | **Low** |

### Not worth adding

- **Parse error reporting**: If `.jscpd.json` is invalid JSON, TS-JSCPD-01 already catches this (existence + valid JSON).
- **Ignoring test files**: jscpd SHOULD scan test files for duplication — duplicated tests are a real problem. No rule needed to exclude them.

### Verdict

TS-JSCPD-09 (reporters validation) is a real silent-disable vector. TS-JSCPD-10 is a nice-to-have clarity check.

---

## 3. TS-STYL (6 rules)

### Already covered (confirmed in code)

- Config file variants: **Mostly covered.** `stylelint_check.rs` searches: `.stylelintrc.mjs`, `.stylelintrc.json`, `.stylelintrc.yml`, `.stylelintrc.yaml`, `stylelint.config.mjs`, `stylelint.config.js`.

### Missing rules — recommend adding

| Candidate ID | What | Why it matters | Priority |
|---|---|---|---|
| TS-STYL-07 | `.stylelintrc.cjs` / `.stylelintrc.ts` / `stylelint.config.cjs` / `stylelint.config.ts` variants | The current search list is missing `.cjs` and `.ts` config variants. Stylelint supports these (especially `.ts` via `jiti` since v16). A project using `stylelint.config.ts` would get a false "config not found" error from TS-STYL-01. | **High** — false negative on valid configs |
| TS-STYL-08 | CSS custom property naming convention (`custom-property-pattern`) | Enforcing a naming convention on CSS custom properties (e.g., `--component-property`) prevents naming collisions in design systems. Standard stylelint rule, easy to check. | **Low** — opinionated, content-profile only |
| TS-STYL-09 | CSS nesting depth (`max-nesting-depth`) | Deeply nested CSS selectors create specificity wars. Checking that `max-nesting-depth` is configured prevents runaway nesting. | **Low** — opinionated |

### Not worth adding

- **Media query validation**: Overly opinionated and not a guardrail concern — this is style enforcement territory.

### Verdict

**TS-STYL-07 is a bug**, not a feature request. The config file list is incomplete. The other two are nice-to-have for content profiles.

---

## 4. TS-TOOL (6 rules)

### Missing rules — recommend adding

| Candidate ID | What | Why it matters | Priority |
|---|---|---|---|
| TS-TOOL-13 | cspell `language` field validation | TS-TOOL-07 checks cspell config existence but not content. If `language` is missing, cspell defaults to `en` which may not match the project. More critically, if `ignorePaths` is empty, cspell will scan `node_modules`, `.next`, `dist` — causing massive slowdowns and false positives. Check that `ignorePaths` is non-empty. | **Medium** |
| TS-TOOL-14 | Prettier config existence | `prettier` is in devDeps (checked by package_deps.rs) but no rule verifies a config file exists (`.prettierrc`, `prettier.config.mjs`, etc.). Without a config, Prettier uses defaults which may conflict with the project's ESLint rules. The hook checks (H-TOOL-04) verify the pre-commit step runs prettier, but nothing checks the config. | **Medium** |
| TS-TOOL-15 | size-limit budget reasonableness | TS-TOOL-11 checks size-limit config exists (content profile). But it doesn't validate that budgets are set. A `size-limit` config with no entries (`[]`) or entries without `limit` fields silently passes — the tool runs but enforces nothing. | **Low** |

### Not worth adding

- **cspell custom words dictionary validation**: Too deep — checking whether custom words are real words is out of scope for a guardrail tool.

### Verdict

TS-TOOL-13 and TS-TOOL-14 are meaningful gaps. A cspell config without `ignorePaths` is a common misconfiguration that makes the tool unusable (it scans everything). A missing Prettier config causes inconsistent formatting.

---

## 5. TS-TEST (5 rules)

### Missing rules — recommend adding

| Candidate ID | What | Why it matters | Priority |
|---|---|---|---|
| TS-TEST-06 | `.todo()` detection | `it.todo("write this test")` and `test.todo()` are placeholders that always pass. Unlike `.skip()` (TS-TEST-04) which silences an existing test, `.todo()` creates the illusion of test coverage. The tree-sitter AST walker in `ts_code_analysis.rs` already has the `collect_test_method_calls` pattern — adding `"todo"` is trivial. | **High** — easy win, real gap |
| TS-TEST-07 | `@stryker-mutator/core` in devDependencies | TS-TEST-01 checks for a Stryker config file, but doesn't verify the package is installed. A config file without the dependency means mutation testing silently fails to run. Check `@stryker-mutator/core` (or `@stryker-mutator/typescript-checker`) in devDeps. | **Medium** |
| TS-TEST-08 | `describe()` nesting depth | Deeply nested `describe()` blocks (4+ levels) indicate poorly organized tests. Tree-sitter can detect nesting depth. | **Low** — opinionated |
| TS-TEST-09 | Test file naming convention | Currently `is_test_file()` accepts both `.test.ts` and `.spec.ts`. Some projects standardize on one. This is too opinionated for a guardrail. | **Skip** — no universal convention |
| TS-TEST-10 | Snapshot staleness | Stale snapshots (`.snap` files) accumulate when tests are deleted. Detecting orphan snapshots requires cross-referencing snap files with test files. | **Skip** — better handled by `vitest --update` or CI |

### Verdict

**TS-TEST-06 (`.todo()`) is a clear gap** — it's the same class of problem as `.skip()` and `.only()`, already partially implemented (the AST walker exists), and the plan simply omitted it. TS-TEST-07 (Stryker in devDeps) is a natural companion to TS-TEST-01.

---

## 6. TS-ARCH (8 rules)

### Already covered (confirmed in code)

- Import boundary check: `ts_arch_checks.rs` handles static imports via `from '...'`, `from "..."`, `require('...')`, `require("...")`.

### Missing rules — recommend adding

| Candidate ID | What | Why it matters | Priority |
|---|---|---|---|
| TS-ARCH-03 | Dynamic `import()` boundary violations | `extract_import_path` only matches `from '...'` and `require('...')` patterns. Dynamic `import('./adapters/foo')` completely bypasses boundary checking. An agent could use `const mod = await import('../adapters/db')` in the domain layer and TS-ARCH-02 would not flag it. Tree-sitter parses `call_expression` with callee `import` — the same line-scanning approach can add `import('` and `import("` patterns. | **High** — real bypass vector |
| TS-ARCH-04 | Re-export barrel file validation | Barrel files (`index.ts` with `export * from './internal'`) can accidentally re-export internal symbols, breaking encapsulation. Within hex arch modules, a barrel in `adapters/outbound/index.ts` that re-exports from `../../domain/` would violate boundaries but wouldn't be caught by TS-ARCH-02 (which checks imports, not re-exports). Checking `export ... from` lines against the same boundary rules closes this hole. | **Medium** |
| TS-ARCH-05 | Circular dependency detection within modules | Two modules importing each other (`domain/a.ts` imports `domain/b.ts` which imports `domain/a.ts`) creates initialization order bugs and tight coupling. While not a cross-layer violation, circular deps within a layer indicate poor decomposition. Could be done by building a file-level import graph and running cycle detection. | **Low** — complex to implement, better caught by `eslint-plugin-import/no-cycle` |

### Not worth adding

- **Deep circular dependency detection across layers**: TS-ARCH-02 already enforces unidirectional flow (adapters -> application -> ports -> domain). True cross-layer cycles are structurally impossible if TS-ARCH-02 passes.

### Verdict

**TS-ARCH-03 (dynamic `import()`) is a real bypass.** The current implementation only handles static `import`/`require` statements. Any use of `import()` as a function call skips boundary validation entirely. TS-ARCH-04 (re-export checking) is a secondary hole — `export { X } from '../domain/secret'` in an adapter barrel is a boundary violation through re-export, not import.

---

## Summary: Priority additions

| Priority | ID | Plan file | Rule |
|---|---|---|---|
| **High** | TS-SOURCE-14 | source | `@ts-nocheck` — nuclear type suppression, completely undetected |
| **High** | TS-TEST-06 | test | `.todo()` — test placeholder, same class as `.skip()`/`.only()` |
| **High** | TS-ARCH-03 | arch | Dynamic `import()` — bypasses all boundary checking |
| **High** | TS-STYL-07 | stylelint | Missing `.cjs`/`.ts` config variants — false negatives |
| **Medium** | TS-SOURCE-15 | source | `as unknown as T` double-cast — defeats type system |
| **Medium** | TS-JSCPD-09 | jscpd | Empty `reporters` — silently disables output |
| **Medium** | TS-TOOL-13 | tools | cspell `ignorePaths` — missing makes tool unusable |
| **Medium** | TS-TOOL-14 | tools | Prettier config existence — formatting without config |
| **Medium** | TS-TEST-07 | test | Stryker in devDeps — config without package |
| **Medium** | TS-ARCH-04 | arch | Re-export barrel boundary violations |
| **Low** | 6 more | various | See per-section tables above |
