# Before Summary: Grep-Based Scanner Results

**Captured:** 2026-03-15
**Scanner:** guardrail3 grep-based source scan (filter_non_comment_lines + pattern matching)
**Test file:** `tests/adversarial_grep_attacks.rs` (40 tests, all passing)

## Methodology

Each fixture was placed into a minimal Rust project (`Cargo.toml` + `src/lib.rs`) and validated
with `guardrail3 rs validate --format json`. Only source-scan results (R30-R58, excluding R49)
are recorded.

---

## rust-allow/ (10 fixtures)

Tests `#[allow()]` patterns in non-code contexts (strings, comments, macros).

| Fixture | Source Scan Hits | Classification | Notes |
|---|---|---|---|
| `string_literal.rs` | (none) | CORRECT | `filter_non_comment_lines` strips string contents; `#[allow(` pattern not at line start |
| `raw_string.rs` | (none) | CORRECT | Same — raw string contents stripped |
| `doc_comment.rs` | (none) | CORRECT | Doc comments (`///`) filtered as comments |
| `block_comment.rs` | (none) | CORRECT | Block comments (`/* */`) filtered |
| `format_macro.rs` | (none) | CORRECT | `#[allow(` inside `format!()` string arg — string stripped |
| `println_macro.rs` | (none) | CORRECT | `#[allow(` inside `println!()` string arg — string stripped |
| `assert_macro.rs` | (none) | CORRECT | `#[allow(` inside `assert_eq!()` string arg — string stripped |
| `concat_string.rs` | (none) | CORRECT | String concat fragments don't start with `#[allow(` |
| **`multiline_string.rs`** | **R32(error)** | **GREP_BUG** | Multiline string continuation `"\n#[allow(\n..."` — line-by-line scanner sees continuation line starting with pattern |
| `byte_string.rs` | (none) | CORRECT | Byte string `b"..."` contents stripped |

**Summary:** 9/10 correct, 1 false positive (multiline string continuation).

---

## rust-code-quality/ (10 fixtures)

Tests `unsafe`/`todo!()`/`.unwrap()` patterns in non-code contexts.

| Fixture | Source Scan Hits | Classification | Notes |
|---|---|---|---|
| `comment_todo.rs` | (none) | CORRECT | `// TODO:` is a comment, filtered out; also check looks for `todo!(` not `TODO` |
| `comment_unsafe.rs` | (none) | CORRECT | `// unsafe is forbidden` is a comment, filtered out |
| `comment_unwrap.rs` | (none) | CORRECT | `// .unwrap()` is a comment, filtered out |
| `doc_unsafe.rs` | (none) | CORRECT | `/// not unsafe` is a doc comment, filtered out |
| `field_name_unwrap.rs` | (none) | CORRECT | `unwrap_result` field — check looks for `.unwrap()` with dot+parens |
| `function_name_todo.rs` | (none) | CORRECT | `fn todo_list()` — check looks for `todo!(` with exclamation |
| `string_todo.rs` | (none) | CORRECT | `"todo"` string — stripped by `filter_non_comment_lines` |
| `string_unsafe.rs` | (none) | CORRECT | `"unsafe"` string — stripped, and check needs `unsafe {` etc. |
| **`string_unwrap.rs`** | **R44(warn)** | **GREP_BUG** | `let method = ".unwrap()";` — `filter_non_comment_lines` returns the full original line (stripping is only for comment detection), so `.unwrap()` in a string literal still matches |
| `variable_unsafe.rs` | (none) | CORRECT | `let unsafe_count` — check requires `unsafe {`, `unsafe fn`, etc. |

**Summary:** 9/10 correct, 1 false positive (`.unwrap()` inside string literal on non-comment line).

---

## rust-structural/ (10 fixtures)

Tests `use std::fs`, line count boundaries, and use-count boundaries.

