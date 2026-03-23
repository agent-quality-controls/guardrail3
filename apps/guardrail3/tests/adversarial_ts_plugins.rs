//! Adversarial integration tests for T-PLUG package check IDs.
//!
//! Each test creates a minimal TypeScript project in a temp directory, runs
//! `guardrail3 ts validate --format json`, and asserts T-PLUG check presence/absence
//! based on devDependencies and content-type configuration.
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

/// Create a TS project with package.json containing specified devDependencies.
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations to set up temp project
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_ts_project(dev_deps: &[&str], config: Option<&str>) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");

    let deps_json: Vec<String> = dev_deps.iter().map(|d| format!("\"{d}\": \"*\"")).collect();
    let pkg = format!(
        r#"{{"name": "test", "devDependencies": {{{}}}}}"#,
        deps_json.join(", ")
    );
    std::fs::write(tmp.path().join("package.json"), pkg).expect("write package.json");

    // Need at least one .ts file for TS detection
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("write ts");

    if let Some(cfg) = config {
        std::fs::write(tmp.path().join("guardrail3.toml"), cfg).expect("write config");
    }

    tmp
}

/// Create a TS project with a content-type app directory structure.
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations to set up temp project with apps
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_ts_project_with_app(dev_deps: &[&str], config: &str, app_name: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");

    let deps_json: Vec<String> = dev_deps.iter().map(|d| format!("\"{d}\": \"*\"")).collect();
    let pkg = format!(
        r#"{{"name": "test-monorepo", "private": true, "devDependencies": {{{}}}}}"#,
        deps_json.join(", ")
    );
    std::fs::write(tmp.path().join("package.json"), pkg).expect("write package.json");

    // Create the app directory with a .ts file
    let app_dir = tmp.path().join("apps").join(app_name);
    let app_src = app_dir.join("src");
    std::fs::create_dir_all(&app_src).expect("create app src");
    std::fs::write(
        app_dir.join("package.json"),
        format!(r#"{{"name": "{app_name}", "version": "0.1.0"}}"#),
    )
    .expect("write app package.json");
    std::fs::write(app_src.join("index.ts"), "export const x = 1;").expect("write app ts");

    std::fs::write(tmp.path().join("guardrail3.toml"), config).expect("write config");

    tmp
}

/// Run guardrail3 ts validate --format json on the given path with extra CLI args.
#[allow(clippy::disallowed_methods)] // reason: test helper — Command::new for binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn run_ts_validate(path: &Path, extra_args: &[&str]) -> String {
    let mut args = vec!["ts", "validate", "--format", "json"];
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

/// Collect check IDs that have severity "error" from the JSON output.
#[allow(clippy::expect_used, clippy::indexing_slicing)] // reason: test helper — JSON parsing for assertion
fn collect_error_check_ids(json_output: &str) -> Vec<String> {
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
            if result["severity"].as_str() == Some("error") {
                if let Some(id) = result["id"].as_str() {
                    ids.push(id.to_owned());
                }
            }
        }
    }
    ids
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
fn assert_no_check(ids: &[String], check_id: &str, json_output: &str) {
    assert!(
        !ids.iter().any(|id| id == check_id),
        "Did NOT expect check '{check_id}', but it was present.\nFull output:\n{json_output}"
    );
}

// ============================================================
// Test 1: Missing eslint-plugin-unicorn fires T-PLUG-01
// ============================================================

