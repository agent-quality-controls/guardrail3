//! "Before" capture: run current grep-based guardrail3 against adversarial fixtures.
//!
//! Each test creates a minimal Rust project with the fixture as src/lib.rs,
//! runs `guardrail3 rs validate --format json`, and records which source-scan
//! check IDs fire. These tests document CURRENT behavior — they pass with the
//! grep-based scanner. After migration to syn/AST, compare results.
//!
//! Classification of each fixture result:
//! - `GREP_BUG`: grep incorrectly flags a non-code pattern (false positive)
//! - CORRECT: grep correctly flags a real violation or correctly doesn't flag
//! - BOUNDARY: exact boundary test result (correct by specification)
use garde as _;

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

use std::path::Path;
use std::process::Command;

const MINIMAL_CARGO_TOML: &str = r#"[package]
name = "adversarial-test"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true

[workspace.lints.rust]
unsafe_code = "forbid"
"#;

/// Collected source-scan results for a single fixture (excluding R49/CLAUDE.md).
#[allow(clippy::type_complexity)] // reason: (id, severity) tuple is clear in context
struct FixtureResult {
    /// List of (check ID, severity) pairs from the source scan section.
    checks: Vec<(String, String)>,
}

/// Copy a fixture file into a temp project and run guardrail3 rs validate on it.
/// Returns only the source-scan results (R30-R58), excluding R49 (CLAUDE.md).
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn validate_grep_attack_fixture(category: &str, fixture_name: &str) -> FixtureResult {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/grep-attacks")
        .join(category)
        .join(fixture_name);

    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("create src/");

    // Write minimal Cargo.toml
    std::fs::write(tmp.path().join("Cargo.toml"), MINIMAL_CARGO_TOML).expect("write Cargo.toml");

    // Copy fixture as src/lib.rs
    let content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    std::fs::write(src_dir.join("lib.rs"), &content).expect("write lib.rs");

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args([
            "rs",
            "validate",
            "--format",
            "json",
            tmp.path().to_str().expect("path"),
        ])
        .output()
        .expect("failed to run guardrail3");

    let json_str = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).expect("guardrail3 output should be valid JSON");

    let mut checks = Vec::new();

    #[allow(clippy::indexing_slicing)]
    // reason: JSON structure is known from guardrail3 output format
    if let Some(sections) = parsed["sections"].as_array() {
        for section in sections {
            if section["name"].as_str() == Some("Source code scan") {
                if let Some(results) = section["results"].as_array() {
                    for result in results {
                        let id = result["id"].as_str().unwrap_or("").to_owned();
                        let severity = result["severity"].as_str().unwrap_or("").to_owned();
                        // Skip R49 (CLAUDE.md) — not relevant to source scan testing
                        if id != "R49" {
                            checks.push((id, severity));
                        }
                    }
                }
            }
        }
    }

    FixtureResult { checks }
}

/// Assert that a fixture produces NO source-scan hits (excluding R49).
fn assert_no_hits(result: &FixtureResult, fixture: &str) {
    assert!(
        result.checks.is_empty(),
        "{fixture}: expected no source scan hits, got: {:?}",
        result.checks
    );
}

/// Assert that a fixture produces at least one hit with the given check ID and severity.
fn assert_has_check(result: &FixtureResult, fixture: &str, check_id: &str, severity: &str) {
    let found = result
        .checks
        .iter()
        .any(|(id, sev)| id == check_id && sev == severity);
    assert!(
        found,
        "{fixture}: expected {check_id}({severity}), got: {:?}",
        result.checks
    );
}

/// Assert that a fixture does NOT produce a hit with the given check ID.
fn assert_no_check(result: &FixtureResult, fixture: &str, check_id: &str) {
    let found = result.checks.iter().any(|(id, _)| id == check_id);
    assert!(
        !found,
        "{fixture}: expected no {check_id}, got: {:?}",
        result.checks
    );
}

// ============================================================
// rust-allow/ — #[allow()] patterns in non-code contexts
// ============================================================

// CORRECT: grep does NOT flag these (filter_non_comment_lines strips strings/comments)

#[test]
fn grep_before_rust_allow_string_literal() {
    // CORRECT: grep correctly ignores #[allow()] inside a string literal
    let r = validate_grep_attack_fixture("rust-allow", "string_literal.rs");
    assert_no_hits(&r, "string_literal.rs");
}

