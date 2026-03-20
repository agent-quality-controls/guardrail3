//! Adversarial integration tests for app name → path resolution in RS generate.
//!
//! These tests verify:
//! - Suffix-matching does not confuse similarly-named apps
//! - Phantom apps (not in workspace) fall back to name-as-path
//! - Nested workspaces resolve correctly
//! - Single-crate projects generate at root
//! - Packages-only configs generate library profile at root
//! - Apps + packages generate both per-app and root configs

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

/// Minimal workspace Cargo.toml template.
fn workspace_toml(members: &[&str]) -> String {
    let members_str: Vec<String> = members.iter().map(|m| format!("    \"{m}\"")).collect();
    format!(
        "[workspace]\nmembers = [\n{}\n]\nresolver = \"2\"\n",
        members_str.join(",\n")
    )
}

/// Minimal crate Cargo.toml template.
fn crate_toml(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n")
}

// ---------------------------------------------------------------------------
// Test 1: suffix_match_does_not_confuse_apps
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn suffix_match_does_not_confuse_apps() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    // Create workspace with two apps that share a suffix
    write_fixture(root, "Cargo.toml", &workspace_toml(&["apps/*"]));
    write_fixture(root, "apps/validator/Cargo.toml", &crate_toml("validator"));
    write_fixture(
        root,
        "apps/my-validator/Cargo.toml",
        &crate_toml("my-validator"),
    );

    // Config references only "validator"
    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.apps.validator]\n",
            "type = \"service\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    // Should reference apps/validator/clippy.toml
    assert!(
        stdout.contains("apps/validator/clippy.toml"),
        "Output should reference apps/validator/clippy.toml, got:\n{stdout}"
    );

    // Should NOT reference apps/my-validator/clippy.toml
    assert!(
        !stdout.contains("apps/my-validator/clippy.toml"),
        "Output should NOT reference apps/my-validator/clippy.toml (suffix confusion), got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: app_not_in_workspace_uses_name_as_fallback
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn app_not_in_workspace_uses_name_as_fallback() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(root, "Cargo.toml", &workspace_toml(&["apps/*"]));
    write_fixture(root, "apps/real-app/Cargo.toml", &crate_toml("real-app"));

    // Config references "phantom" which does NOT exist in workspace
    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.apps.phantom]\n",
            "type = \"service\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    // Phantom app should fall back to using its name as the path
    assert!(
        stdout.contains("phantom/clippy.toml"),
        "Phantom app should fall back to name as path (phantom/clippy.toml), got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: nested_workspace_app_resolves_correctly
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn nested_workspace_app_resolves_correctly() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    // Root workspace with packages/*, excluding apps/*
    write_fixture(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/*\"]\nexclude = [\"apps/*\"]\nresolver = \"2\"\n",
    );
    write_fixture(root, "packages/utils/Cargo.toml", &crate_toml("utils"));

    // Nested workspace at apps/shedul3r/
    write_fixture(
        root,
        "apps/shedul3r/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    write_fixture(
        root,
        "apps/shedul3r/crates/core/Cargo.toml",
        &crate_toml("shedul3r-core"),
    );

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.apps.shedul3r]\n",
            "type = \"service\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(
        stdout.contains("apps/shedul3r/clippy.toml"),
        "Nested workspace app should resolve to apps/shedul3r/clippy.toml, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: single_crate_project_generates_at_root
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn single_crate_project_generates_at_root() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    // Single crate: [package] but no [workspace]
    write_fixture(root, "Cargo.toml", &crate_toml("my-tool"));

    // No [rust.apps.*], just workspace_root
    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    // Should generate clippy.toml at root (no prefix)
    assert!(
        stdout.contains("clippy.toml"),
        "Single crate should generate clippy.toml at root, got:\n{stdout}"
    );

    // Should NOT have any "apps/" prefix
    assert!(
        !stdout.contains("apps/"),
        "Single crate should not have apps/ prefix in paths, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: packages_only_generates_library_profile_at_root
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn packages_only_generates_library_profile_at_root() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(root, "Cargo.toml", &workspace_toml(&["packages/*"]));
    write_fixture(root, "packages/utils/Cargo.toml", &crate_toml("utils"));

    // packages config with library type, no apps
    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.packages]\n",
            "type = \"library\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    // Should reference root clippy.toml (no app prefix)
    assert!(
        stdout.contains("clippy.toml"),
        "Packages-only should generate clippy.toml at root, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 6: app_plus_packages_generates_both
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn app_plus_packages_generates_both() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let root = tmp.path();

    write_fixture(
        root,
        "Cargo.toml",
        &workspace_toml(&["apps/api/crates/*", "packages/*"]),
    );
    write_fixture(
        root,
        "apps/api/crates/domain/Cargo.toml",
        &crate_toml("api-domain"),
    );
    write_fixture(root, "packages/utils/Cargo.toml", &crate_toml("utils"));

    write_fixture(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.apps.api]\n",
            "type = \"service\"\n",
            "\n",
            "[rust.packages]\n",
            "type = \"library\"\n",
        ),
    );

    let path_str = root.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run dry-run");

    let stdout = String::from_utf8_lossy(&out.stdout);

    // Should have per-app clippy.toml
    assert!(
        stdout.contains("api") && stdout.contains("clippy.toml"),
        "Should generate app-level clippy.toml for api, got:\n{stdout}"
    );

    // Count clippy.toml occurrences — should be at least 2 (app + root)
    let clippy_count = stdout.matches("clippy.toml").count();
    assert!(
        clippy_count >= 2,
        "Should generate at least 2 clippy.toml files (app + root packages), found {clippy_count} in:\n{stdout}"
    );
}
