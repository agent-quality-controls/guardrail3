//! Adversarial integration tests for T-STYL stylelint configuration checks.
//!
//! These tests create temporary content-type TypeScript projects and verify that
//! guardrail3 correctly detects missing or incomplete stylelint configurations.

// Suppress unused-crate-dependencies for workspace deps not used in this test binary.
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use semver as _;
use serde as _;
use serde_yaml as _;
use std::path::Path;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a temp content-type TypeScript project with optional stylelint config.
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations for temp project setup
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_content_project_with_stylelint(stylelint_content: Option<&str>) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::write(tmp.path().join("package.json"), r#"{"name":"test"}"#).expect("pkg");

    // Create content app structure
    let app_dir = tmp.path().join("apps").join("landing");
    let src = app_dir.join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(app_dir.join("package.json"), r#"{"name":"landing"}"#).expect("app pkg");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("ts");

    // Config declaring content type
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        r#"
version = "0.1"
[typescript.apps.landing]
type = "content"
"#,
    )
    .expect("config");

    if let Some(content) = stylelint_content {
        std::fs::write(tmp.path().join(".stylelintrc.mjs"), content).expect("stylelint");
    }

    tmp
}

/// Run `guardrail3 ts validate --format json --inventory` on the given path.
#[allow(clippy::disallowed_methods)] // reason: test helper — Command::new for binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn run_ts_validate(path: &Path) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args(["ts", "validate", "--format", "json", "--inventory"])
        .arg(path)
        .output()
        .expect("failed to run guardrail3");

    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Collect all check IDs from the JSON output.
#[allow(clippy::expect_used)] // reason: test helper — JSON parsing for assertion
#[allow(clippy::indexing_slicing)] // reason: test helper — JSON access
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