#[test]
fn grep_before_rust_allow_raw_string() {
    // CORRECT: grep correctly ignores #[allow()] inside a raw string
    let r = validate_grep_attack_fixture("rust-allow", "raw_string.rs");
    assert_no_hits(&r, "raw_string.rs");
}

#[test]
fn grep_before_rust_allow_doc_comment() {
    // CORRECT: grep correctly ignores #[allow()] inside a doc comment
    let r = validate_grep_attack_fixture("rust-allow", "doc_comment.rs");
    assert_no_hits(&r, "doc_comment.rs");
}

#[test]
fn grep_before_rust_allow_block_comment() {
    // CORRECT: grep correctly ignores #[allow()] inside a block comment
    let r = validate_grep_attack_fixture("rust-allow", "block_comment.rs");
    assert_no_hits(&r, "block_comment.rs");
}

#[test]
fn grep_before_rust_allow_format_macro() {
    // CORRECT: grep correctly ignores #[allow()] inside format!() string arg
    let r = validate_grep_attack_fixture("rust-allow", "format_macro.rs");
    assert_no_hits(&r, "format_macro.rs");
}

#[test]
fn grep_before_rust_allow_println_macro() {
    // CORRECT: grep correctly ignores #[allow()] inside println!() string arg
    let r = validate_grep_attack_fixture("rust-allow", "println_macro.rs");
    assert_no_hits(&r, "println_macro.rs");
}

#[test]
fn grep_before_rust_allow_assert_macro() {
    // CORRECT: grep correctly ignores #[allow()] inside assert_eq!() string arg
    let r = validate_grep_attack_fixture("rust-allow", "assert_macro.rs");
    assert_no_hits(&r, "assert_macro.rs");
}

#[test]
fn grep_before_rust_allow_concat_string() {
    // CORRECT: grep correctly ignores #[allow()] across string concatenation
    let r = validate_grep_attack_fixture("rust-allow", "concat_string.rs");
    assert_no_hits(&r, "concat_string.rs");
}

#[test]
fn grep_before_rust_allow_multiline_string() {
    // FIXED: multi-line string tracking in the grep supplement now correctly
    // detects that `#[allow(` on a continuation line is inside a string literal.
    // Previously the line-by-line scanner couldn't track quote parity across lines.
    let r = validate_grep_attack_fixture("rust-allow", "multiline_string.rs");
    assert_no_hits(&r, "multiline_string.rs");
}

#[test]
fn grep_before_rust_allow_byte_string() {
    // CORRECT: grep correctly ignores #[allow()] inside a byte string b"..."
    let r = validate_grep_attack_fixture("rust-allow", "byte_string.rs");
    assert_no_hits(&r, "byte_string.rs");
}

// ============================================================
// rust-code-quality/ — unsafe/todo/unwrap in non-code contexts
// ============================================================

#[test]
fn grep_before_code_quality_comment_todo() {
    // CORRECT: grep correctly ignores "TODO:" in a line comment
    let r = validate_grep_attack_fixture("rust-code-quality", "comment_todo.rs");
    assert_no_hits(&r, "comment_todo.rs");
}

#[test]
fn grep_before_code_quality_comment_unsafe() {
    // CORRECT: grep correctly ignores "unsafe" in a line comment
    let r = validate_grep_attack_fixture("rust-code-quality", "comment_unsafe.rs");
    assert_no_hits(&r, "comment_unsafe.rs");
}

#[test]
fn grep_before_code_quality_comment_unwrap() {
    // CORRECT: grep correctly ignores ".unwrap()" in a line comment
    let r = validate_grep_attack_fixture("rust-code-quality", "comment_unwrap.rs");
    assert_no_hits(&r, "comment_unwrap.rs");
}

#[test]
fn grep_before_code_quality_doc_unsafe() {
    // CORRECT: grep correctly ignores "unsafe" in a doc comment
    let r = validate_grep_attack_fixture("rust-code-quality", "doc_unsafe.rs");
    assert_no_hits(&r, "doc_unsafe.rs");
}

#[test]
fn grep_before_code_quality_field_name_unwrap() {
    // CORRECT: grep does NOT false-positive on field name `unwrap_result`
    // because the check looks for `.unwrap()` (with dot and parens)
    let r = validate_grep_attack_fixture("rust-code-quality", "field_name_unwrap.rs");
    assert_no_hits(&r, "field_name_unwrap.rs");
}

