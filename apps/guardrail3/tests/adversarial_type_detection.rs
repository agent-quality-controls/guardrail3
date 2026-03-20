//! Adversarial integration tests for TypeScript app type auto-detection.
//!
//! These tests exercise `ts init --dry-run` and `ts generate --dry-run` with
//! various app directory structures and package.json configurations to verify
//! that auto-detection correctly identifies content, service, and library app types.

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
use serde_json as _;
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

/// Helper: write a file at a relative path inside the temp dir, creating parent dirs.
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper — panics on write failure
fn write_file(root: &std::path::Path, rel: &str, content: &str) {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs"); // reason: test setup
    }
    std::fs::write(&path, content).expect("write file"); // reason: test setup
}

/// Helper: set up a root package.json with pnpm workspace config.
#[allow(clippy::disallowed_methods)] // reason: test helper — writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper — panics on write failure
fn write_root_package_json(root: &std::path::Path) {
    write_file(
        root,
        "package.json",
        r#"{"name": "test-monorepo", "private": true}"#,
    );
}

/// Helper: run `ts init --dry-run` and return (stdout, stderr).
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics on command failure
fn run_ts_init_dry_run(path: &std::path::Path) -> (String, String) {
    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup
    let out = guardrail3()
        .args(["ts", "init", "--dry-run", path_str])
        .output()
        .expect("failed to run guardrail3"); // reason: test infra

    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (stdout, stderr)
}

/// Helper: run `ts generate --dry-run` and return (stdout, stderr).
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics on command failure
fn run_ts_generate_dry_run(path: &std::path::Path) -> (String, String) {
    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup
    let out = guardrail3()
        .args(["ts", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run guardrail3"); // reason: test infra

    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (stdout, stderr)
}

// ============================================================
// Test 1: velite in devDependencies detected as content
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn velite_in_devdeps_detected_as_content() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // App with velite in devDependencies (not dependencies)
    write_file(
        root,
        "apps/landing/package.json",
        r#"{"name": "landing", "version": "0.1.0", "devDependencies": {"velite": "^0.1.0"}}"#,
    );
    write_file(root, "apps/landing/src/index.ts", "export const x = 1;");

    let (stdout, stderr) = run_ts_init_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // ts init --dry-run should detect landing as content type
    assert!(
        combined.contains("type = \"content\""),
        "velite in devDependencies should trigger content type detection.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 2: content/ directory detected as content
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn content_dir_detected_as_content() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // App with content/ subdirectory, basic package.json (no content deps)
    write_file(
        root,
        "apps/blog/package.json",
        r#"{"name": "blog", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/blog/src/index.ts", "export const x = 1;");
    // Create content/ directory with a file inside so it's a real directory
    write_file(root, "apps/blog/content/post.md", "# Hello");

    let (stdout, stderr) = run_ts_init_dry_run(root);

    // Should find the blog app's type section
    // The init output prints the generated TOML content
    let has_content_type = stdout.contains("type = \"content\"");
    assert!(
        has_content_type,
        "content/ directory should trigger content type detection.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 3: hex arch structure detected as service
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn hex_arch_detected_as_service() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // App with hex arch directory structure
    write_file(
        root,
        "apps/admin/package.json",
        r#"{"name": "admin", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/admin/src/index.ts", "export const x = 1;");
    write_file(
        root,
        "apps/admin/src/modules/domain/index.ts",
        "export type X = string;",
    );

    let (stdout, stderr) = run_ts_init_dry_run(root);

    // Hex arch structure should detect as service
    assert!(
        stdout.contains("type = \"service\""),
        "src/modules/domain/ should trigger service type detection.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 4: no signals defaults to service
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn no_signals_defaults_to_service() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // App with no content deps, no hex arch, no content/ dir
    write_file(
        root,
        "apps/misc/package.json",
        r#"{"name": "misc", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/misc/src/index.ts", "export const x = 1;");

    let (stdout, stderr) = run_ts_init_dry_run(root);

    // No signals → default to service
    assert!(
        stdout.contains("type = \"service\""),
        "No detection signals should default to service type.\nstdout: {stdout}\nstderr: {stderr}"
    );
    // Also verify the detection reason mentions default
    assert!(
        stdout.contains("default"),
        "Should mention 'default' in the detection reason comment.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 5: content and service signals — content/ dir vs hex arch
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn content_and_service_signals_resolution() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // App with BOTH content/ directory AND src/modules/domain/ (hex arch)
    write_file(
        root,
        "apps/hybrid/package.json",
        r#"{"name": "hybrid", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/hybrid/src/index.ts", "export const x = 1;");
    write_file(root, "apps/hybrid/content/post.md", "# Hello");
    write_file(
        root,
        "apps/hybrid/src/modules/domain/index.ts",
        "export type X = string;",
    );

    let (stdout, stderr) = run_ts_init_dry_run(root);

    // The auto_detect_app_type function checks hex arch FIRST (Signal 1),
    // then content/ dir (Signal 2). So hex arch should win → service.
    assert!(
        stdout.contains("type = \"service\""),
        "Hex arch signal should take priority over content/ dir.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 6: library type from config generates no stylelint
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn library_type_from_config_generates_no_stylelint() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // Config with ONLY a library type app
    write_file(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n\n",
            "[profile]\nname = \"service\"\n\n",
            "[typescript]\n\n",
            "[typescript.apps.utils]\ntype = \"library\"\n",
        ),
    );

    let (stdout, stderr) = run_ts_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Library app should NOT generate .stylelintrc.mjs (that's content-only)
    assert!(
        !combined.contains(".stylelintrc.mjs"),
        "Library type app should not generate .stylelintrc.mjs.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 7: mixed types generate correct configs
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn mixed_types_generate_correct_configs() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_root_package_json(root);

    // Create app directories so discovery works
    write_file(
        root,
        "apps/web/package.json",
        r#"{"name": "web", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/web/src/index.ts", "export const x = 1;");
    write_file(
        root,
        "apps/api/package.json",
        r#"{"name": "api", "version": "0.1.0"}"#,
    );
    write_file(root, "apps/api/src/index.ts", "export const x = 1;");

    // Config with content + service apps
    write_file(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n\n",
            "[profile]\nname = \"service\"\n\n",
            "[typescript]\n\n",
            "[typescript.apps.web]\ntype = \"content\"\n\n",
            "[typescript.apps.api]\ntype = \"service\"\n",
        ),
    );

    // Run ts generate (not dry-run) to actually write files
    let path_str = root.to_str().expect("non-utf8 path"); // reason: test setup
    let out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("failed to run guardrail3"); // reason: test infra

    assert!(
        out.status.success(),
        "ts generate should succeed.\nstderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Read generated eslint.config.mjs
    let eslint_path = root.join("eslint.config.mjs");
    let eslint_content =
        std::fs::read_to_string(&eslint_path).expect("read generated eslint.config.mjs"); // reason: test assertion

    // Content app → eslint should have jsx-a11y plugin (accessibility)
    assert!(
        eslint_content.contains("jsx-a11y"),
        "Mixed monorepo with content app should generate eslint config with jsx-a11y.\nContent: {eslint_content}"
    );

    // Having a content app should also generate .stylelintrc.mjs
    assert!(
        root.join(".stylelintrc.mjs").exists(),
        "Content app in monorepo should trigger .stylelintrc.mjs generation"
    );
}
