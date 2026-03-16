# Adversarial Grep Attack Fixtures

## Categories

- `rust-allow/`: 10 fixtures testing `#[allow(` in non-code contexts
- `rust-code-quality/`: 10 fixtures testing unsafe/todo/unwrap in non-code contexts
- `rust-structural/`: 10 fixtures testing use/fs/line-count boundaries
- `typescript/`: 10 fixtures testing eslint-disable/ts-ignore/process.env in non-code contexts
- `edge-cases/`: 10 fixtures testing parser robustness

## Expected Behavior

- **GREP_BUG**: current grep tool gives wrong result (false positive or false negative)
- **CORRECT**: both grep and AST should give same result
- **BOUNDARY**: exact boundary test (`>` vs `>=`)

---

## rust-allow/

| Fixture | What it tests | Expected |
|---|---|---|
| `string_literal.rs` | `#[allow(clippy::unwrap_used)]` inside a string literal | GREP_BUG — grep flags R32, syn sees it's a string, not an attribute |
| `raw_string.rs` | `#[allow(dead_code)]` inside a raw string `r#"..."#` | GREP_BUG — grep flags R32, syn sees raw string |
| `doc_comment.rs` | `#[allow(unused)]` inside a `///` doc comment | GREP_BUG — grep flags R32, syn sees doc comment |
| `block_comment.rs` | `#[allow(clippy::panic)]` inside `/* ... */` | GREP_BUG — grep flags R32, syn sees block comment |
| `format_macro.rs` | `#[allow(clippy::unwrap_used)]` inside `format!()` argument | GREP_BUG — grep flags R32, syn sees macro argument string |
| `println_macro.rs` | `#[allow(dead_code)]` inside `println!()` argument | GREP_BUG — grep flags R32, syn sees macro argument string |
| `assert_macro.rs` | `#[allow(unused)]` inside `assert_eq!()` argument | GREP_BUG — grep flags R32, syn sees macro argument string |
| `concat_string.rs` | `#[allow(` split across string concatenation | GREP_BUG — grep flags R32, syn sees string fragments |
| `multiline_string.rs` | `#[allow(` pattern spanning multiple lines in a string | GREP_BUG — grep flags R32, syn sees multi-line string |
| `byte_string.rs` | `#[allow(dead_code)]` inside `b"..."` byte string | GREP_BUG — grep flags R32, syn sees byte string literal |

## rust-code-quality/

| Fixture | What it tests | Expected |
|---|---|---|
| `string_unsafe.rs` | `"unsafe"` as a string value | GREP_BUG — grep flags R42, syn sees string literal not `unsafe` block |
| `comment_unsafe.rs` | `// unsafe is forbidden` in a comment | GREP_BUG — grep flags R42, syn sees comment |
| `doc_unsafe.rs` | `/// This function is not unsafe` in doc comment | GREP_BUG — grep flags R42, syn sees doc comment |
| `string_todo.rs` | `"todo"` as a string value | GREP_BUG — grep flags R43, syn sees string literal not `todo!()` macro |
| `comment_todo.rs` | `// TODO: refactor` in a comment | GREP_BUG — grep flags R43, syn sees comment (TODO comments are not `todo!()` macros) |
| `string_unwrap.rs` | `".unwrap()"` as a string value | GREP_BUG — grep flags R44, syn sees string literal |
| `comment_unwrap.rs` | `// Don't use .unwrap()` in a comment | GREP_BUG — grep flags R44, syn sees comment |
| `field_name_unwrap.rs` | `unwrap_result` as a struct field name | GREP_BUG — grep flags R44, syn sees identifier not method call |
| `function_name_todo.rs` | `fn todo_list()` as a function name | GREP_BUG — grep flags R43, syn sees identifier not `todo!()` macro |
| `variable_unsafe.rs` | `let unsafe_count = 0` as a variable name | GREP_BUG — grep flags R42, syn sees identifier not `unsafe` block |

## rust-structural/