/// Assert that a specific check ID exists in the output.
fn assert_has_check(ids: &[String], check_id: &str, json_output: &str) {
    assert!(
        ids.iter().any(|id| id == check_id),
        "Expected check '{check_id}' in output.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert that a specific check ID does NOT exist in the output.
#[allow(dead_code)] // reason: test helper — available for future stylelint adversarial tests
fn assert_no_check(ids: &[String], check_id: &str, json_output: &str) {
    assert!(
        !ids.iter().any(|id| id == check_id),
        "Did NOT expect check '{check_id}', but it was present.\nFull output:\n{json_output}"
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

// ---------------------------------------------------------------------------
// Full stylelint config for passing tests
// ---------------------------------------------------------------------------

const FULL_STYLELINT_CONFIG: &str = r"export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: ['@double-great/stylelint-a11y'],
  rules: {
    'a11y/content-property-no-static-value': true,
    'a11y/font-size-is-readable': true,
    'a11y/line-height-is-vertical-rhythmed': true,
    'a11y/media-prefers-reduced-motion': true,
    'a11y/no-display-none': true,
    'a11y/no-obsolete-attribute': true,
    'a11y/no-obsolete-element': true,
    'a11y/no-outline-none': true,
    'a11y/no-spread-text': true,
    'a11y/no-text-align-justify': true,
    'a11y/selector-pseudo-class-focus': true,
  },
};";

// ===========================================================================
// Test 1: T-STYL-01 fires when .stylelintrc.mjs is missing
// ===========================================================================

#[test]
fn t_styl_01_config_missing() {
    let tmp = setup_content_project_with_stylelint(None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // Content project without any stylelint config → T-STYL-01 should fire as error
    assert_has_check(&ids, "T-STYL-01", &output);
}

// ===========================================================================
// Test 2: All T-STYL checks pass with full config
// ===========================================================================

#[test]
fn t_styl_complete_config() {
    let tmp = setup_content_project_with_stylelint(Some(FULL_STYLELINT_CONFIG));
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // With a complete config, all T-STYL checks should be present (as inventory/info)
    // and none should be missing from the output (--inventory shows passing checks too)
    assert_has_check(&ids, "T-STYL-01", &output);
    assert_has_check(&ids, "T-STYL-02", &output);
    assert_has_check(&ids, "T-STYL-03", &output);
    assert_has_check(&ids, "T-STYL-04", &output);
    assert_has_check(&ids, "T-STYL-05", &output);
}

// ===========================================================================
// Test 3: T-STYL-04 fires when a11y plugin is missing
// ===========================================================================

#[test]
fn t_styl_04_missing_a11y_plugin() {
    // Config has extends but no @double-great/stylelint-a11y plugin
    let config = r"export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: [],
  rules: {},
};";

    let tmp = setup_content_project_with_stylelint(Some(config));
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-STYL-04 should fire — a11y plugin is missing
    assert_has_check(&ids, "T-STYL-04", &output);
}

// ===========================================================================
// Test 4: T-STYL-05 fires when a11y rules are missing
// ===========================================================================

#[test]
fn t_styl_05_missing_a11y_rules() {
    // Config has the plugin but only some of the required a11y rules
    let config = r"export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: ['@double-great/stylelint-a11y'],
  rules: {
    'a11y/content-property-no-static-value': true,
    'a11y/font-size-is-readable': true,
  },
};";

    let tmp = setup_content_project_with_stylelint(Some(config));
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-STYL-05 should fire — most a11y rules are missing
    assert_has_check(&ids, "T-STYL-05", &output);

    // T-STYL-04 should pass — plugin IS present
    // (with --inventory, the passing check should appear)
    assert_has_check(&ids, "T-STYL-04", &output);
}

// ===========================================================================
// Test 5: T-STYL checks do NOT fire for service-type projects
// ===========================================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test setup — fs operations for temp project
#[allow(clippy::expect_used)] // reason: test setup — panics indicate broken test infrastructure
fn t_styl_not_checked_for_service() {
    // Create a service-type project (not content) — no stylelint checks should appear
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::write(tmp.path().join("package.json"), r#"{"name":"test"}"#).expect("pkg");

    let app_dir = tmp.path().join("apps").join("api");
    let src = app_dir.join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(app_dir.join("package.json"), r#"{"name":"api"}"#).expect("app pkg");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("ts");

    // Config declaring service type (NOT content)
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        r#"
version = "0.1"
[typescript.apps.api]
type = "service"
"#,
    )
    .expect("config");

    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // Service project → no T-STYL checks should appear at all
    assert_no_check_prefix(&ids, "T-STYL-", &output);
}

// ---------------------------------------------------------------------------
// Severity-aware helpers for warn checks
// ---------------------------------------------------------------------------

/// Collect check IDs with their severity from JSON output.
#[allow(clippy::expect_used, clippy::indexing_slicing, clippy::type_complexity)] // reason: test helper — JSON parsing for severity assertion; tuple vec is clear in context
fn collect_checks_with_severity(json_output: &str) -> Vec<(String, String)> {
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
            let id = result["id"].as_str().unwrap_or("").to_owned();
            let severity = result["severity"].as_str().unwrap_or("").to_owned();
            ids.push((id, severity));
        }
    }
    ids
}

/// Assert that a specific check ID fired as warn.
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_has_warn(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|(id, sev)| id == check_id && sev == "warn");
    assert!(
        found,
        "Expected check '{check_id}' to fire as warn.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert that a specific check ID did NOT fire as warn or error.
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_no_warn_or_error(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found = ids
        .iter()
        .any(|(id, sev)| id == check_id && (sev == "warn" || sev == "error"));
    assert!(
        !found,
        "Did NOT expect check '{check_id}' as warn/error, but it was present.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

// ===========================================================================
// Test 6: T-STYL-06 fires when a11y rules present but exception rules missing
// ===========================================================================

#[test]
fn t_styl_06_missing_exceptions() {
    // Config has all 11 a11y rules but does NOT include the exception rules
    // (a11y/media-prefers-color-scheme and no-duplicate-selectors)
    let config = r"export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: ['@double-great/stylelint-a11y'],
  rules: {
    'a11y/content-property-no-static-value': true,
    'a11y/font-size-is-readable': true,
    'a11y/line-height-is-vertical-rhythmed': true,
    'a11y/media-prefers-reduced-motion': true,
    'a11y/no-display-none': true,
    'a11y/no-obsolete-attribute': true,
    'a11y/no-obsolete-element': true,
    'a11y/no-outline-none': true,
    'a11y/no-spread-text': true,
    'a11y/no-text-align-justify': true,
    'a11y/selector-pseudo-class-focus': true,
  },
};";

    let tmp = setup_content_project_with_stylelint(Some(config));
    let output = run_ts_validate(tmp.path());
    let ids = collect_checks_with_severity(&output);

    // T-STYL-06 should fire as warn — exception rules not present
    assert_has_warn(&ids, "T-STYL-06", &output);
}

// ===========================================================================
// Test 7: T-STYL-06 does NOT fire when exception rules are present
// ===========================================================================

#[test]
fn t_styl_06_exceptions_present() {
    // Config has all 11 a11y rules AND both exception rules disabled via null
    let config = r"export default {
  extends: ['stylelint-config-standard', 'stylelint-config-tailwindcss'],
  plugins: ['@double-great/stylelint-a11y'],
  rules: {
    'a11y/content-property-no-static-value': true,
    'a11y/font-size-is-readable': true,
    'a11y/line-height-is-vertical-rhythmed': true,
    'a11y/media-prefers-reduced-motion': true,
    'a11y/no-display-none': true,
    'a11y/no-obsolete-attribute': true,
    'a11y/no-obsolete-element': true,
    'a11y/no-outline-none': true,
    'a11y/no-spread-text': true,
    'a11y/no-text-align-justify': true,
    'a11y/selector-pseudo-class-focus': true,
    'a11y/media-prefers-color-scheme': null,
    'no-duplicate-selectors': null,
  },
};";

    let tmp = setup_content_project_with_stylelint(Some(config));
    let output = run_ts_validate(tmp.path());
    let ids = collect_checks_with_severity(&output);

    // T-STYL-06 should NOT fire as warn/error — exception rules are present
    assert_no_warn_or_error(&ids, "T-STYL-06", &output);
}
