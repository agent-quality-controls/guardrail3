//! Adversarial integration tests for T-TOOL check IDs.
//!
//! Each test creates a minimal TypeScript project in a temp directory, runs
//! `guardrail3 ts validate --format json`, and asserts T-TOOL check presence/absence
//! based on devDependencies, scripts, config files, and content-type configuration.
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

/// Create a TS project with package.json containing specified devDependencies,
/// scripts, and optional config files on disk.
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations to set up temp project
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
#[allow(clippy::type_complexity)] // reason: test helper — slice of tuples is clear in context
fn setup_ts_project_with_tools(
    dev_deps: &[&str],
    scripts: &[(&str, &str)],
    config_files: &[(&str, &str)],
    config: Option<&str>,
) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");

    let deps_json: Vec<String> = dev_deps.iter().map(|d| format!("\"{d}\": \"*\"")).collect();
    let scripts_json: Vec<String> = scripts
        .iter()
        .map(|(k, v)| format!("\"{k}\": \"{v}\""))
        .collect();
    let pkg = format!(
        r#"{{"name": "test", "devDependencies": {{{}}}, "scripts": {{{}}}}}"#,
        deps_json.join(", "),
        scripts_json.join(", ")
    );
    std::fs::write(tmp.path().join("package.json"), pkg).expect("write package.json");

    // Need at least one .ts file for TS detection
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("write ts");

    for (name, content) in config_files {
        std::fs::write(tmp.path().join(name), content).expect("write config");
    }

    if let Some(cfg) = config {
        std::fs::write(tmp.path().join("guardrail3.toml"), cfg).expect("write guardrail3.toml");
    }

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