| Fixture | What it tests | Expected |
|---|---|---|
| `string_use_std_fs.rs` | `"use std::fs"` as a string value | GREP_BUG — grep flags R58, syn sees string literal not import |
| `comment_use_std_fs.rs` | `// Don't use std::fs` in a comment | GREP_BUG — grep flags R58, syn sees comment |
| `use_in_doc_comment.rs` | `/// Uses std::fs for file operations` in doc comment | GREP_BUG — grep flags R58, syn sees doc comment |
| `reexport_fs.rs` | `pub use crate::fs as filesystem` — re-export not `std::fs` | GREP_BUG — grep flags R58, syn sees `crate::fs` not `std::fs` |
| `cfg_gated_use.rs` | `#[cfg(test)] use std::fs;` — conditional import | CORRECT — real import, but gated to test cfg; may warrant special handling |
| `exactly_500_lines.rs` | File with exactly 500 effective lines | BOUNDARY — R38 fires on `> 500`, NOT `>= 500`; should NOT fire |
| `exactly_501_lines.rs` | File with exactly 501 effective lines | BOUNDARY — R38 fires on `> 500`; SHOULD fire |
| `exactly_20_uses.rs` | File with exactly 20 `use` statements | BOUNDARY — R40 fires on `> 20`, NOT `>= 20`; should NOT fire |
| `exactly_21_uses.rs` | File with exactly 21 `use` statements | BOUNDARY — R40 fires on `> 20`; SHOULD fire |
| `blank_lines_only.rs` | 600 lines but all blank + comments | CORRECT — R38 should NOT fire; effective lines = 0 |

## typescript/

| Fixture | What it tests | Expected |
|---|---|---|
| `string_eslint_disable.ts` | `"eslint-disable-next-line"` as a string value | GREP_BUG — grep flags T23, tree-sitter sees string literal |
| `template_eslint_disable.ts` | `` `eslint-disable` `` in template literal | GREP_BUG — grep flags T23, tree-sitter sees template string |
| `comment_about_eslint.ts` | `// We use eslint-disable sparingly` — comment mentioning eslint | GREP_BUG — grep may flag T23, but this comment is about eslint-disable, not an actual directive |
| `string_ts_ignore.ts` | `"@ts-ignore"` as a string value | GREP_BUG — grep flags T27, tree-sitter sees string literal |
| `string_process_env.ts` | `"process.env.NODE_ENV"` as a string value | GREP_BUG — grep flags T30, tree-sitter sees string literal |
| `comment_process_env.ts` | `// process.env is banned` in a comment | GREP_BUG — grep flags T30, tree-sitter sees comment |
| `type_any_in_string.ts` | `": any"` as a string value | GREP_BUG — grep flags T31, tree-sitter sees string literal |
| `generic_any.ts` | `function foo<T = any>()` — actual `any` type usage | CORRECT — real `any` usage, SHOULD be flagged by both grep and tree-sitter |
| `exactly_300_lines.ts` | TypeScript file with exactly 300 lines | BOUNDARY — T32 fires on `> 300`, NOT `>= 300`; should NOT fire |
| `exactly_301_lines.ts` | TypeScript file with exactly 301 lines | BOUNDARY — T32 fires on `> 300`; SHOULD fire |

## edge-cases/

| Fixture | What it tests | Expected |
|---|---|---|
| `empty_file.rs` | Completely empty file (0 bytes) | CORRECT — parser should handle gracefully, no violations, no crash |
| `only_comments.rs` | File with ONLY comments, zero code lines | CORRECT — 0 effective lines, no violations; grep may false-positive on patterns inside comments |
| `unicode_bom.rs` | UTF-8 BOM (`\xEF\xBB\xBF`) before normal Rust code with `#[allow]` | GREP_BUG — BOM may cause grep line-start anchors to fail; AST parser should handle BOM transparently |
| `crlf_line_endings.rs` | Windows-style `\r\n` line endings with `#[allow]` patterns | GREP_BUG — `\r` before `\n` may break line-based grep patterns; AST parser normalizes line endings |
| `very_long_line.rs` | One 10,000-character line (string literal) | CORRECT — parser should handle long lines without truncation or OOM |
| `nested_cfg_attr.rs` | `#[cfg_attr(feature = "x", cfg_attr(feature = "y", allow(unused)))]` — deeply nested | GREP_BUG — grep sees `allow(unused)` and flags it; AST sees conditional attribute, may need special handling for R37 |
| `multiple_allows_one_line.rs` | `#[allow(unused, dead_code, clippy::unwrap_used)]` — multiple lints in one attribute | GREP_BUG — grep may count as 1 allow; AST should count 3 separate lint suppressions |
| `attribute_on_expression.rs` | `#[allow(unused)] let y = 1;` inside a block expression | CORRECT — real allow attribute with reason comment; narrow scope is valid Rust |
| `syntax_error_midway.rs` | Valid code followed by syntax error followed by more valid code | CORRECT — parser should not crash; may report partial results or skip the file gracefully |
| `no_main_lib.rs` | Library-style file with no `main()`, just `pub fn` + `mod tests` | CORRECT — should validate normally; not all Rust files have main |
