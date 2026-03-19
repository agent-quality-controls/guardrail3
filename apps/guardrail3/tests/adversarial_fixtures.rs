//! Adversarial fixture tests for guardrail3 source scan checks R30-R58.
//!
//! Each test creates a minimal Rust project in a temp directory with a single
//! fixture file, runs `guardrail3 rs validate --format json`, and asserts the
//! expected check ID appears (or doesn't appear for clean fixtures).
use garde as _;

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use std::path::Path;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

const MINIMAL_CARGO_TOML: &str = r#"[package]
name = "adversarial-test"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true

[workspace.lints.rust]
unsafe_code = "forbid"
"#;

/// Copy a fixture file into a temp project and run guardrail3 rs validate on it.
/// Returns the JSON stdout as a string.
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn validate_fixture(fixture_name: &str) -> String {
    validate_fixture_with_args(fixture_name, &[])
}

/// Like `validate_fixture` but with extra CLI args (e.g., `--garde` to enable garde checks).
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn validate_fixture_with_args(fixture_name: &str, extra_args: &[&str]) -> String {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/adversarial")
        .join(fixture_name);

    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("create src/");

    // Write minimal Cargo.toml
    std::fs::write(tmp.path().join("Cargo.toml"), MINIMAL_CARGO_TOML).expect("write Cargo.toml");

    // Copy fixture as src/lib.rs (or src/main.rs for files that need it)
    let content = std::fs::read_to_string(&fixture_path).expect("read fixture");
    std::fs::write(src_dir.join("lib.rs"), &content).expect("write lib.rs");

    let mut args = vec!["rs", "validate", "--format", "json"];
    args.extend_from_slice(extra_args);
    args.push(tmp.path().to_str().expect("path"));

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args(&args)
        .output()
        .expect("failed to run guardrail3");

    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Assert that the JSON output contains at least one result with the given check ID and severity.
#[allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing
)] // reason: test helper — JSON parsing + indexing for assertion
fn assert_contains_check(json_output: &str, check_id: &str, severity: &str) {
    let parsed: serde_json::Value =
        serde_json::from_str(json_output).expect("guardrail3 output should be valid JSON");

    let sections = parsed["sections"]
        .as_array()
        .expect("sections should be array");

    let mut found = false;
    for section in sections {
        let results = section["results"].as_array().expect("results array");
        for result in results {
            if result["id"].as_str() == Some(check_id)
                && result["severity"].as_str() == Some(severity)
            {
                found = true;
            }
        }
    }

    assert!(
        found,
        "Expected check {check_id} with severity {severity} in output.\nFull output:\n{json_output}"
    );
}

/// Assert that the JSON output does NOT contain any result with the given check ID and severity.
#[allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic
)] // reason: test helper — JSON parsing + indexing + panic for assertion
fn assert_not_contains_check(json_output: &str, check_id: &str, severity: &str) {
    let parsed: serde_json::Value =
        serde_json::from_str(json_output).expect("guardrail3 output should be valid JSON");

    let sections = parsed["sections"]
        .as_array()
        .expect("sections should be array");

    for section in sections {
        let results = section["results"].as_array().expect("results array");
        for result in results {
            if result["id"].as_str() == Some(check_id)
                && result["severity"].as_str() == Some(severity)
            {
                panic!(
                    "Did NOT expect check {check_id} with severity {severity}, but found it.\n\
                     Result: {result}\nFull output:\n{json_output}"
                );
            }
        }
    }
}

// ============================================================
// Known-bad fixtures: SHOULD be flagged
// ============================================================

#[test]
fn adversarial_r30_crate_level_allow_detected() {
    let result = validate_fixture("allow_crate_wide.rs");
    assert_contains_check(&result, "R30", "error");
}

#[test]
fn adversarial_r32_allow_no_reason_detected() {
    let result = validate_fixture("allow_no_reason.rs");
    assert_contains_check(&result, "R32", "error");
}

#[test]
fn adversarial_r37_cfg_attr_allow_detected() {
    // R37 is Info severity — it's an inventory report, not an error
    let result = validate_fixture("allow_in_cfg_attr.rs");
    assert_contains_check(&result, "R37", "info");
}

#[test]
fn adversarial_r32_multiline_allow_detected() {
    // Multi-line #[allow(\n  clippy::unwrap_used\n)] — the checker processes
    // line-by-line. The first line `#[allow(` has no closing `)`, so it appends "...".
    // It should still produce R32 since there's no // comment.
    let result = validate_fixture("allow_multiline.rs");
    assert_contains_check(&result, "R32", "error");
}

#[test]
fn adversarial_r32_allow_in_macro_invisible_to_ast() {
    // Allow inside a macro_rules! body is INVISIBLE to syn AST analysis.
    // syn does not expand macros, so the body is opaque tokens.
    // This is a known limitation of AST-only analysis — no grep fallback.
    let result = validate_fixture("allow_in_macro.rs");
    assert!(
        !result.contains("\"R32\""),
        "AST-only analysis should NOT see #[allow] inside macro_rules! bodies"
    );
}

#[test]
fn adversarial_r34_garde_skip_no_reason_detected() {
    let result = validate_fixture_with_args("garde_skip_no_reason.rs", &["--garde"]);
    assert_contains_check(&result, "R34", "error");
}

#[test]
fn adversarial_r38_file_too_long_detected() {
    let result = validate_fixture("file_too_long.rs");
    assert_contains_check(&result, "R38", "error");
}

#[test]
fn adversarial_r40_too_many_uses_detected() {
    let result = validate_fixture("too_many_uses.rs");
    assert_contains_check(&result, "R40", "error");
}

// R42, R43, R44 REMOVED — these are enforced by clippy lints (configured via R26),
// not by guardrail3 source scanning. guardrail3's job is to verify the lint
// configuration exists, not to re-implement the linter.

#[test]
fn adversarial_r58_direct_std_fs_detected() {
    let result = validate_fixture("direct_std_fs.rs");
    assert_contains_check(&result, "R58", "error");
}

#[test]
fn adversarial_r32_reason_on_wrong_line_still_flagged() {
    // The reason comment is on the NEXT line, not the same line as #[allow].
    // The checker only looks for `//` on the SAME line, so this should be R32 error.
    let result = validate_fixture("allow_reason_wrong_line.rs");
    assert_contains_check(&result, "R32", "error");
}

#[test]
fn adversarial_r33_unicode_allow_treated_as_justified() {
    // The comment has a zero-width space in "rea​son" — not a valid `// reason:` prefix.
    // With the stricter check, this is now correctly caught as R32 (Error).
    let result = validate_fixture("unicode_allow.rs");
    assert_contains_check(&result, "R32", "error");
}

// ============================================================
// Known-good fixtures: should NOT be flagged as errors
// ============================================================

#[test]
fn adversarial_allow_with_reason_not_error() {
    let result = validate_fixture("allow_with_reason.rs");
    // Should be R33 Info, NOT R32 Error
    assert_contains_check(&result, "R33", "info");
    assert_not_contains_check(&result, "R32", "error");
}

#[test]
fn adversarial_clean_file_no_source_scan_errors() {
    let result = validate_fixture("clean_file.rs");
    // Should have no R30-R44 or R58 errors
    for check_id in &[
        "R30", "R32", "R34", "R37", "R38", "R40", "R42", "R43", "R44", "R58",
    ] {
        assert_not_contains_check(&result, check_id, "error");
    }
}
