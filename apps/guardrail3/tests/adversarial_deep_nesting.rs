//! Adversarial integration tests for deeply nested workspace structures
//! and path resolution.
//!
//! These tests verify:
//! - Apps in `apps/` subdirectories resolve to correct paths
//! - Root vs app clippy.toml use correct profiles (library vs service)
//! - deny.toml generated at all workspace roots
//! - rust-toolchain.toml only at project root
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
use semver as _;
use serde as _;
use serde_json as _;
use serde_yaml as _;
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

/// Set up a deeply nested workspace structure with two nested workspaces
/// under `apps/` and a `packages/` directory at root.
#[allow(clippy::disallowed_methods)] // reason: test helper -- writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper -- panics on write failure
fn setup_deep_nesting(dir: &std::path::Path) {
    // Root Cargo.toml: workspace with members and excludes
    std::fs::write(
        dir.join("Cargo.toml"),
        concat!(
            "[workspace]\n",
            "members = [\"packages/*\"]\n",
            "exclude = [\"apps/platform\", \"apps/tools\"]\n",
            "resolver = \"2\"\n",
        ),
    )
    .expect("write root Cargo.toml");

    // guardrail3.toml with apps and packages config
    std::fs::write(
        dir.join("guardrail3.toml"),
        concat!(
            "version = \"0.1\"\n",
            "\n",
            "[profile]\n",
            "name = \"service\"\n",
            "\n",
            "[rust]\n",
            "workspace_root = \".\"\n",
            "\n",
            "[rust.apps.platform]\n",
            "type = \"service\"\n",
            "\n",
            "[rust.apps.platform.checks]\n",
            "hexarch = true\n",
            "garde = true\n",
            "test = true\n",
            "release = true\n",
            "\n",
            "[rust.apps.tools]\n",
            "type = \"service\"\n",
            "\n",
            "[rust.packages]\n",
            "type = \"library\"\n",
            "\n",
            "[rust.packages.checks]\n",
            "hexarch = false\n",
            "garde = false\n",
            "test = true\n",
            "release = false\n",
        ),
    )
    .expect("write guardrail3.toml");

    // packages/types/Cargo.toml
    std::fs::create_dir_all(dir.join("packages/types")).expect("create packages/types");
    std::fs::write(
        dir.join("packages/types/Cargo.toml"),
        "[package]\nname = \"types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write packages/types/Cargo.toml");

    // apps/platform nested workspace
    let platform = dir.join("apps/platform");
    std::fs::create_dir_all(&platform).expect("create apps/platform");
    std::fs::write(
        platform.join("Cargo.toml"),
        concat!(
            "[workspace]\n",
            "members = [\"crates/*\"]\n",
            "resolver = \"2\"\n",
        ),
    )
    .expect("write apps/platform/Cargo.toml");

    for (name, pkg) in [
        ("core", "platform-core"),
        ("web", "platform-web"),
        ("cli", "platform-cli"),
    ] {
        let crate_dir = platform.join(format!("crates/{name}"));
        std::fs::create_dir_all(&crate_dir).expect("create platform crate dir");
        std::fs::write(
            crate_dir.join("Cargo.toml"),
            format!("[package]\nname = \"{pkg}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
        )
        .expect("write platform crate Cargo.toml");
    }

    // apps/tools nested workspace
    let tools = dir.join("apps/tools");
    std::fs::create_dir_all(&tools).expect("create apps/tools");
    std::fs::write(
        tools.join("Cargo.toml"),
        concat!(
            "[workspace]\n",
            "members = [\"crates/*\"]\n",
            "resolver = \"2\"\n",
        ),
    )
    .expect("write apps/tools/Cargo.toml");

    let migrator = tools.join("crates/migrator");
    std::fs::create_dir_all(&migrator).expect("create migrator dir");
    std::fs::write(
        migrator.join("Cargo.toml"),
        "[package]\nname = \"migrator\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write migrator Cargo.toml");
}

// ---------------------------------------------------------------------------
// Test 1: platform app resolves to apps/platform (not just "platform")
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn platform_resolves_to_apps_platform() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("apps/platform/clippy.toml"),
        "Expected 'apps/platform/clippy.toml' in diff output, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: tools app resolves to apps/tools
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn tools_resolves_to_apps_tools() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("apps/tools/clippy.toml"),
        "Expected 'apps/tools/clippy.toml' in diff output, got:\n{stdout}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: root clippy.toml uses library profile (from [rust.packages] type)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn root_clippy_uses_library_profile() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Root clippy.toml should use library profile (is_pure=true for library)
    // which includes global-state bans (LazyLock, OnceLock)
    let clippy = std::fs::read_to_string(dir.join("clippy.toml")).expect("read root clippy.toml");
    assert!(
        clippy.contains("LazyLock"),
        "Root clippy.toml (library profile) should contain LazyLock ban, got:\n{clippy}"
    );
    assert!(
        clippy.contains("OnceLock"),
        "Root clippy.toml (library profile) should contain OnceLock ban, got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 4: app clippy.toml uses service profile (is_pure=false, no global-state)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn app_clippy_uses_service_profile() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Service profile with is_pure=false should NOT include global-state bans
    let clippy = std::fs::read_to_string(dir.join("apps/platform/clippy.toml"))
        .expect("read platform clippy.toml");
    assert!(
        !clippy.contains("LazyLock"),
        "Service profile clippy.toml should NOT contain LazyLock ban (is_pure=false), got:\n{clippy}"
    );
}

// ---------------------------------------------------------------------------
// Test 5: deny.toml generated at root and both app workspaces
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn three_deny_toml_files_generated() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        dir.join("deny.toml").exists(),
        "Root deny.toml should exist"
    );
    assert!(
        dir.join("apps/platform/deny.toml").exists(),
        "apps/platform/deny.toml should exist"
    );
    assert!(
        dir.join("apps/tools/deny.toml").exists(),
        "apps/tools/deny.toml should exist"
    );
}

// ---------------------------------------------------------------------------
// Test 6: rust-toolchain.toml at root only, NOT in app subdirectories
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn rust_toolchain_at_root_only() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");
    let out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    assert!(
        dir.join("rust-toolchain.toml").exists(),
        "rust-toolchain.toml should exist at root"
    );
    assert!(
        !dir.join("apps/platform/rust-toolchain.toml").exists(),
        "rust-toolchain.toml should NOT exist in apps/platform"
    );
    assert!(
        !dir.join("apps/tools/rust-toolchain.toml").exists(),
        "rust-toolchain.toml should NOT exist in apps/tools"
    );
}

// ---------------------------------------------------------------------------
// Test 7: generate is idempotent (second run shows no changes in diff)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn generate_idempotent() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let dir = tmp.path();
    setup_deep_nesting(dir);

    let path_str = dir.to_str().expect("non-utf8 path");

    // First generate
    let out1 = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run first generate");
    assert!(
        out1.status.success(),
        "first generate should succeed: {}",
        String::from_utf8_lossy(&out1.stderr)
    );

    // Second generate
    let out2 = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("failed to run second generate");
    assert!(
        out2.status.success(),
        "second generate should succeed: {}",
        String::from_utf8_lossy(&out2.stderr)
    );

    // Diff should show "No changes needed"
    let diff_out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("failed to run diff");

    let stdout = String::from_utf8_lossy(&diff_out.stdout);
    assert!(
        stdout.contains("No changes needed"),
        "After two generates, diff should say 'No changes needed', got:\n{stdout}"
    );
}
