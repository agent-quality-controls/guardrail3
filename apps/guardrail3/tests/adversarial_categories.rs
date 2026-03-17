//! Adversarial integration tests for guardrail3 check categories feature.
//!
//! Each test creates a minimal Rust project in a temp directory (with or without
//! guardrail3.toml), runs `guardrail3 rs validate --format json`, and asserts
//! that the correct check categories are present or absent based on config/CLI flags.
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

use std::process::Command;

const MINIMAL_CARGO_TOML: &str = r#"[package]
name = "category-test"
version = "0.1.0"
edition = "2024"

[lints]
workspace = true

[workspace.lints.rust]
unsafe_code = "forbid"
"#;

const MINIMAL_LIB_RS: &str = r#"pub fn hello() -> &'static str {
    "hello"
}
"#;

/// Create a temp project with a minimal Cargo.toml + src/lib.rs and optionally a guardrail3.toml.
/// Returns the temp dir (held alive by the caller).
#[allow(clippy::disallowed_methods)] // reason: fs operations needed to set up temp project
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_project(guardrail3_toml: Option<&str>) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).expect("create src/");
    std::fs::write(tmp.path().join("Cargo.toml"), MINIMAL_CARGO_TOML).expect("write Cargo.toml");
    std::fs::write(src_dir.join("lib.rs"), MINIMAL_LIB_RS).expect("write lib.rs");

    if let Some(toml_content) = guardrail3_toml {
        std::fs::write(tmp.path().join("guardrail3.toml"), toml_content)
            .expect("write guardrail3.toml");
    }

    tmp
}

