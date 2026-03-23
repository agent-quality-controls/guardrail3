# Adversarial Audit: TypeScript Source Scan (T23-T35, T59)

**File:** `apps/guardrail3/src/app/ts/validate/source_scan.rs`
**Supporting files:** `ts_comment_checks.rs`, `ast_helpers.rs`, `ts_code_analysis.rs`

---

## FINDINGS

### F-01: T32/T33 threshold mismatch with documented spec (REAL BUG)
**Severity: HIGH**

The help text (`help_gen.rs:486`) documents: `T32-T33 File length (>300 error, >200 warn)`.
The code implements: T32 fires at >400, T33 does not exist at all.
The test comment says "T33 was killed" and "Only T32 (>300 effective lines) remains" but the actual threshold in code is 400, not 300.

Three conflicting sources of truth:
- `help_gen.rs`: >300 error, >200 warn
- Test comment: >300 (T32 only, T33 killed)
- Actual code: >400

**Either the code is wrong (should be 300) or the help text and test comment are wrong (should say 400). No matter what, at least two of the three are lying.**

### F-02: T30 process.env -- `process["env"]` bypasses AST detection (REAL BUG)
**Severity: HIGH**

`is_process_dot_env()` in `ts_code_analysis.rs` only checks `member_expression` nodes (dot access). The code at line 45 also checks `subscript_expression` in `collect_process_env`, but `is_process_env_access` delegates to `is_process_dot_env` which requires `member_expression` kind.

For `process["env"]`, tree-sitter produces a `subscript_expression` with `object=process` and `index="env"` (a string). The function `is_process_dot_env` returns false because `node.kind() != "member_expression"`. The `is_process_env_access` function does check `subscript_expression` in its recursion case (line 79) but only for the *object* being a nested access -- it never detects `process["env"]` as the base case itself.

**Bypass:** `const x = process["env"].NODE_ENV;` -- the outer node is a `member_expression` with object being a `subscript_expression`. `is_process_env_access` recurses into the object and calls `is_process_env_access` on the subscript. That call gets to line 79 and checks if the *object's* object is a subscript, but `process` is an `identifier`, not a member/subscript expression, so recursion stops. The function returns false.

Similarly, **destructuring bypasses detection entirely:** `const { env } = process; env.NODE_ENV` -- no `process.env` member expression ever appears in the AST.

### F-03: T31 `any` type -- does not catch all forms (MODERATE)
**Severity: MODERATE**

The AST walker catches:
- `: any` type annotations (via `type_annotation` nodes)
- `as any` expressions (via `as_expression` nodes)
- `type X = any` (via standalone `predefined_type`)
- `Record<string, any>` (via recursive `has_predefined_any_child`)

What it likely misses:
- **Generic type parameters with `any` default:** `function foo<T = any>()` -- the `any` is inside a `type_parameter` node, not a `type_annotation` or `as_expression`. The `predefined_type` fallback should catch this IF tree-sitter emits a standalone `predefined_type` outside those parent contexts.
- **Intersection/union types:** `type X = string | any` -- the `any` is inside a `union_type`, not directly under `type_annotation`. The recursive `has_predefined_any_child` should catch this since it recurses... but only when the parent is `type_annotation` or `as_expression`. If the union is at the type alias level (`type X = string | any`), the `predefined_type` branch (line 112-117) would fire.
- **Index signatures:** `{ [key: string]: any }` -- the `: any` is a `type_annotation` so it should be caught.
- **Mapped types:** `{ [K in keyof T]: any }` -- same, should be caught.

The implementation appears reasonably comprehensive for `any` detection. **The main gap is `Function` type being used as a synonym for `any` in practice, but that's a different check.**

### F-04: T34/T35 -- `check_comment_pattern` uses substring match, not prefix match (DESIGN INTENT, LOW RISK)
**Severity: LOW**

The patterns checked are `"// noinspection"` and `"/* noinspection"`. The check is `comment.text.contains(p)`. Since the scanner uses AST comment extraction, false positives from strings are eliminated. But this means a comment like `// don't use noinspection here` would match. This is a minor false positive concern -- the implementation is aware this is inventory-level (Info severity, `.as_inventory()`), so the impact is minimal.

### F-05: T35 coverage -- missing `v8 ignore` pattern (GAP)
**Severity: MODERATE**