/// Assert that NO check ID matching the prefix exists in the output.
fn assert_no_check_prefix(ids: &[String], prefix: &str, json_output: &str) {
    let matching: Vec<_> = ids.iter().filter(|id| id.starts_with(prefix)).collect();
    assert!(
        matching.is_empty(),
        "Did NOT expect any check starting with '{prefix}', but found: {matching:?}\nFull output:\n{json_output}"
    );
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

// ============================================================
// Test 1: T-TOOL-01 fires when cspell is missing from devDeps
// ============================================================

#[test]
fn t_tool_01_cspell_missing() {
    let tmp = setup_ts_project_with_tools(&[], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    assert_has_check(&error_ids, "T-TOOL-01", &output);
}

// ============================================================
// Test 2: T-TOOL-04 fires when prettier is missing from devDeps
// ============================================================

#[test]
fn t_tool_04_prettier_missing() {
    let tmp = setup_ts_project_with_tools(&[], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    assert_has_check(&error_ids, "T-TOOL-04", &output);
}

// ============================================================
// Test 3: T-TOOL-07 fires when cspell in devDeps but no config
// ============================================================

#[test]
fn t_tool_07_cspell_config_missing() {
    // cspell is installed but NO cspell.json or any cspell config variant
    let tmp = setup_ts_project_with_tools(&["cspell"], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    assert_has_check(&error_ids, "T-TOOL-07", &output);
}

// ============================================================
// Test 4: T-TOOL-07 passes when cspell.json exists
// ============================================================

#[test]
fn t_tool_07_cspell_config_present() {
    let tmp = setup_ts_project_with_tools(
        &["cspell"],
        &[],
        &[("cspell.json", r#"{"language": "en", "words": []}"#)],
        None,
    );
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // T-TOOL-07 should NOT fire as error — config exists
    assert_no_check(&error_ids, "T-TOOL-07", &output);
}

// ============================================================
// Test 5: T-TOOL-08 fires as warn when type-coverage dep present but no script
// ============================================================

#[test]
fn t_tool_08_type_coverage_script_missing() {
    // type-coverage in devDeps but no "type-coverage" script
    let tmp = setup_ts_project_with_tools(&["type-coverage"], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    assert_has_warn(&ids, "T-TOOL-08", &output);
}

// ============================================================
// Test 6: T-TOOL-08 passes when type-coverage script exists
// ============================================================

#[test]
fn t_tool_08_type_coverage_script_present() {
    let tmp = setup_ts_project_with_tools(
        &["type-coverage"],
        &[("type-coverage", "type-coverage --at-least 95")],
        &[],
        None,
    );
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    // T-TOOL-08 should NOT fire as warn/error — script exists
    assert_no_warn_or_error(&ids, "T-TOOL-08", &output);
}

// ============================================================
// Test 7: Content-only tools NOT checked without content type
// ============================================================

#[test]
fn t_tool_content_tools_not_checked_for_service() {
    // Project without any content type config — T-TOOL-05/06/11 should not appear
    let tmp = setup_ts_project_with_tools(&[], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let all_ids = collect_check_ids(&output);

    assert_no_check(&all_ids, "T-TOOL-05", &output);
    assert_no_check(&all_ids, "T-TOOL-06", &output);
    assert_no_check(&all_ids, "T-TOOL-11", &output);
}

// ============================================================
// Test 8: T-TOOL-05 fires when content project missing size-limit
// ============================================================

#[test]
fn t_tool_content_size_limit_missing() {
    let content_config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.checks]
content = true
"#;
    let tmp = setup_ts_project_with_tools(&[], &[], &[], Some(content_config));
    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // T-TOOL-05 should fire — size-limit missing from devDeps in content project
    assert_has_check(&error_ids, "T-TOOL-05", &output);
}

// ============================================================
// Test 9: T-TOOL-12 does NOT fire when no i18n library present
// ============================================================

#[test]
fn t_tool_12_i18n_no_library() {
    // Project without next-intl or any i18n library — T-TOOL-12 should not appear at all
    let tmp = setup_ts_project_with_tools(&[], &[], &[], None);
    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    // T-TOOL-12 should not fire in any severity — no i18n library detected
    let has_t_tool_12 = ids.iter().any(|(id, _)| id == "T-TOOL-12");
    assert!(
        !has_t_tool_12,
        "Did NOT expect T-TOOL-12 to appear at all (no i18n library), but found it.\nFull output:\n{output}"
    );
}

// ============================================================
// Test 10: T-TOOL-12 fires as warn when next-intl present but no messages dir
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test — writing custom package.json with dependencies for i18n test
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn t_tool_12_i18n_missing_messages() {
    let tmp = tempfile::tempdir().expect("tempdir");

    // next-intl in dependencies (not devDeps) — i18n check looks at both
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name": "test", "dependencies": {"next-intl": "*"}, "devDependencies": {}, "scripts": {}}"#,
    )
    .expect("write package.json");

    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("create src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("write ts");

    // i18n check is gated behind content_enabled — need content config
    let content_config = r#"
version = "0.1"

[typescript.checks]
content = true
"#;
    std::fs::write(tmp.path().join("guardrail3.toml"), content_config)
        .expect("write guardrail3.toml");

    let output = run_ts_validate(tmp.path(), &[]);
    let ids = collect_checks_with_severity(&output);

    // T-TOOL-12 should fire as warn — next-intl present but no messages directory
    assert_has_warn(&ids, "T-TOOL-12", &output);
}

// ============================================================
// Test 11: No T-TOOL errors when all tools, scripts, and configs present
// ============================================================

#[test]
fn t_tool_all_present() {
    let content_config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.checks]
content = true
"#;
    let tmp = setup_ts_project_with_tools(
        &[
            // Core tools (T-TOOL-01..04)
            "cspell",
            "type-coverage",
            "license-checker",
            "prettier",
            // Content-profile tools (T-TOOL-05..06)
            "size-limit",
            "@size-limit/preset-app",
        ],
        &[
            // Scripts (T-TOOL-08..10)
            ("type-coverage", "type-coverage --at-least 95"),
            ("license-check", "license-checker --onlyAllow 'MIT;ISC'"),
            ("audit", "pnpm audit --prod"),
        ],
        &[
            // Config files (T-TOOL-07)
            ("cspell.json", r#"{"language": "en", "words": []}"#),
        ],
        Some(content_config),
    );

    // Add size-limit config inline in package.json for T-TOOL-11
    // Need to rewrite package.json to include "size-limit" key
    #[allow(clippy::disallowed_methods)] // reason: test — rewriting package.json with size-limit config
    #[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
    {
        std::fs::write(
            tmp.path().join("package.json"),
            r#"{
                "name": "test",
                "devDependencies": {
                    "cspell": "*",
                    "type-coverage": "*",
                    "license-checker": "*",
                    "prettier": "*",
                    "size-limit": "*",
                    "@size-limit/preset-app": "*"
                },
                "scripts": {
                    "type-coverage": "type-coverage --at-least 95",
                    "license-check": "license-checker --onlyAllow 'MIT;ISC'",
                    "audit": "pnpm audit --prod"
                },
                "size-limit": [{"path": "dist/index.js", "limit": "10 kB"}]
            }"#,
        )
        .expect("rewrite package.json with size-limit");
    }

    let output = run_ts_validate(tmp.path(), &[]);
    let error_ids = collect_error_check_ids(&output);

    // No T-TOOL errors at all
    assert_no_check_prefix(&error_ids, "T-TOOL-", &output);
}