/// Run guardrail3 rs validate --format json on the given path with extra CLI args.
/// Returns the JSON stdout as a string.
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn run_validate(path: &std::path::Path, extra_args: &[&str]) -> String {
    let mut args = vec!["rs", "validate", "--format", "json"];
    args.extend_from_slice(extra_args);
    args.push(path.to_str().expect("path"));

    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args(&args)
        .output()
        .expect("failed to run guardrail3");

    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Collect all check IDs from the JSON output into a Vec.
#[allow(clippy::expect_used, clippy::indexing_slicing)] // reason: test helper — JSON parsing for assertion
fn collect_check_ids(json_output: &str) -> Vec<String> {
    #[allow(clippy::disallowed_methods)] // reason: test helper — JSON parsing of guardrail3 output
    let parsed: serde_json::Value =
        serde_json::from_str(json_output).expect("guardrail3 output should be valid JSON");

    let sections = parsed["sections"]
        .as_array()
        .expect("sections should be array");

    let mut ids = Vec::new();
    for section in sections {
        let results = section["results"].as_array().expect("results array");
        for result in results {
            if let Some(id) = result["id"].as_str() {
                ids.push(id.to_owned());
            }
        }
    }
    ids
}

/// Assert that at least one check ID matching the prefix exists in the output.
#[allow(clippy::expect_used)] // reason: test assertion helper
fn assert_has_check_prefix(ids: &[String], prefix: &str, json_output: &str) {
    let found = ids.iter().any(|id| id.starts_with(prefix));
    assert!(
        found,
        "Expected at least one check starting with '{prefix}' in output.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert that NO check ID matching the prefix exists in the output.
fn assert_no_check_prefix(ids: &[String], prefix: &str, json_output: &str) {
    let matching: Vec<_> = ids.iter().filter(|id| id.starts_with(prefix)).collect();
    assert!(
        matching.is_empty(),
        "Did NOT expect any check starting with '{prefix}', but found: {matching:?}\nFull output:\n{json_output}"
    );
}

/// Assert a specific check ID exists in the output.
#[allow(clippy::expect_used)] // reason: test assertion helper
fn assert_has_check(ids: &[String], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|id| id == check_id);
    assert!(
        found,
        "Expected check '{check_id}' in output.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert a specific check ID does NOT exist in the output.
#[allow(dead_code)] // reason: test helper used by category tests below
fn assert_no_check(ids: &[String], check_id: &str, json_output: &str) {
    assert!(
        !ids.iter().any(|id| id == check_id),
        "Did NOT expect check '{check_id}', but it was present.\nFull output:\n{json_output}"
    );
}

// ============================================================
// Test 1: Default behavior — no config, no CLI flags
// Core checks present, architecture and garde absent
// ============================================================

#[test]
fn categories_default_no_config_has_core_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Core checks should be present (R1 = clippy.toml, R26 = workspace lints)
    assert_has_check(&ids, "R1", &output);
    assert_has_check(&ids, "R26", &output);
}

#[test]
fn categories_default_no_config_no_architecture_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Architecture checks should NOT appear by default
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_default_no_config_no_garde_checks() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Garde checks should NOT appear by default
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 2: Config enables architecture category
// ============================================================

#[test]
fn categories_config_enables_architecture() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
architecture = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Architecture checks SHOULD appear when config enables them
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
}

// ============================================================
// Test 3: Config enables garde category
// ============================================================

#[test]
fn categories_config_enables_garde() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
garde = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Garde checks SHOULD appear when config enables them
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 4: CLI --garde flag enables garde without config
// ============================================================

#[test]
fn categories_cli_garde_flag_enables_garde() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--garde"]);
    let ids = collect_check_ids(&output);

    // Garde checks SHOULD appear when --garde flag is used
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
}

// ============================================================
// Test 5: CLI --architecture flag enables architecture, excludes tests
// ============================================================

#[test]
fn categories_cli_architecture_flag_enables_architecture() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    // Architecture checks SHOULD appear when --architecture flag is used
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_cli_architecture_flag_excludes_tests() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    // Test checks should NOT appear when only --architecture is specified
    // (CLI flags act as a filter — only the requested category runs)
    assert_no_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Test 6: Tests category defaults to on
// ============================================================

#[test]
fn categories_tests_default_on_without_config() {
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Test checks SHOULD appear by default (tests category defaults to true)
    assert_has_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Test 7: Config disables tests category
// ============================================================

#[test]
fn categories_config_disables_tests() {
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
tests = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Test checks should NOT appear when config disables them
    assert_no_check_prefix(&ids, "R-TEST-", &output);
}

// ============================================================
// Additional adversarial tests: edge cases and interactions
// ============================================================

#[test]
fn categories_cli_flag_overrides_config_disabled() {
    // Config disables tests, but CLI --tests flag should re-enable them
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
tests = false
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &["--tests"]);
    let ids = collect_check_ids(&output);

    // CLI --tests flag should enable test checks regardless of config
    assert_has_check_prefix(&ids, "R-TEST-", &output);
}

#[test]
fn categories_cli_garde_does_not_include_architecture() {
    // --garde flag should only enable garde, not architecture
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--garde"]);
    let ids = collect_check_ids(&output);

    assert_has_check_prefix(&ids, "R-GARDE-", &output);
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
}

#[test]
fn categories_cli_architecture_does_not_include_garde() {
    // --architecture flag should only enable architecture, not garde
    let tmp = setup_project(None);
    let output = run_validate(tmp.path(), &["--architecture"]);
    let ids = collect_check_ids(&output);

    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}

#[test]
fn categories_config_enables_all_optional() {
    // Config enables all optional categories at once
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
architecture = true
garde = true
tests = true
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // All categories should be present
    assert_has_check_prefix(&ids, "R-ARCH-", &output);
    assert_has_check_prefix(&ids, "R-GARDE-", &output);
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    // Core checks should still be present
    assert_has_check(&ids, "R1", &output);
}

#[test]
fn categories_empty_checks_section_uses_defaults() {
    // Config has [rust.checks] but no fields — should use defaults (tests=true, rest=false)
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.checks]
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Default: tests on, architecture/garde off
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}

#[test]
fn categories_config_without_checks_section_uses_defaults() {
    // Config has [rust] but no [rust.checks] — should use defaults
    let config = r#"
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."
"#;
    let tmp = setup_project(Some(config));
    let output = run_validate(tmp.path(), &[]);
    let ids = collect_check_ids(&output);

    // Default: tests on, architecture/garde off
    assert_has_check_prefix(&ids, "R-TEST-", &output);
    assert_no_check_prefix(&ids, "R-ARCH-", &output);
    assert_no_check_prefix(&ids, "R-GARDE-", &output);
}
