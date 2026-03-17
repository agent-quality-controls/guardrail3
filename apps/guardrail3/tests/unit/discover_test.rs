use std::fs as stdfs;

use guardrail3::adapters::outbound::fs::RealFileSystem;
use guardrail3::app::discover::*;

// ---- Bug 8: Monorepo workspace detection ----

#[test]
#[allow(clippy::expect_used)] // reason: test setup uses expect for clarity
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
#[allow(clippy::uninlined_format_args)] // reason: assert! macros with format args
fn detects_workspace_in_apps_backend() {
    let fs = RealFileSystem;
    // Create a temp directory mimicking a monorepo with marker Cargo.toml at root
    // and real workspace at apps/backend/
    let tmp = std::env::temp_dir().join("guardrail3_test_monorepo");
    let _ = stdfs::remove_dir_all(&tmp);
    let _ = stdfs::create_dir_all(tmp.join("apps/backend/crates/api/src"));

    // Marker Cargo.toml at root (no workspace members)
    let _ = stdfs::write(tmp.join("Cargo.toml"), "[workspace]\nmembers = []\n");

    // Real workspace at apps/backend/
    let _ = stdfs::write(
        tmp.join("apps/backend/Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );

    // Crate Cargo.toml
    let _ = stdfs::write(
        tmp.join("apps/backend/crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let _ = stdfs::write(
        tmp.join("apps/backend/crates/api/src/main.rs"),
        "fn main() {}\n",
    );

    // Also add a package.json so TypeScript is detected
    let _ = stdfs::write(tmp.join("package.json"), "{}");

    let project = detect_project(&fs, &tmp);

    assert!(project.has_rust, "Should detect Rust");
    assert!(project.has_typescript, "Should detect TypeScript");

    let workspace_root = project
        .primary_workspace_root()
        .expect("Should have workspace root");
    assert!(
        workspace_root.ends_with("apps/backend"),
        "Workspace root should be apps/backend, got {:?}",
        workspace_root
    );
    let member_names = project.all_member_names();
    assert!(
        !member_names.is_empty(),
        "Should have detected workspace members"
    );
    assert!(
        member_names.contains(&"api".to_owned()),
        "Should find 'api' crate, got {:?}",
        member_names
    );

    // Cleanup
    let _ = stdfs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::expect_used)] // reason: test setup uses expect for clarity
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn direct_workspace_detected_at_root() {
    let fs = RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_test_direct_ws");
    let _ = stdfs::remove_dir_all(&tmp);
    let _ = stdfs::create_dir_all(tmp.join("crates/core/src"));

    let _ = stdfs::write(
        tmp.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\n",
    );
    let _ = stdfs::write(
        tmp.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    let _ = stdfs::write(tmp.join("crates/core/src/lib.rs"), "");

    let project = detect_project(&fs, &tmp);
    assert!(project.has_rust);

    let workspace_root = project
        .primary_workspace_root()
        .expect("Should have workspace root");
    // Should be the root itself, not apps/backend
    assert_eq!(
        workspace_root,
        tmp.as_path(),
        "Direct workspace should have root as workspace root"
    );

    let _ = stdfs::remove_dir_all(&tmp);
}