#[test]
fn grep_before_code_quality_function_name_todo() {
    // CORRECT: grep does NOT false-positive on function name `todo_list()`
    // because the check looks for `todo!(` (with exclamation and paren)
    let r = validate_grep_attack_fixture("rust-code-quality", "function_name_todo.rs");
    assert_no_hits(&r, "function_name_todo.rs");
}

#[test]
fn grep_before_code_quality_string_todo() {
    // CORRECT: grep correctly ignores "todo" in a string literal
    let r = validate_grep_attack_fixture("rust-code-quality", "string_todo.rs");
    assert_no_hits(&r, "string_todo.rs");
}

#[test]
fn grep_before_code_quality_string_unsafe() {
    // CORRECT: grep correctly ignores "unsafe" in a string literal
    let r = validate_grep_attack_fixture("rust-code-quality", "string_unsafe.rs");
    assert_no_hits(&r, "string_unsafe.rs");
}

#[test]
fn grep_before_code_quality_string_unwrap() {
    // BEFORE MIGRATION: grep scanner produced R44 false positive on .unwrap() inside string literal
    // FIXED: syn AST correctly ignores `.unwrap()` in a string literal.
    // Previously grep false-positived because filter_non_comment_lines returns the
    // original trimmed line (not the stripped version), so `.unwrap()` inside a
    // string on a non-comment line was flagged. With AST-based detection,
    // only actual method calls are detected.
    let r = validate_grep_attack_fixture("rust-code-quality", "string_unwrap.rs");
    assert_no_hits(&r, "string_unwrap.rs");
}

#[test]
fn grep_before_code_quality_variable_unsafe() {
    // CORRECT: grep does NOT false-positive on variable name `unsafe_count`
    // because the check looks for specific patterns like "unsafe {", "unsafe fn", etc.
    let r = validate_grep_attack_fixture("rust-code-quality", "variable_unsafe.rs");
    assert_no_hits(&r, "variable_unsafe.rs");
}

// ============================================================
// rust-structural/ — use/fs/line-count boundaries
// ============================================================

#[test]
fn grep_before_structural_blank_lines_only() {
    // CORRECT: 600 lines but all blank/comments, effective lines = 0, R38 does NOT fire
    let r = validate_grep_attack_fixture("rust-structural", "blank_lines_only.rs");
    assert_no_check(&r, "blank_lines_only.rs", "R38");
}

#[test]
fn grep_before_structural_cfg_gated_use() {
    // FIXED: `find_std_fs_imports` in AST path now checks for `#[cfg(test)]` attribute
    // on use items and skips them. A standalone `#[cfg(test)] use std::fs;` at module
    // level is no longer flagged by R58.
    let r = validate_grep_attack_fixture("rust-structural", "cfg_gated_use.rs");
    assert_no_check(&r, "cfg_gated_use.rs", "R58");
}

#[test]
fn grep_before_structural_comment_use_std_fs() {
    // CORRECT: grep correctly ignores `use std::fs` in a line comment
    // (R58 checks `trimmed.starts_with("//")` and skips)
    let r = validate_grep_attack_fixture("rust-structural", "comment_use_std_fs.rs");
    assert_no_check(&r, "comment_use_std_fs.rs", "R58");
}

#[test]
fn grep_before_structural_exactly_20_uses() {
    // BOUNDARY: exactly 20 uses. R40 fires on > 20, so should NOT fire.
    // R41 (warn) fires on > 15, so it SHOULD fire.
    let r = validate_grep_attack_fixture("rust-structural", "exactly_20_uses.rs");
    assert_no_check(&r, "exactly_20_uses.rs", "R40");
    assert_has_check(&r, "exactly_20_uses.rs", "R41", "warn");
}

#[test]
fn grep_before_structural_exactly_21_uses() {
    // BOUNDARY: exactly 21 uses. R40 fires on > 20, SHOULD fire.
    let r = validate_grep_attack_fixture("rust-structural", "exactly_21_uses.rs");
    assert_has_check(&r, "exactly_21_uses.rs", "R40", "error");
}

#[test]
fn grep_before_structural_exactly_500_lines() {
    // BOUNDARY: exactly 500 effective lines. R38 fires on > 500, so should NOT fire.
    // R39 was removed — no warning for 400-500 range.
    let r = validate_grep_attack_fixture("rust-structural", "exactly_500_lines.rs");
    assert_no_check(&r, "exactly_500_lines.rs", "R38");
}

