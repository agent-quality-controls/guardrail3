use guardrail3_adapters_outbound_fs::RealFileSystem;

use super::detect_project;

fn reset_test_root(path: &std::path::Path) {
    let _ = guardrail3_shared_fs::remove_dir_all(path);
    let _ = guardrail3_shared_fs::create_dir_all(path);
}

fn write_test_file(root: &std::path::Path, rel: &str, contents: &str) {
    let full_path = root.join(rel);
    let write_result = guardrail3_shared_fs::write_file(&full_path, contents);
    assert!(
        write_result.is_ok(),
        "test setup should write {}: {write_result:?}",
        full_path.display()
    );
}

#[test]
fn detects_workspace_in_apps_backend() {
    let fs = RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_test_monorepo");
    reset_test_root(&tmp);

    write_test_file(&tmp, "Cargo.toml", "[workspace]\nmembers = []\n");
    write_test_file(
        &tmp,
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    write_test_file(
        &tmp,
        "apps/backend/crates/api/Cargo.toml",
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_test_file(
        &tmp,
        "apps/backend/crates/api/src/main.rs",
        "fn main() {}\n",
    );
    write_test_file(&tmp, "package.json", "{}");
    write_test_file(&tmp, "tsconfig.json", "{}");

    let project = detect_project(&fs, &tmp);

    assert!(project.has_rust, "Should detect Rust");
    assert!(project.has_typescript, "Should detect TypeScript");

    let Some(workspace_root) = project.primary_workspace_root() else {
        panic!("should have workspace root");
    };
    assert!(
        workspace_root.ends_with("apps/backend"),
        "workspace root should be apps/backend, got {workspace_root:?}"
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

    let _ = guardrail3_shared_fs::remove_dir_all(&tmp);
}

#[test]
fn direct_workspace_detected_at_root() {
    let fs = RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_test_direct_ws");
    reset_test_root(&tmp);

    write_test_file(
        &tmp,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crates/core\"]\n",
    );
    write_test_file(
        &tmp,
        "crates/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_test_file(&tmp, "crates/core/src/lib.rs", "");

    let project = detect_project(&fs, &tmp);
    assert!(project.has_rust);

    let Some(workspace_root) = project.primary_workspace_root() else {
        panic!("should have workspace root");
    };
    assert_eq!(
        workspace_root,
        tmp.as_path(),
        "Direct workspace should have root as workspace root"
    );

    let _ = guardrail3_shared_fs::remove_dir_all(&tmp);
}
