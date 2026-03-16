# Migrate Source Checks from Grep to AST Parsing (syn + tree-sitter)

## Problem

guardrail3's source scan checks (R30-R58, T23-T35) are grep-based — they scan lines for text patterns. This causes:

1. **False positives:** String literals containing `#[allow(` are flagged as violations
2. **False positives:** Comments discussing `#[allow(` patterns are flagged
3. **Missed patterns:** Attributes split across lines may be missed
4. **No understanding of scope:** Can't distinguish module-level from function-level attributes
5. **No understanding of context:** Can't tell if an `unsafe` block is inside a test vs production code

## Solution

- **Rust checks (R30-R58):** Use `syn` crate to parse Rust AST. Walk the tree checking attributes, items, expressions structurally.
- **TypeScript checks (T23-T35):** Use `tree-sitter` + `tree-sitter-typescript` for structural parsing.
- **Config checks (R1-R29, T1-T22):** Already structured (TOML/JSON parsing). No change needed.

## Migration strategy

### Phase 1: Capture golden baselines (DONE)

Golden snapshots against 5 real projects:
- `golden-tests/golden/self-validate.json` (511 checks)
- `golden-tests/golden/external-websmasher.json` (900 checks)
- `golden-tests/golden/external-pipelin3r.json` (480 checks)
- `golden-tests/golden/external-schedulr.json` (379 checks)
- `golden-tests/golden/external-steady-parent.json` (611 checks)

Total: 2,881 checks across 5 projects. Any change in output after migration is visible.

### Phase 2: Write grep-should-fail fixtures

These are inputs where the CURRENT grep-based tool gives WRONG results. After migration to AST, these should give correct results. This proves the migration improved correctness.

**False positive fixtures (grep flags, AST should NOT flag):**

1. `string_literal_allow.rs` — `let s = "#[allow(clippy::unwrap_used)]";` — grep sees the pattern in a string literal. syn knows it's an Expr::Lit.

2. `comment_about_allow.rs` — `// Don't add #[allow(clippy::unwrap_used)] without reason` — grep sees the pattern in a comment. syn ignores comments.

3. `doc_comment_allow.rs` — `/// Example: #[allow(dead_code)]` — same issue in doc comments.

4. `raw_string_allow.rs` — `r#"#[allow(unused)]"#` — raw string literal.

5. `string_unsafe.rs` — `let keyword = "unsafe";` — grep sees `unsafe` in a string.

6. `comment_todo.rs` — `// TODO: remove this todo!() once fixed` — grep might flag `todo!()` in a comment.

7. `string_unwrap.rs` — `assert_eq!(s, ".unwrap()");` — grep sees `.unwrap()` in a string.

8. `macro_invocation_allow.rs` — `println!("#[allow(dead_code)]");` — grep sees it in a macro arg.

**For TypeScript:**

9. `string_eslint_disable.ts` — `const msg = "// eslint-disable-next-line";` — string literal.

10. `template_literal_ts_ignore.ts` — `` `// @ts-ignore` `` — template literal.

11. `comment_about_process_env.ts` — `// Don't use process.env directly` — comment.

### Phase 3: Migrate Rust checks to syn

For each check in R30-R58:

1. Write a `syn`-based version that walks the AST
2. Run against golden snapshots — output should be IDENTICAL for non-false-positive cases
3. Run against grep-should-fail fixtures — output should DIFFER (fewer false positives)
4. Replace the grep version
5. Update golden snapshots for any intentional changes (false positive removals)

**Check-by-check migration:**

| Check | Current (grep) | Migration (syn) |
|-------|---------------|----------------|
| R30-R31 | Scan for `#![allow(` | Walk top-level `ItemMod`/`ItemFn` attributes |
| R32-R33 | Scan for `#[allow(` + check for `//` | Walk all `Item` attributes, check `Attribute::Meta` |
| R34-R35 | Scan for `#[garde(skip)]` | Walk derive attributes |
| R37 | Scan for `cfg_attr` + `allow` | Walk `Attribute::Meta::List` nested attrs |
| R38-R39 | Count effective lines | syn span gives line count (or keep line counting, it's fine) |
| R40-R41 | Count `use` statements | Walk `ItemUse` nodes |
| R42 | Scan for `unsafe` | Walk `ExprUnsafe` / `ItemImpl` unsafety |
| R43 | Scan for `todo!()` | Walk `ExprMacro` with path `todo` |
| R44 | Scan for `.unwrap()` / `.expect()` | Walk `ExprMethodCall` with method name |
| R58 | Scan for `use std::fs` | Walk `ItemUse` with path matching |

### Phase 4: Migrate TypeScript checks to tree-sitter

Same approach but using `tree-sitter-typescript`:

| Check | Current (grep) | Migration (tree-sitter) |
|-------|---------------|------------------------|
| T23-T26 | Scan for `eslint-disable` | Find Comment nodes |
| T27 | Scan for `@ts-ignore` | Find Comment nodes |
| T28-T29 | Scan for `@ts-expect-error` | Find Comment nodes |
| T30 | Scan for `process.env` | Find MemberExpression nodes |
| T31 | Scan for `: any` | Find TypeAnnotation nodes |
| T32-T33 | Count lines | Tree root span |

### Phase 5: Verify and release

1. All 2,881 golden checks still pass (with documented exceptions for fixed false positives)
2. All grep-should-fail fixtures now give correct results
3. All 218+ existing tests pass
4. Mutation testing kill rate maintained or improved
5. No new dependencies that break the tool's portability

## Dependencies to add

```toml
# For Rust AST parsing
syn = { version = "2", features = ["full", "parsing"] }

# For TypeScript/JavaScript parsing
tree-sitter = "0.24"
tree-sitter-typescript = "0.23"
tree-sitter-javascript = "0.23"
```

## Timeline estimate

- Phase 2 (fixtures): 1 session
- Phase 3 (Rust migration): 2-3 sessions (12 checks to migrate)
- Phase 4 (TS migration): 1-2 sessions (8 checks to migrate)
- Phase 5 (verify): 1 session

## Risk

The main risk is that grep catches things syn doesn't — for example, grep sees through macro invocations (it doesn't care about macro boundaries), while syn only sees the un-expanded source. If a macro generates `#[allow(...)]`, syn won't see it but grep would. This is actually a CORRECTNESS improvement (we shouldn't flag macro-generated attributes), but it's a behavioral change.

Golden snapshots against real projects will catch any unexpected changes.