#[test]
fn t_plug_01_missing_unicorn() {
    // Project without eslint-plugin-unicorn in devDependencies
    let tmp = setup_ts_project(&[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // T-PLUG-01 should fire as error because eslint-plugin-unicorn is missing
    assert_has_check(&error_ids, "T-PLUG-01", &output);
}

// ============================================================
// Test 2: Present eslint-plugin-unicorn — T-PLUG-01 not error
// ============================================================

#[test]
fn t_plug_01_present_unicorn() {
    // Project WITH eslint-plugin-unicorn in devDependencies
    let tmp = setup_ts_project(&["eslint-plugin-unicorn"], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // T-PLUG-01 should NOT fire as error — plugin is present (will be inventory/info)
    assert_no_check(&error_ids, "T-PLUG-01", &output);
}

// ============================================================
// Test 3: All 4 core plugins present — no T-PLUG errors for 01/02/03/10
// ============================================================

#[test]
fn t_plug_core_all_present() {
    let tmp = setup_ts_project(
        &[
            "eslint-plugin-unicorn",
            "eslint-plugin-regexp",
            "eslint-plugin-sonarjs",
            "knip",
        ],
        None,
    );
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // None of the core plugin checks should fire as errors
    assert_no_check(&error_ids, "T-PLUG-01", &output);
    assert_no_check(&error_ids, "T-PLUG-02", &output);
    assert_no_check(&error_ids, "T-PLUG-03", &output);
    assert_no_check(&error_ids, "T-PLUG-10", &output);
}

// ============================================================
// Test 4: Content plugins NOT checked without content type
// ============================================================

#[test]
fn t_plug_content_not_checked_without_content_type() {
    // Project WITHOUT content type config — content plugins should not be checked at all.
    // Even though jsx-a11y is missing, T-PLUG-04 should NOT appear.
    let tmp = setup_ts_project(&[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let all_ids = collect_check_ids(&output);

    // Content-profile plugin checks should not appear at all (not even as inventory)
    assert_no_check(&all_ids, "T-PLUG-04", &output);
    assert_no_check(&all_ids, "T-PLUG-05", &output);
    assert_no_check(&all_ids, "T-PLUG-06", &output);
    assert_no_check(&all_ids, "T-PLUG-07", &output);
    assert_no_check(&all_ids, "T-PLUG-08", &output);
    assert_no_check(&all_ids, "T-PLUG-09", &output);
}

// ============================================================
// Test 5: Content-type project missing jsx-a11y fires T-PLUG-04
// ============================================================

#[test]
fn t_plug_content_checked_with_content_type() {
    let config = r#"
version = "0.1"

[typescript.apps.landing]
type = "content"
"#;
    // Content-type project with NO content plugins installed
    let tmp = setup_ts_project_with_app(&[], config, "landing");
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // T-PLUG-04 should fire as error because jsx-a11y is missing
    assert_has_check(&error_ids, "T-PLUG-04", &output);
}

// ============================================================
// Test 6: Content project with all content plugins — no T-PLUG-04..09 errors
// ============================================================

#[test]
fn t_plug_content_all_present() {
    let config = r#"
version = "0.1"

[typescript.apps.landing]
type = "content"
"#;
    let tmp = setup_ts_project_with_app(
        &[
            // Core plugins (to avoid T-PLUG-01/02/03/10 errors)
            "eslint-plugin-unicorn",
            "eslint-plugin-regexp",
            "eslint-plugin-sonarjs",
            "knip",
            // Core toolchain (T-PLUG-12..19)
            "eslint",
            "typescript",
            "typescript-eslint",
            "eslint-plugin-import-x",
            "eslint-import-resolver-typescript",
            "eslint-plugin-boundaries",
            "only-allow",
            "jscpd",
            // Content plugins
            "eslint-plugin-jsx-a11y",
            "stylelint",
            "@double-great/stylelint-a11y",
            "stylelint-config-standard",
            "stylelint-config-tailwindcss",
            "eslint-plugin-tailwind-ban",
        ],
        config,
        "landing",
    );
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // None of the content plugin checks should fire as errors
    assert_no_check(&error_ids, "T-PLUG-04", &output);
    assert_no_check(&error_ids, "T-PLUG-05", &output);
    assert_no_check(&error_ids, "T-PLUG-06", &output);
    assert_no_check(&error_ids, "T-PLUG-07", &output);
    assert_no_check(&error_ids, "T-PLUG-08", &output);
    assert_no_check(&error_ids, "T-PLUG-09", &output);
    // Core plugins should also not fire (all provided in devDeps)
    assert_no_check(&error_ids, "T-PLUG-01", &output);
    assert_no_check(&error_ids, "T-PLUG-02", &output);
    assert_no_check(&error_ids, "T-PLUG-03", &output);
    assert_no_check(&error_ids, "T-PLUG-10", &output);
    assert_no_check(&error_ids, "T-PLUG-12", &output);
    assert_no_check(&error_ids, "T-PLUG-13", &output);
    assert_no_check(&error_ids, "T-PLUG-14", &output);
    assert_no_check(&error_ids, "T-PLUG-15", &output);
    assert_no_check(&error_ids, "T-PLUG-16", &output);
    assert_no_check(&error_ids, "T-PLUG-17", &output);
    assert_no_check(&error_ids, "T-PLUG-18", &output);
    assert_no_check(&error_ids, "T-PLUG-19", &output);
}

// ---------------------------------------------------------------------------
// Warn-severity helper
// ---------------------------------------------------------------------------

/// Collect check IDs with their severity from JSON output.
#[allow(clippy::expect_used, clippy::indexing_slicing, clippy::type_complexity)] // reason: test helper — JSON parsing for assertion; tuple vec is clear in context
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

/// Assert that a specific check ID fired as error.
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_has_error(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|(id, sev)| id == check_id && sev == "error");
    assert!(
        found,
        "Expected check '{check_id}' to fire as error.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
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

// ============================================================
// Test 7: T-PLUG-11 fires when knip dep present but no script
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test — writing custom package.json for knip script test
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn t_plug_11_knip_script_missing() {
    let tmp = tempfile::tempdir().expect("tempdir");

    // knip in devDeps but NO "knip" script
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name":"test","devDependencies":{"knip":"*"},"scripts":{}}"#,
    )
    .expect("write package.json");

    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("write ts");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    // T-PLUG-11 should fire as error — knip dep present but no script to run it
    assert_has_error(&ids, "T-PLUG-11", &output);
}

// ============================================================
// Test 8: T-PLUG-11 does NOT fire when knip script is present
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test — writing custom package.json for knip script test
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn t_plug_11_knip_script_present() {
    let tmp = tempfile::tempdir().expect("tempdir");

    // knip in devDeps AND "knip" script present
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name":"test","devDependencies":{"knip":"*"},"scripts":{"knip":"knip"}}"#,
    )
    .expect("write package.json");

    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("write ts");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    // T-PLUG-11 should NOT fire as warn/error — knip script exists
    assert_no_warn_or_error(&ids, "T-PLUG-11", &output);
}