| Fixture | Source Scan Hits | Classification | Notes |
|---|---|---|---|
| `blank_lines_only.rs` | (none) | CORRECT | 600 lines all blank/comments, 0 effective lines |
| **`cfg_gated_use.rs`** | **R58(error)** | **GREP_BUG** | `#[cfg(test)] use std::fs;` at module level — R58 only skips `#[cfg(test)] mod tests { }` blocks, not standalone cfg-gated imports |
| `comment_use_std_fs.rs` | (none) | CORRECT | R58 skips lines starting with `//` |
| `exactly_20_uses.rs` | R41(warn) | BOUNDARY | 20 uses: R40 (>20) doesn't fire, R41 (>15) fires correctly |
| `exactly_21_uses.rs` | R40(error) | BOUNDARY | 21 uses: R40 (>20) fires correctly |
| `exactly_500_lines.rs` | R39(warn) | BOUNDARY | 500 effective lines: R38 (>500) doesn't fire, R39 (>400) fires correctly |
| `exactly_501_lines.rs` | R38(error) | BOUNDARY | 501 effective lines: R38 (>500) fires correctly |
| `reexport_fs.rs` | (none) | CORRECT | `pub use crate::fs` — not `use std::fs`, correctly ignored |
| `string_use_std_fs.rs` | (none) | CORRECT | `"use std::fs"` in string — line doesn't start with `use std::fs` |
| `use_in_doc_comment.rs` | (none) | CORRECT | `/// Uses std::fs` — doc comment, starts with `///`, skipped |

**Summary:** 9/10 correct (4 boundary tests all correct), 1 debatable false positive (cfg-gated test import).

---

## edge-cases/ (10 fixtures)

Tests parser robustness: empty files, syntax errors, BOM, CRLF, long lines, etc.

| Fixture | Source Scan Hits | Classification | Notes |
|---|---|---|---|
| `empty_file.rs` | (none) | CORRECT | Empty file, no crash |
| `only_comments.rs` | (none) | CORRECT | All comments (incl. patterns like `#[allow(dead_code)]`), correctly filtered |
| `unicode_bom.rs` | R33(info) | CORRECT | BOM doesn't break `#[allow] // reason:` detection |
| `crlf_line_endings.rs` | R33(info) | CORRECT | CRLF doesn't break `#[allow] // reason:` detection |
| `very_long_line.rs` | (none) | CORRECT | 10k-char line handled without crash |
| `nested_cfg_attr.rs` | R37(info) x2 | CORRECT | `cfg_attr` with nested `allow` detected as R37 inventory |
| `multiple_allows_one_line.rs` | R33(info) x4, R44(warn) | CORRECT | Multiple `#[allow]` with reasons + `.unwrap()` usage all detected |
| `attribute_on_expression.rs` | R33(info) x3 | CORRECT | Expression-level `#[allow] // reason:` correctly detected |
| `syntax_error_midway.rs` | R33(info) x3 | CORRECT | Grep doesn't care about syntax — processes all lines |
| `no_main_lib.rs` | R33(info) x2 | CORRECT | Library file handled normally |

**Summary:** 10/10 correct. Grep handles all edge cases well.

---

## Overall Tally

| Category | Total | Correct | GREP_BUG | BOUNDARY |
|---|---|---|---|---|
| rust-allow | 10 | 9 | 1 | 0 |
| rust-code-quality | 10 | 9 | 1 | 0 |
| rust-structural | 10 | 5 | 1 | 4 |
| edge-cases | 10 | 10 | 0 | 0 |
| **Total** | **40** | **33** | **3** | **4** |

### Grep Bugs Found (false positives)

1. **`rust-allow/multiline_string.rs`** — R32(error): `#[allow(` pattern in multiline string continuation fools line-by-line scanner
2. **`rust-code-quality/string_unwrap.rs`** — R44(warn): `.unwrap()` inside a string literal on a non-comment line; `filter_non_comment_lines` returns the original line (stripping is only for comment boundary detection)
3. **`rust-structural/cfg_gated_use.rs`** — R58(error): `#[cfg(test)] use std::fs;` flagged because R58 only handles `#[cfg(test)] mod tests { }` blocks, not standalone cfg-gated imports

### Key Finding

The grep-based scanner is **better than expected**. The `filter_non_comment_lines` function handles most string and comment cases correctly. The main weaknesses are:

1. **Multiline string continuations** — line-by-line processing can't track string state across lines
2. **Pattern matching on original (non-stripped) lines** — string literal stripping is only used for comment detection, not for pattern matching in R44/R43/R42 checks
3. **Limited `#[cfg(test)]` understanding** — only handles the common `mod tests { }` pattern
