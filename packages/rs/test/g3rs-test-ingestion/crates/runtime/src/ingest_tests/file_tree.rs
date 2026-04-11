use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_file_tree_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_test_file_tree_checks::check)
        .collect()
}

#[test]
fn ingest_for_file_tree_checks_classifies_structural_file_roles() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\", \"test_support\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\ntest_support = { path = \"../../test_support\" }\n",
    );
    write(
        root.join("crates/runtime/src/lib.rs"),
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n",
    );
    write(
        root.join("crates/runtime/src/lib_tests/mod.rs"),
        "#[test]\nfn owned() { assert!(true); }\n",
    );
    write(
        root.join("crates/runtime/tests/public_surface.rs"),
        "#[test]\nfn public_surface() { assert!(true); }\n",
    );
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write(root.join("crates/assertions/src/lib.rs"), "pub fn assert_runtime() {}\n");
    write(
        root.join("test_support/Cargo.toml"),
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("test_support/src/lib.rs"), "pub fn helper() {}\n");
    write(root.join("src/lib_tests/mod.rs"), "#[test]\nfn stray() { assert!(true); }\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed");

    assert_eq!(inputs.len(), 1, "{inputs:#?}");
    let input = &inputs[0];
    let file_kinds = input
        .files
        .iter()
        .map(|file| (file.rel_path.as_str(), file.kind))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(
        file_kinds.get("crates/runtime/src/lib.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::Source)
    );
    assert_eq!(
        file_kinds.get("crates/runtime/src/lib_tests/mod.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::InternalSidecarMod)
    );
    assert_eq!(
        file_kinds.get("crates/runtime/tests/public_surface.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::ExternalHarness)
    );
    assert_eq!(
        file_kinds.get("crates/assertions/src/lib.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::AssertionsModule)
    );
    assert_eq!(
        file_kinds.get("test_support/src/lib.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::TestSupport)
    );
    assert_eq!(
        file_kinds.get("src/lib_tests/mod.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::InternalSidecarMod)
    );
}

#[test]
fn file_tree_pipeline_reports_structural_test_findings() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\", \"test_support\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/runtime/src/lib.rs"), "pub fn value() -> u8 { 1 }\n");
    write(
        root.join("crates/runtime/tests/public_surface.rs"),
        "use crate::value;\n#[test]\nfn public_surface() { assert_eq!(value(), 1); }\n",
    );
    write(
        root.join("test_support/Cargo.toml"),
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("test_support/src/lib.rs"),
        "use demo_runtime::value;\npub fn fixture_value() -> u8 { value() }\n",
    );
    write(root.join("src/tests/helper.rs"), "#[test]\nfn stray() { assert!(true); }\n");

    let results = run_file_tree_pipeline(root);

    assert!(results.iter().any(|result| {
        result.id() == "RS-TEST-02" && result.file() == Some("src/tests")
    }), "{results:#?}");
    assert!(results.iter().any(|result| {
        result.id() == "RS-TEST-03"
            && result.title() == "assertions crate missing"
            && result.file() == Some("crates/assertions/Cargo.toml")
    }), "{results:#?}");
    assert!(results.iter().any(|result| {
        result.id() == "RS-TEST-18"
            && result.title() == "test_support imports local component crate"
            && result.file() == Some("test_support/src/lib.rs")
    }), "{results:#?}");
}

#[test]
fn file_tree_pipeline_reports_input_failures_as_rs_test_10() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/runtime/src/lib.rs"), "pub fn value() -> u8 { 1 }\n");
    write(
        root.join("crates/runtime/tests/public_surface.rs"),
        "#[test]\nfn public_surface() { assert_eq!(crate::value(), 1); }\n",
    );
    let unreadable = root.join("crates/runtime/tests/broken.rs");
    write(&unreadable, "#[test]\nfn broken() { assert!(true); }\n");
    let mut permissions = fs::metadata(&unreadable)
        .expect("metadata")
        .permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.set_mode(0o000);
        fs::set_permissions(&unreadable, permissions).expect("set unreadable permissions");
    }

    let results = run_file_tree_pipeline(root);

    #[cfg(unix)]
    {
        let mut restore = fs::metadata(&unreadable)
            .expect("metadata")
            .permissions();
        use std::os::unix::fs::PermissionsExt;
        restore.set_mode(0o644);
        fs::set_permissions(&unreadable, restore).expect("restore permissions");
    }

    assert!(results.iter().any(|result| {
        result.id() == "RS-TEST-10"
            && result.file() == Some("crates/runtime/tests/broken.rs")
            && result.title() == "failed to read test input"
    }), "{results:#?}");
}

#[test]
fn file_tree_pipeline_reports_nested_ad_hoc_src_tests_tree() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dev-dependencies]\ndemo_assertions = { path = \"../assertions\" }\n",
    );
    write(root.join("crates/runtime/src/lib.rs"), "pub mod foo;\n");
    write(root.join("crates/runtime/src/foo.rs"), "pub fn value() -> u8 { 1 }\n");
    write(
        root.join("crates/runtime/src/foo/tests/helper.rs"),
        "#[test]\nfn stray() { assert!(true); }\n",
    );
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write(root.join("crates/assertions/src/lib.rs"), "pub fn assert_runtime() {}\n");

    let results = run_file_tree_pipeline(root);

    assert!(results.iter().any(|result| {
        result.id() == "RS-TEST-02"
            && result.title() == "ad hoc src/tests tree"
            && result.file() == Some("crates/runtime/src/foo/tests")
    }), "{results:#?}");
}

#[test]
fn file_tree_pipeline_skips_inactive_root_with_only_test_support() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"test_support\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("test_support/Cargo.toml"),
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("test_support/src/lib.rs"),
        "pub fn fixture_name(name: &str) -> String { name.to_owned() }\n",
    );

    let results = run_file_tree_pipeline(root);

    assert!(results.is_empty(), "{results:#?}");
}