#[test]
fn grep_before_structural_exactly_501_lines() {
    // BOUNDARY: exactly 501 effective lines. R38 fires on > 500, SHOULD fire.
    let r = validate_grep_attack_fixture("rust-structural", "exactly_501_lines.rs");
    assert_has_check(&r, "exactly_501_lines.rs", "R38", "error");
}

#[test]
fn grep_before_structural_reexport_fs() {
    // CORRECT: `pub use crate::fs as filesystem` is NOT `use std::fs`, so R58 does NOT fire
    let r = validate_grep_attack_fixture("rust-structural", "reexport_fs.rs");
    assert_no_check(&r, "reexport_fs.rs", "R58");
}

#[test]
fn grep_before_structural_string_use_std_fs() {
    // CORRECT: `"use std::fs"` is inside a string. R58 checks `trimmed.starts_with("use std::fs")`
    // and the line `let msg = "use std::fs";` does NOT start with `use std::fs`, so R58 is fine.
    let r = validate_grep_attack_fixture("rust-structural", "string_use_std_fs.rs");
    assert_no_check(&r, "string_use_std_fs.rs", "R58");
}

#[test]
fn grep_before_structural_use_in_doc_comment() {
    // CORRECT: `/// Uses std::fs for file operations` is a doc comment.
    // R58 checks `trimmed.starts_with("//")` and skips.
    let r = validate_grep_attack_fixture("rust-structural", "use_in_doc_comment.rs");
    assert_no_check(&r, "use_in_doc_comment.rs", "R58");
}

// ============================================================
// edge-cases/ — parser robustness
// ============================================================

#[test]
fn grep_before_edge_empty_file() {
    // CORRECT: empty file produces no source scan hits
    let r = validate_grep_attack_fixture("edge-cases", "empty_file.rs");
    assert_no_hits(&r, "empty_file.rs");
}

#[test]
fn grep_before_edge_only_comments() {
    // CORRECT: file with only comments produces no source scan hits
    // (even though comments contain patterns like `#[allow(dead_code)]`, `unsafe`, `todo!()`)
    let r = validate_grep_attack_fixture("edge-cases", "only_comments.rs");
    assert_no_hits(&r, "only_comments.rs");
}

#[test]
fn grep_before_edge_unicode_bom() {
    // CORRECT: BOM does not prevent #[allow] with reason from being detected as R33
    let r = validate_grep_attack_fixture("edge-cases", "unicode_bom.rs");
    assert_has_check(&r, "unicode_bom.rs", "R33", "info");
    assert_no_check(&r, "unicode_bom.rs", "R32");
}

#[test]
fn grep_before_edge_crlf_line_endings() {
    // CORRECT: CRLF line endings do not prevent #[allow] with reason from being detected
    let r = validate_grep_attack_fixture("edge-cases", "crlf_line_endings.rs");
    assert_has_check(&r, "crlf_line_endings.rs", "R33", "info");
    assert_no_check(&r, "crlf_line_endings.rs", "R32");
}

#[test]
fn grep_before_edge_very_long_line() {
    // CORRECT: very long line (10k chars) does not crash the scanner
    let r = validate_grep_attack_fixture("edge-cases", "very_long_line.rs");
    assert_no_check(&r, "very_long_line.rs", "R32");
}

#[test]
fn grep_before_edge_nested_cfg_attr() {
    // CORRECT: nested cfg_attr with allow is detected as R37 (cfg_attr inventory)
    let r = validate_grep_attack_fixture("edge-cases", "nested_cfg_attr.rs");
    assert_has_check(&r, "nested_cfg_attr.rs", "R37", "info");
}

#[test]
fn grep_before_edge_multiple_allows_one_line() {
    // CORRECT: multiple #[allow()] attributes on separate lines are each detected as R33
    // Also detects R44 for .unwrap() usage inside the function body
    let r = validate_grep_attack_fixture("edge-cases", "multiple_allows_one_line.rs");
    assert_has_check(&r, "multiple_allows_one_line.rs", "R33", "info");
    assert_has_check(&r, "multiple_allows_one_line.rs", "R44", "warn");
}