The patterns checked are `"istanbul ignore"` and `"c8 ignore"`. Node.js also supports `/* v8 ignore next */` as a coverage suppression directive (used by Node.js built-in coverage). This is not checked.

### F-06: T23-T26 eslint-disable -- `eslint-enable` not tracked (INTENTIONAL?)
**Severity: LOW**

`eslint-enable` is not checked or inventoried. This means a `/* eslint-disable */` block followed by `/* eslint-enable */` is reported as a block disable (T23/T24), which is correct. But orphaned `eslint-enable` without a matching `eslint-disable` is not flagged. This is a minor completeness gap.

### F-07: T23 reason detection uses `"-- "` (with trailing space) (POTENTIAL BYPASS)
**Severity: MODERATE**

The reason check in `check_block_eslint_disable` is `text.contains("-- ")`. This means:
- `/* eslint-disable no-console --reason */` (no space after `--`) is treated as NO reason (T23 error).
- ESLint itself accepts `--reason` (no space required). So a comment that ESLint considers documented would be flagged as undocumented.
- Conversely, `/* eslint-disable no-console -- */` (double-dash, space, nothing) passes as "has reason" because `"-- "` is contained. The reason content is empty.

The same applies to T25/T26 in `emit_line_suppression_result`.

### F-08: `.mjs` files are scanned for T34/T35 but not T23-T31 (INCONSISTENCY)
**Severity: LOW**

In `check_process_env`, all `.mjs` files are skipped (line 129). But `.mjs` files ARE scanned for T34 (noinspection) and T35 (istanbul/c8 ignore) since those run unconditionally on all ts_files. The `is_ts_file` function includes `.mjs` (line 76). So `.mjs` files get file length checks and suppression checks but NOT eslint-disable, ts-ignore, process.env, or any-type checks.

This is probably intentional for `process.env` (config files) but inconsistent for eslint-disable and ts-ignore -- those could appear in `.mjs` files too.

### F-09: T59 banned packages -- only checks top-level node_modules (DESIGN LIMITATION)
**Severity: MODERATE**

`check_banned_in_node_modules` checks `path.join("node_modules").join(dep)`. With pnpm (which this project uses per npmrc checks), packages are hoisted to a flat `node_modules/.pnpm` structure with symlinks. The check `nm_path.join(dep)` will find the symlink in the top-level `node_modules/`, which works for **direct** dependencies.

