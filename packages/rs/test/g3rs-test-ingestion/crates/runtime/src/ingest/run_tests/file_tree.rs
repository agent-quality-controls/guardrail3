#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures must call std::fs and std::process::Command directly to seed and tear down filesystem state"
)]
#![expect(
    clippy::indexing_slicing,
    reason = "test fixtures index into known-shaped fixture data"
)]
#![expect(
    clippy::items_after_statements,
    reason = "test fixture inline helpers improve readability when colocated with the using test"
)]
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use g3rs_test_ingestion_assertions::ingest::run::{assert_file_has_result, assert_result};
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
        super::super::create_fixture_dir(parent).expect("create parent directory");
    }
    super::super::write_fixture(path.as_ref(), content).expect("write fixture file");
}

fn run_file_tree_pipeline(root: &Path) -> Vec<guardrail3_check_types::G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");
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
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_runtime() {}\n",
    );
    write(
        root.join("test_support/Cargo.toml"),
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("test_support/src/lib.rs"), "pub fn helper() {}\n");
    write(
        root.join("src/lib_tests/mod.rs"),
        "#[test]\nfn stray() { assert!(true); }\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

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
fn ingest_for_file_tree_checks_records_nested_assertions_manifest_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"packages/demo/crates/runtime\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("packages/demo/Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("packages/demo/crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("packages/demo/crates/runtime/src/lib.rs"),
        "pub fn value() -> u8 { 1 }\n",
    );
    write(
        root.join("packages/demo/assertions/Cargo.toml"),
        "[package]\nname = \"wrong-demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    assert_eq!(inputs.len(), 1, "{inputs:#?}");
    let component = &inputs[0].components[0];
    assert_eq!(
        component.assertions_rel_dir,
        "packages/demo/crates/assertions"
    );
    assert_eq!(
        component.assertions_cargo_rel_path,
        "packages/demo/crates/assertions/Cargo.toml"
    );
    assert!(!component.assertions_exists);
    assert_eq!(
        component.nested_assertions_cargo_rel_path.as_deref(),
        Some("packages/demo/assertions/Cargo.toml")
    );
}

#[test]
fn ingest_for_file_tree_checks_keeps_valid_analyzed_files_when_one_source_file_fails_to_parse() {
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
    write(
        root.join("crates/runtime/src/lib.rs"),
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n",
    );
    write(
        root.join("crates/runtime/src/lib_tests/mod.rs"),
        "#[test]\nfn owned() { assert!(true); }\n",
    );
    write(
        root.join("crates/runtime/src/broken.rs"),
        "pub fn broken( -> u8 { 1 }\n",
    );
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_runtime() {}\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    assert_eq!(inputs.len(), 1, "{inputs:#?}");
    let input = &inputs[0];
    assert!(input.has_tests, "{input:#?}");
    assert!(
        input
            .files
            .iter()
            .any(|file| file.rel_path == "crates/runtime/src/lib.rs")
    );
    assert!(
        input
            .files
            .iter()
            .any(|file| file.rel_path == "crates/runtime/src/lib_tests/mod.rs")
    );
    assert!(
        !input
            .files
            .iter()
            .any(|file| file.rel_path == "crates/runtime/src/broken.rs")
    );
    let rendered_failures = input
        .input_failures
        .iter()
        .map(|failure| format!("{failure:?}"))
        .collect::<Vec<_>>();
    assert!(rendered_failures.iter().any(|failure| {
        failure.contains("crates/runtime/src/broken.rs")
            && failure
                .contains("Failed to parse Rust source file for test-family file-tree analysis")
    }));
}

#[test]
fn ingest_for_file_tree_checks_preserves_calls_inside_macro_arguments() {
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
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_runtime() {}\n",
    );
    write(
        root.join("test_support/Cargo.toml"),
        "[package]\nname = \"test_support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("test_support/src/lib.rs"),
        "fn fixture_path() -> &'static str { \"fixtures/demo.json\" }\npub fn demo_fixture() -> Vec<&'static str> { vec![fixture_path()] }\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    let file = inputs[0]
        .files
        .iter()
        .find(|file| file.rel_path == "test_support/src/lib.rs")
        .expect("test_support analyzed file should exist");
    let function = file
        .parsed
        .functions
        .iter()
        .find(|function| function.name == "demo_fixture")
        .expect("demo_fixture should be analyzed");

    assert!(
        function
            .body
            .call_paths
            .iter()
            .any(|path| path == &["fixture_path".to_owned()]),
        "{function:#?}"
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
    write(
        root.join("crates/runtime/src/lib.rs"),
        "pub fn value() -> u8 { 1 }\n",
    );
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
    write(
        root.join("src/tests/helper.rs"),
        "#[test]\nfn stray() { assert!(true); }\n",
    );

    let results = run_file_tree_pipeline(root);

    assert_file_has_result(&results, "src/tests", "g3rs-test/owned-sidecar-shape");
    assert_result(
        &results,
        "g3rs-test/runtime-assertions-split",
        "assertions crate missing",
        Some("crates/assertions/Cargo.toml"),
    );
    assert_result(
        &results,
        "g3rs-test/test-support-generic",
        "test_support imports local component crate",
        Some("test_support/src/lib.rs"),
    );
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
    write(
        root.join("crates/runtime/src/lib.rs"),
        "pub fn value() -> u8 { 1 }\n",
    );
    write(
        root.join("crates/runtime/tests/public_surface.rs"),
        "#[test]\nfn public_surface() { assert_eq!(crate::value(), 1); }\n",
    );
    let unreadable = root.join("crates/runtime/tests/broken.rs");
    write(&unreadable, "#[test]\nfn broken() { assert!(true); }\n");
    let mut permissions = super::super::read_fixture_metadata(&unreadable)
        .expect("should read unreadable fixture metadata before chmod")
        .permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.set_mode(0o000);
        super::super::set_fixture_permissions(&unreadable, permissions)
            .expect("should mark fixture unreadable for file-tree ingestion");
    }

    let results = run_file_tree_pipeline(root);

    #[cfg(unix)]
    {
        let mut restore = super::super::read_fixture_metadata(&unreadable)
            .expect("should read unreadable fixture metadata before restore")
            .permissions();
        use std::os::unix::fs::PermissionsExt;
        restore.set_mode(0o644);
        super::super::set_fixture_permissions(&unreadable, restore)
            .expect("should restore unreadable fixture permissions after assertion");
    }

    assert_result(
        &results,
        "g3rs-test/filetree-input-failures",
        "failed to read test input",
        Some("crates/runtime/tests/broken.rs"),
    );
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
    write(
        root.join("crates/runtime/src/foo.rs"),
        "pub fn value() -> u8 { 1 }\n",
    );
    write(
        root.join("crates/runtime/src/foo/tests/helper.rs"),
        "#[test]\nfn stray() { assert!(true); }\n",
    );
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\ndemo_runtime = { path = \"../runtime\" }\n",
    );
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_runtime() {}\n",
    );

    let results = run_file_tree_pipeline(root);

    assert_result(
        &results,
        "g3rs-test/owned-sidecar-shape",
        "ad hoc src/tests tree",
        Some("crates/runtime/src/foo/tests"),
    );
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
