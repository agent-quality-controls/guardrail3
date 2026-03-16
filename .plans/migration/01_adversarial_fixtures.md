# Step 01: Write 50+ Adversarial Test Fixtures

## Goal
Create test inputs that expose every weakness of grep-based source scanning. These fixtures serve two purposes:
1. Document the current tool's false positives/negatives
2. After migration, prove the AST-based approach is correct

## Task Breakdown (5 agents, ~10 fixtures each)

### Agent 1: Rust #[allow] false positives (10 fixtures)
Create `tests/fixtures/grep-attacks/rust-allow/`:

1. `string_literal.rs` — `let s = "#[allow(clippy::unwrap_used)]";` (grep flags, syn shouldn't)
2. `raw_string.rs` — `r#"#[allow(dead_code)]"#` (grep flags, syn shouldn't)
3. `doc_comment.rs` — `/// Example: #[allow(unused)]` (grep flags, syn shouldn't)
4. `block_comment.rs` — `/* #[allow(clippy::panic)] */` (grep flags, syn shouldn't)
5. `format_macro.rs` — `format!("#[allow(clippy::unwrap_used)]")` (grep flags, syn shouldn't)
6. `println_macro.rs` — `println!("use #[allow(dead_code)] to suppress")` (grep flags, syn shouldn't)
7. `assert_macro.rs` — `assert_eq!(s, "#[allow(unused)]")` (grep flags, syn shouldn't)
8. `concat_string.rs` — `String::from("#[allow(") + "clippy::todo)]"` (grep flags, syn shouldn't)
9. `multiline_string.rs` — multi-line string literal containing allow pattern across lines
10. `byte_string.rs` — `b"#[allow(dead_code)]"` (grep flags, syn shouldn't)

### Agent 2: Rust unsafe/todo/unwrap false positives (10 fixtures)
Create `tests/fixtures/grep-attacks/rust-code-quality/`:

1. `string_unsafe.rs` — `let keyword = "unsafe";` (grep flags R42, syn shouldn't)
2. `comment_unsafe.rs` — `// unsafe is forbidden` (grep flags, syn shouldn't)
3. `doc_unsafe.rs` — `/// # Safety\n/// This function is not unsafe` (grep flags, syn shouldn't)
4. `string_todo.rs` — `let label = "todo";` (grep may flag R43, syn shouldn't)
5. `comment_todo.rs` — `// TODO: refactor this` (grep may flag R43, syn shouldn't — TODO comments are not todo!() macros)
6. `string_unwrap.rs` — `let method = ".unwrap()";` (grep flags R44, syn shouldn't)
7. `comment_unwrap.rs` — `// Don't use .unwrap() here` (grep flags, syn shouldn't)
8. `field_name_unwrap.rs` — `struct Foo { unwrap_result: bool }` (grep may flag, syn shouldn't)
9. `function_name_todo.rs` — `fn todo_list() {}` (grep may flag, syn shouldn't)
10. `variable_unsafe.rs` — `let unsafe_count = 0;` (grep may flag, syn shouldn't)

### Agent 3: Rust use/std::fs false positives + structural edge cases (10 fixtures)
Create `tests/fixtures/grep-attacks/rust-structural/`:

1. `string_use_std_fs.rs` — `let msg = "use std::fs";` (grep flags R58, syn shouldn't)
2. `comment_use_std_fs.rs` — `// Don't use std::fs directly` (grep flags R58, syn shouldn't)
3. `use_in_doc_comment.rs` — `/// Uses std::fs for file operations` (grep may flag, syn shouldn't)
4. `reexport_fs.rs` — `pub use crate::fs as filesystem;` (grep may flag, syn sees it's a re-export not std::fs)
5. `cfg_gated_use.rs` — `#[cfg(test)] use std::fs;` (grep flags, should this be allowed in tests?)
6. `exactly_500_lines.rs` — file with exactly 500 effective lines (R38 should NOT fire — it's > 500, not >= 500)
7. `exactly_501_lines.rs` — file with exactly 501 effective lines (R38 SHOULD fire)
8. `exactly_20_uses.rs` — file with exactly 20 use statements (R40 should NOT fire — it's > 20, not >= 20)
9. `exactly_21_uses.rs` — file with exactly 21 use statements (R40 SHOULD fire)
10. `blank_lines_only.rs` — 600 lines but all blank + comments (R38 should NOT fire — effective lines = 0)

### Agent 4: TypeScript false positives (10 fixtures)
Create `tests/fixtures/grep-attacks/typescript/`:

1. `string_eslint_disable.ts` — `const s = "eslint-disable-next-line";` (grep flags T23, tree-sitter shouldn't)
2. `template_eslint_disable.ts` — `` `eslint-disable` `` (grep flags, tree-sitter shouldn't)
3. `comment_about_eslint.ts` — `// We use eslint-disable sparingly` (this IS a comment — depends on whether it has the full pattern)
4. `string_ts_ignore.ts` — `const s = "@ts-ignore";` (grep flags T27, tree-sitter shouldn't)
5. `string_process_env.ts` — `const s = "process.env.NODE_ENV";` (grep flags T30, tree-sitter shouldn't)
6. `comment_process_env.ts` — `// process.env is banned` (grep flags T30, tree-sitter shouldn't)
7. `type_any_in_string.ts` — `const s = ": any";` (grep flags T31, tree-sitter shouldn't)
8. `generic_any.ts` — `function foo<T = any>()` — actual any usage, SHOULD be flagged
9. `exactly_300_lines.ts` — TypeScript file with exactly 300 lines (T32 should NOT fire)
10. `exactly_301_lines.ts` — TypeScript file with exactly 301 lines (T32 SHOULD fire)

### Agent 5: Cross-cutting edge cases (10+ fixtures)
Create `tests/fixtures/grep-attacks/edge-cases/`:

1. `empty_file.rs` — completely empty file (0 bytes)
2. `only_comments.rs` — file with only comments, no code
3. `unicode_bom.rs` — file starting with UTF-8 BOM before `#[allow(`
4. `crlf_line_endings.rs` — Windows line endings with `#[allow(` patterns
5. `mixed_line_endings.rs` — mix of LF and CRLF
6. `very_long_line.rs` — 10,000 char line containing `#[allow(` somewhere in the middle
7. `nested_attributes.rs` — `#[cfg_attr(feature = "x", cfg_attr(feature = "y", allow(unused)))]`
8. `multiple_allows_one_line.rs` — `#[allow(unused, dead_code, clippy::unwrap_used)]`
9. `attribute_on_expression.rs` — `let x = #[allow(unused)] { 42 };` (expression-level attribute)
10. `proc_macro_output.rs` — `#[derive(Debug)]` where Debug generates code — grep can't see macro output

## For Each Fixture

Write an integration test that:
1. Runs `guardrail3 rs validate` (or `ts validate`) against the fixture
2. Asserts the EXPECTED result (false positive or correct detection)
3. Documents whether this is:
   - `GREP_BUG`: grep gives wrong result (should change after migration)
   - `CORRECT`: both grep and AST should give same result
   - `BOUNDARY`: exact boundary test

## Verification

```bash
cargo test --test adversarial_grep_attacks
```

ALL tests should PASS against the CURRENT grep-based tool. Tests marked `GREP_BUG` assert the WRONG behavior (the false positive). After migration, these tests will be updated to assert the CORRECT behavior.

## Output

- `tests/fixtures/grep-attacks/` — 50+ fixture files
- `tests/adversarial_grep_attacks.rs` — integration tests for all fixtures
- `tests/fixtures/grep-attacks/MANIFEST.md` — list of all fixtures with expected behavior
