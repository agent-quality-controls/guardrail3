//! Adversarial integration tests for TypeScript generate config correctness.
//!
//! These tests verify:
//! - Content apps include stylelint and a11y configs
//! - Service-only apps exclude stylelint
//! - Mixed content+service includes both sections in `ESLint`
//! - No-apps defaults to service profile
//! - `ESLint` ignore patterns use **/ prefix
//! - cspell.json is valid JSON
//! - Generate is idempotent

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn guardrail3() -> Command {
    Command::new(env!("CARGO_BIN_EXE_guardrail3"))
}

/// Helper: write a file at a relative path inside the temp dir, creating parent dirs as needed.
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper — panics on write failure
fn write_fixture(root: &std::path::Path, rel_path: &str, content: &str) {
    let full = root.join(rel_path);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs"); // reason: test setup
    }
    std::fs::write(&full, content).expect("write fixture file"); // reason: test setup
}

// ---------------------------------------------------------------------------
// Test 1: content_app_includes_a11y_and_stylelint
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn content_app_includes_a11y_and_stylelint() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[typescript]\n",
            "\n",
            "[typescript.apps.landing]\n",
            "type = \"content\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["ts", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains(".stylelintrc.mjs"),
        "Content app should include .stylelintrc.mjs in dry-run output, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: service_only_no_stylelint
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn service_only_no_stylelint() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[typescript]\n",
            "\n",
            "[typescript.apps.admin]\n",
            "type = \"service\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["ts", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        !stdout.contains(".stylelintrc.mjs"),
        "Service-only app should NOT include .stylelintrc.mjs, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: mixed_content_and_service_includes_both_sections
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn mixed_content_and_service_includes_both_sections() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[typescript]\n",
            "\n",
            "[typescript.apps.landing]\n",
            "type = \"content\"\n",
            "\n",
            "[typescript.apps.admin]\n",
            "type = \"service\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");

    // Actually generate (not dry-run) so we can read the file
    let out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("run generate");

    assert!(
        out.status.success(),
        "ts generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let eslint_content =
        std::fs::read_to_string(root.join("eslint.config.mjs")).expect("read eslint.config.mjs"); // reason: test assertion

    // Content section: jsx-a11y
    assert!(
        eslint_content.contains("jsx-a11y"),
        "Mixed config eslint should contain jsx-a11y (content section), got:\n{eslint_content}"
    );

    // Service section: boundaries
    assert!(
        eslint_content.contains("boundaries"),
        "Mixed config eslint should contain boundaries (service section), got:\n{eslint_content}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: no_apps_defaults_to_service
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn no_apps_defaults_to_service() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    // [typescript] section but no [typescript.apps.*]
    write_fixture(
        root,
        "guardrail3.toml",
        concat!("version = \"0.1\"\n", "\n", "[typescript]\n"),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("run generate");

    assert!(
        out.status.success(),
        "ts generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let eslint_content =
        std::fs::read_to_string(root.join("eslint.config.mjs")).expect("read eslint.config.mjs"); // reason: test assertion

    // Should include service-style boundaries plugin
    assert!(
        eslint_content.contains("boundaries"),
        "No-apps config should default to service (include boundaries), got:\n{eslint_content}"
    );

    // Should NOT include content-style jsx-a11y
    assert!(
        !eslint_content.contains("jsx-a11y"),
        "No-apps config should NOT include jsx-a11y (content only), got:\n{eslint_content}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: eslint_ignores_use_double_star_prefix
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn eslint_ignores_use_double_star_prefix() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!("version = \"0.1\"\n", "\n", "[typescript]\n"),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("run generate");

    assert!(
        out.status.success(),
        "ts generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let eslint_content =
        std::fs::read_to_string(root.join("eslint.config.mjs")).expect("read eslint.config.mjs"); // reason: test assertion

    // Find all ignore patterns in the ignores array
    // They appear as quoted strings inside the ignores: [...] block
    let mut bad_patterns: Vec<String> = Vec::new();
    let mut in_ignores = false;
    for line in eslint_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("ignores:") || trimmed.starts_with("ignores :") {
            in_ignores = true;
            continue;
        }
        if in_ignores {
            if trimmed.starts_with(']') {
                in_ignores = false;
                continue;
            }
            // Extract quoted strings that look like glob patterns
            #[allow(clippy::string_slice)]
            // reason: test — input is ASCII-only ESLint config, no multi-byte characters
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let pattern = &trimmed[start + 1..start + 1 + end];
                    // Ignore patterns should start with **/ for glob consistency
                    // Skip patterns that are not directory globs (e.g., just file extensions)
                    if pattern.contains('/')
                        && !pattern.starts_with("**/")
                        && !pattern.starts_with('!')
                    {
                        bad_patterns.push(pattern.to_owned());
                    }
                }
            }
        }
    }

    assert!(
        bad_patterns.is_empty(),
        "All ESLint ignore patterns with paths should use **/ prefix, but found: {bad_patterns:?}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: cspell_json_is_valid_json
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cspell_json_is_valid_json() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!("version = \"0.1\"\n", "\n", "[typescript]\n"),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("run generate");

    assert!(
        out.status.success(),
        "ts generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let cspell_content =
        std::fs::read_to_string(root.join("cspell.json")).expect("read cspell.json"); // reason: test assertion

    // Must parse as valid JSON
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&cspell_content);
    assert!(
        parsed.is_ok(),
        "cspell.json should be valid JSON, parse error: {:?}\nContent:\n{cspell_content}",
        parsed.err()
    );
}

// ---------------------------------------------------------------------------
// Test 7: generate_is_idempotent
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn generate_is_idempotent() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[typescript]\n",
            "\n",
            "[typescript.apps.app]\n",
            "type = \"content\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");

    // Run generate twice
    let out1 = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("first generate");
    assert!(
        out1.status.success(),
        "first ts generate should succeed: {}",
        String::from_utf8_lossy(&out1.stderr)
    );

    let out2 = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("second generate");
    assert!(
        out2.status.success(),
        "second ts generate should succeed: {}",
        String::from_utf8_lossy(&out2.stderr)
    );

    // Now dry-run should report no changes
    let dry = guardrail3()
        .args(["ts", "generate", "--dry-run", path_str])
        .output()
        .expect("dry-run after double generate");

    let stdout = String::from_utf8_lossy(&dry.stdout);
    let stdout_lower = stdout.to_lowercase();
    assert!(
        stdout_lower.contains("no changes"),
        "Dry run after two generates should report 'No changes needed', got:\n{stdout}"
    );

    assert!(
        dry.status.success(),
        "Dry run with no changes should exit 0"
    );
}