#[test]
fn grep_before_edge_attribute_on_expression() {
    // Expression-level #[allow] on let bindings and match arms is now detected.
    // All three allows in the fixture have // reason: comments, so they appear as R33 info.
    let r = validate_grep_attack_fixture("edge-cases", "attribute_on_expression.rs");
    assert_has_check(&r, "attribute_on_expression.rs", "R33", "info");
    assert_no_check(&r, "attribute_on_expression.rs", "R32");
}

#[test]
fn grep_before_edge_syntax_error_midway() {
    // AST-ONLY: syn cannot parse files with syntax errors. No results emitted.
    // This is a known trade-off of AST-only analysis — unparseable files get zero checks.
    let r = validate_grep_attack_fixture("edge-cases", "syntax_error_midway.rs");
    assert_no_hits(&r, "syntax_error_midway.rs");
}

#[test]
fn grep_before_edge_no_main_lib() {
    // CORRECT: library-style file with #[allow] + reason detected as R33
    let r = validate_grep_attack_fixture("edge-cases", "no_main_lib.rs");
    assert_has_check(&r, "no_main_lib.rs", "R33", "info");
    assert_no_check(&r, "no_main_lib.rs", "R32");
}

// ============================================================
// typescript/ — eslint-disable/ts-ignore/process.env/any in non-code contexts
// ============================================================

/// Copy a TS fixture file into a temp directory and run `guardrail3 ts validate` on it.
/// Returns only the TS source-scan results (T23-T35, T59).
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn validate_ts_grep_attack_fixture(fixture_name: &str) -> FixtureResult {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/grep-attacks/typescript")
        .join(fixture_name);

    let tmp = tempfile::tempdir().expect("failed to create temp dir");

    // Copy fixture into the temp directory, preserving the .ts filename
    let content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    std::fs::write(tmp.path().join(fixture_name), &content).expect("write fixture");

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args([
            "ts",
            "validate",
            "--format",
            "json",
            tmp.path().to_str().expect("path"),
        ])
        .output()
        .expect("failed to run guardrail3");

    let json_str = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).expect("guardrail3 output should be valid JSON");

    let mut checks = Vec::new();

    #[allow(clippy::indexing_slicing)]
    // reason: JSON structure is known from guardrail3 output format
    if let Some(sections) = parsed["sections"].as_array() {
        for section in sections {
            if section["name"].as_str() == Some("TS source code scan") {
                if let Some(results) = section["results"].as_array() {
                    for result in results {
                        let id = result["id"].as_str().unwrap_or("").to_owned();
                        let severity = result["severity"].as_str().unwrap_or("").to_owned();
                        checks.push((id, severity));
                    }
                }
            }
        }
    }

    FixtureResult { checks }
}

#[test]
fn grep_before_typescript_string_eslint_disable() {
    // The string literal `"eslint-disable-next-line"` is correctly ignored by tree-sitter.
    // However, the fixture's own comment line 1 says "eslint-disable pattern" which
    // IS a real comment containing "eslint-disable" — tree-sitter correctly flags it as T23.
    // The key test: the string on line 3 does NOT produce an additional T25 hit.
    let r = validate_ts_grep_attack_fixture("string_eslint_disable.ts");
    assert_has_check(&r, "string_eslint_disable.ts", "T23", "error");
    // Should be exactly 1 hit (from the fixture comment), not 2 (which would mean the string was also flagged)
    let t23_count = r.checks.iter().filter(|(id, _)| id == "T23").count();
    assert!(
        t23_count == 1,
        "string_eslint_disable.ts: expected exactly 1 T23 hit (from fixture comment), got {t23_count}"
    );
    assert_no_check(&r, "string_eslint_disable.ts", "T25");
}

#[test]
fn grep_before_typescript_template_eslint_disable() {
    // The template literal `\`eslint-disable\`` is correctly ignored by tree-sitter.
    // However, the fixture's own comment line 1 says "eslint-disable pattern" which
    // IS a real comment containing "eslint-disable" — tree-sitter correctly flags it as T23.
    let r = validate_ts_grep_attack_fixture("template_eslint_disable.ts");
    assert_has_check(&r, "template_eslint_disable.ts", "T23", "error");
    let t23_count = r.checks.iter().filter(|(id, _)| id == "T23").count();
    assert!(
        t23_count == 1,
        "template_eslint_disable.ts: expected exactly 1 T23 hit (from fixture comment), got {t23_count}"
    );
}