For **transitive** dependencies that are NOT hoisted to the top level (pnpm's default behavior with `hoist=false` or certain configurations), a banned package could hide inside `node_modules/.pnpm/` without a top-level symlink. The check would miss it.

Additionally, scoped packages within the banned list are not considered. For example, if `@lodash/lodash` or `lodash-es` were used as alternatives, they wouldn't be caught. The list also doesn't include `moment-timezone` (uses moment internally).

### F-10: `is_ts_test_file` is incomplete (GAP)
**Severity: MODERATE**

The test file detection checks:
- `.test.ts`, `.spec.ts`, `.test.tsx`, `.spec.tsx`
- `__tests__/` in path

Missing patterns:
- `.test.mjs`, `.spec.mjs` -- `.mjs` files are included via `is_ts_file` but `.test.mjs` is not detected as a test file, so they'd get source quality checks applied (though `.mjs` already skips most checks via the `check_process_env` exemption, other checks like T34/T35 would still apply).
- `__mocks__/` directory (Jest mock files)
- `*.stories.ts` / `*.stories.tsx` (Storybook files that often contain `any` and suppressions)
- `*.e2e.ts`, `*.e2e-spec.ts` (end-to-end test patterns)
- Files in `test/` or `tests/` directories (common convention)
- `vitest.config.ts`, `jest.config.ts` (test configuration files)

Since T23-T31 are skipped for test files, missing test file patterns means false positives on legitimate test code.

### F-11: `check_file_length` comment filter is incomplete (MODERATE)
**Severity: MODERATE**

The effective line filter (line 237-240) excludes:
- Empty lines
- Lines starting with `//`
- Lines starting with `*`

What it does NOT exclude:
- `/* ... */` single-line block comments that don't start with `*` -- e.g., `/* comment */` starts with `/`, not `*`, so it counts as an effective line.
- JSDoc opening `/**` lines -- these start with `/`, not `*`, so they count.
- Long import blocks -- a file with 200 lines of imports and 150 lines of actual code would be 350 effective lines and pass. The Rust side has a separate import count check (R40-R41) but there's no equivalent `use count` check for TypeScript.

### F-12: Unicode/encoding bypass potential (LOW RISK)
**Severity: LOW**

All checks operate on UTF-8 `&str`. Tree-sitter parses the raw bytes. Potential attacks:
- **Zero-width characters inside identifiers:** `process\u200B.env` (with zero-width space) -- tree-sitter would treat `process\u200B` as a single identifier, different from `process`. The check would miss it. However, this would also fail at runtime, so it's a theoretical concern.
- **Homoglyph attacks:** Using Cyrillic `а` (U+0430) instead of ASCII `a` in `any` -- `аny` would be a different identifier. Again, this would break TypeScript compilation, so it's theoretical.
- **BOM (Byte Order Mark):** A file starting with BOM could potentially shift line numbers, but tree-sitter handles BOM gracefully.

Not practically exploitable because TypeScript itself would reject these.

### F-13: T30 `eslint-disable-next-line` suppression check is fragile (MODERATE)
**Severity: MODERATE**

In `check_process_env_ast` (line 155-160), the code checks if the previous line contains `eslint-disable-next-line` to downgrade severity to Info. This check:
- Uses raw line text, not AST comments -- it will match `eslint-disable-next-line` inside a string on the previous line.
- Only checks the immediately previous line -- a block comment spanning multiple lines with `eslint-disable-next-line` on a non-adjacent line would be missed.
- Doesn't verify the eslint-disable targets a relevant rule -- `// eslint-disable-next-line no-console` would suppress a `process.env` finding even though the disable is for a different rule.

### F-14: T34 noinspection -- only checks `//` and `/*` prefixes, not JSX comments (LOW)
**Severity: LOW**

The patterns checked are `"// noinspection"` and `"/* noinspection"`. In JSX/TSX, comments are `{/* noinspection ... */}`. However, tree-sitter extracts the comment node text WITHOUT the `{` wrapper, so the comment text would be `/* noinspection ... */`, which DOES match `"/* noinspection"`. This is actually handled correctly.

### F-15: scoped_files path does not skip test fixtures (GAP)
**Severity: LOW**

When `scoped_files` is `Some(files)` (e.g., `--staged` or `--files` mode), the code at line 18 only filters by `is_ts_file`. It does NOT apply the `tests/fixtures/` exclusion that `collect_ts_files` applies (line 110). If a staged file is inside `tests/fixtures/`, it would be scanned and produce violations against adversarial test data.

---

## SUMMARY BY CHECK

| Check | Verdict | Key Issues |
|-------|---------|------------|
| T23-T26 (eslint-disable) | SOLID with caveats | F-07: `"-- "` reason check too strict/loose; F-08: .mjs excluded |
| T27 (@ts-ignore) | SOLID | AST-based, cannot be hidden in strings/templates |
| T28-T29 (@ts-expect-error) | SOLID | AST-based, proper reason detection |
| T30 (process.env) | HAS BYPASS | F-02: `process["env"]` and destructuring bypass; F-13: fragile suppression |
| T31 (any type) | GOOD | F-03: covers main cases, edge cases in generics may slip |
| T32-T33 (file length) | SPEC MISMATCH | F-01: three conflicting thresholds; F-11: comment filter gaps |
| T34 (noinspection) | SOLID | AST-based comment extraction works correctly |
| T35 (coverage ignore) | INCOMPLETE | F-05: missing `v8 ignore` pattern |
| T59 (banned packages) | DESIGN LIMITATION | F-09: only top-level node_modules; no transitive check |
| Test file exclusion | INCOMPLETE | F-10: many test file patterns missing |
| Encoding bypass | NOT EXPLOITABLE | F-12: would break TS compiler first |

## TOP PRIORITY FIXES

1. **F-01** -- Resolve the three-way threshold conflict for T32/T33. Pick one source of truth.
2. **F-02** -- Add `process["env"]` detection to the AST checker. Consider flagging destructured `const { env } = process`.
3. **F-05** -- Add `v8 ignore` to coverage suppression patterns.
4. **F-07** -- Decide on reason syntax: either require `-- ` (space) matching ESLint convention, or accept `--` without trailing space.
5. **F-09** -- For pnpm projects, also scan `node_modules/.pnpm/` for banned packages or use `pnpm list --json` to check the full dependency tree.
6. **F-10** -- Expand test file detection patterns.
7. **F-15** -- Apply fixtures exclusion in scoped_files path.
