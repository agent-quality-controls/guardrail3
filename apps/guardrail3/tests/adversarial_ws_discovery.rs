//! Adversarial integration tests for Rust workspace discovery edge cases.
//!
//! These tests exercise `rs generate --dry-run` with unusual Cargo.toml workspace
//! configurations: empty members, globs matching nothing, nested workspaces,
//! workspace excludes, single crates, and virtual workspaces.

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3 as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use serde_json as _;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

use std::process::Command;

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

/// Helper: run `rs generate --dry-run` and return `(stdout, stderr, exit_code)`.
#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics on command failure
fn run_rs_generate_dry_run(path: &std::path::Path) -> (String, String, i32) {
    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run guardrail3"); // reason: test infra

    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let code = out.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

// ============================================================
// Test 1: empty workspace members generates at root
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn empty_workspace_members_generates_at_root() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write_file(
        root,
        "guardrail3.toml",
        "version = \"0.1\"\n\n[profile]\nname = \"service\"\n\n[rust]\nworkspace_root = \".\"\n",
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Should produce root-level config files (clippy.toml, deny.toml)
    assert!(
        combined.contains("clippy.toml"),
        "Should mention clippy.toml in dry-run output.\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
        combined.contains("deny.toml"),
        "Should mention deny.toml in dry-run output.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 2: glob members matching nothing
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn glob_members_matching_nothing() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    // Create empty crates/ directory (no Cargo.toml inside)
    std::fs::create_dir_all(root.join("crates")).expect("create crates dir"); // reason: test setup

    write_file(
        root,
        "guardrail3.toml",
        "version = \"0.1\"\n\n[profile]\nname = \"service\"\n\n[rust]\nworkspace_root = \".\"\n",
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Should not crash — should still produce root-level configs
    assert!(
        combined.contains("clippy.toml"),
        "Should not crash and should mention clippy.toml.\nstdout: {stdout}\nstderr: {stderr}"
    );
    // Should not contain panic or error about missing members
    assert!(
        !combined.contains("panic"),
        "Should not panic with empty glob match.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 3: multiple nested workspaces
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn multiple_nested_workspaces() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    // Root Cargo.toml with packages/* members, excluding apps/*
    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/*\"]\nexclude = [\"apps/*\"]\nresolver = \"2\"\n",
    );

    // Nested workspace: apps/svc-a/
    write_file(
        root,
        "apps/svc-a/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    write_file(
        root,
        "apps/svc-a/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    // Nested workspace: apps/svc-b/
    write_file(
        root,
        "apps/svc-b/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
    );
    write_file(
        root,
        "apps/svc-b/crates/api/Cargo.toml",
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    // Config referencing both apps
    write_file(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n\n",
            "[profile]\nname = \"service\"\n\n",
            "[rust]\nworkspace_root = \".\"\n\n",
            "[rust.apps.svc-a]\ntype = \"service\"\n\n",
            "[rust.apps.svc-b]\ntype = \"service\"\n",
        ),
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Both nested workspace apps should appear in output
    assert!(
        combined.contains("clippy.toml"),
        "Should generate clippy.toml.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Verify no crash (valid output)
    assert!(
        !combined.contains("panic"),
        "Should not panic with nested workspaces.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Both should produce output without collisions
    assert!(
        !combined.contains("Error"),
        "Should not produce errors.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 4: workspace exclude skips app
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn workspace_exclude_skips_app() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    // Root workspace with exclude
    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"apps/*\"]\nexclude = [\"apps/legacy\"]\nresolver = \"2\"\n",
    );

    // Active app
    write_file(
        root,
        "apps/active/Cargo.toml",
        "[package]\nname = \"active\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    // Legacy app (excluded from workspace)
    write_file(
        root,
        "apps/legacy/Cargo.toml",
        "[package]\nname = \"legacy\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    // Config references both
    write_file(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n\n",
            "[profile]\nname = \"service\"\n\n",
            "[rust]\nworkspace_root = \".\"\n\n",
            "[rust.apps.legacy]\ntype = \"service\"\n\n",
            "[rust.apps.active]\ntype = \"service\"\n",
        ),
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // active should appear in generated configs
    assert!(
        combined.contains("clippy.toml"),
        "Should generate clippy.toml for active app.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Should not crash
    assert!(
        !combined.contains("panic"),
        "Should not panic with excluded workspace members.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 5: single crate, no workspace
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn single_crate_no_workspace() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    // Package only — no [workspace] section
    write_file(
        root,
        "Cargo.toml",
        "[package]\nname = \"solo-crate\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    write_file(
        root,
        "guardrail3.toml",
        "version = \"0.1\"\n\n[profile]\nname = \"service\"\n\n[rust]\nworkspace_root = \".\"\n",
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Should generate root-level configs
    assert!(
        combined.contains("clippy.toml"),
        "Should generate clippy.toml at root.\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
        combined.contains("deny.toml"),
        "Should generate deny.toml at root.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Should not crash or error
    assert!(
        !combined.contains("Error"),
        "Should not produce errors for single-crate project.\nstdout: {stdout}\nstderr: {stderr}"
    );
}

// ============================================================
// Test 6: virtual workspace with packages only
// ============================================================

#[test]
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs to set up fixtures
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
fn virtual_workspace_with_packages_only() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let root = tmp.path();

    // Virtual workspace — no [package], just [workspace]
    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/*\"]\nresolver = \"2\"\n",
    );

    // Package members
    write_file(
        root,
        "packages/utils/Cargo.toml",
        "[package]\nname = \"utils\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "packages/types/Cargo.toml",
        "[package]\nname = \"types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    // Config with packages as library type, no apps
    write_file(
        root,
        "guardrail3.toml",
        concat!(
            "version = \"0.1\"\n\n",
            "[profile]\nname = \"library\"\n\n",
            "[rust]\nworkspace_root = \".\"\n\n",
            "[rust.packages]\ntype = \"library\"\n",
        ),
    );

    let (stdout, stderr, _code) = run_rs_generate_dry_run(root);
    let combined = format!("{stdout}{stderr}");

    // Should generate root-level clippy.toml with library profile
    assert!(
        combined.contains("clippy.toml"),
        "Should generate clippy.toml for virtual workspace.\nstdout: {stdout}\nstderr: {stderr}"
    );

    // Should not crash
    assert!(
        !combined.contains("panic"),
        "Should not panic with virtual workspace.\nstdout: {stdout}\nstderr: {stderr}"
    );
}