#[test]
fn grep_before_typescript_comment_about_eslint() {
    // This is a comment ABOUT eslint-disable, not an actual directive.
    // The tree-sitter path looks at comment nodes and finds "eslint-disable" in the text.
    // The first comment line "// We use eslint-disable sparingly..." contains "eslint-disable"
    // but NOT "eslint-disable-next-line" or "eslint-disable-line", so it matches the
    // block-level eslint-disable pattern (T23/T24). Since it lacks "-- ", it's T23.
    let r = validate_ts_grep_attack_fixture("comment_about_eslint.ts");
    assert_has_check(&r, "comment_about_eslint.ts", "T23", "error");
}

#[test]
fn grep_before_typescript_string_ts_ignore() {
    // The string literal `"@ts-ignore"` is correctly ignored by tree-sitter.
    // However, the fixture's own comment line 1 says "@ts-ignore pattern" which
    // IS a real comment containing "@ts-ignore" — tree-sitter correctly flags it as T27.
    // The key test: the string on line 3 does NOT produce an additional T27 hit.
    let r = validate_ts_grep_attack_fixture("string_ts_ignore.ts");
    assert_has_check(&r, "string_ts_ignore.ts", "T27", "error");
    let t27_count = r.checks.iter().filter(|(id, _)| id == "T27").count();
    assert!(
        t27_count == 1,
        "string_ts_ignore.ts: expected exactly 1 T27 hit (from fixture comment), got {t27_count}"
    );
}

#[test]
fn grep_before_typescript_string_process_env() {
    // FIXED: tree-sitter AST correctly ignores `"process.env.NODE_ENV"` inside a string literal.
    // BEFORE MIGRATION: grep scanner produced T30 false positive on process.env inside string literal
    let r = validate_ts_grep_attack_fixture("string_process_env.ts");
    assert_no_check(&r, "string_process_env.ts", "T30");
}

#[test]
fn grep_before_typescript_comment_process_env() {
    // CORRECT: both grep and tree-sitter correctly ignore `process.env` in a comment.
    // The grep fallback skips comment lines (starts with "//"), and tree-sitter
    // only looks at AST member-expression nodes, not comments.
    let r = validate_ts_grep_attack_fixture("comment_process_env.ts");
    assert_no_check(&r, "comment_process_env.ts", "T30");
}

#[test]
fn grep_before_typescript_type_any_in_string() {
    // FIXED: tree-sitter AST correctly ignores `": any"` inside a string literal.
    // BEFORE MIGRATION: grep scanner produced T31 false positive on ": any" inside string literal
    let r = validate_ts_grep_attack_fixture("type_any_in_string.ts");
    assert_no_check(&r, "type_any_in_string.ts", "T31");
}

#[test]
fn grep_before_typescript_generic_any() {
    // `function foo<T = any>()` is a real `any` type usage.
    // However, tree-sitter's `collect_any_types` only detects `any` in `type_annotation`
    // (`: any`) and `as_expression` (`as any`). A default type parameter (`<T = any>`)
    // uses a `default_type` node inside `type_parameter`, which is not covered.
    // The grep fallback also doesn't match because it checks for `as any` / `: any`,
    // and `= any` matches neither pattern.
    // KNOWN GAP: `<T = any>` is not detected by either path.
    let r = validate_ts_grep_attack_fixture("generic_any.ts");
    assert_no_check(&r, "generic_any.ts", "T31");
}

#[test]
fn grep_before_typescript_exactly_300_lines() {
    // BOUNDARY: exactly 300 total lines, 298 effective lines (2 comment lines excluded).
    // T32 fires on > 300 effective lines, so should NOT fire.
    // T33 was killed — no "approaching limit" warning anymore.
    let r = validate_ts_grep_attack_fixture("exactly_300_lines.ts");
    assert_no_check(&r, "exactly_300_lines.ts", "T32");
    assert_no_check(&r, "exactly_300_lines.ts", "T33");
}

#[test]
fn grep_before_typescript_exactly_301_lines() {
    // BOUNDARY: exactly 301 total lines, 299 effective lines (2 comment lines excluded).
    // T32 fires on > 300 effective lines — 299 is NOT > 300, so T32 should NOT fire.
    // T33 was killed — no "approaching limit" warning anymore.
    let r = validate_ts_grep_attack_fixture("exactly_301_lines.ts");
    assert_no_check(&r, "exactly_301_lines.ts", "T32");
    assert_no_check(&r, "exactly_301_lines.ts", "T33");
}
