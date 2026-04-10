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

fn run_ast_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    inputs.iter().flat_map(g3rs_test_source_checks::check).collect()
}

fn findings_by_file(results: &[G3CheckResult]) -> BTreeMap<String, Vec<&G3CheckResult>> {
    let mut by_file = BTreeMap::<String, Vec<&G3CheckResult>>::new();
    for result in results {
        let key = result.file().unwrap_or("<none>").to_owned();
        by_file.entry(key).or_default().push(result);
    }
    by_file
}

#[test]
fn pipeline_reports_simple_test_ast_findings() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("src/lib.rs"),
        "#[cfg(test)]\nmod tests { #[test] fn inner() { assert_eq!(1, 1); } }\n",
    );
    write(
        root.join("tests/quality.rs"),
        "#[test]\n#[ignore]\nfn ignored() {}\n\n#[test]\n#[should_panic]\nfn panicy() { panic!(\"boom\"); }\n\n#[test]\nfn weak() { assert!(matches!(Some(1), Some(_))); }\n",
    );

    let results = run_ast_pipeline(root);
    let by_file = findings_by_file(&results);

    assert!(
        by_file["src/lib.rs"]
            .iter()
            .any(|result| result.id() == "RS-TEST-01"),
        "{results:#?}"
    );
    assert!(
        by_file["tests/quality.rs"]
            .iter()
            .any(|result| result.id() == "RS-TEST-04"),
        "{results:#?}"
    );
    assert!(
        by_file["tests/quality.rs"]
            .iter()
            .any(|result| result.id() == "RS-TEST-05"),
        "{results:#?}"
    );
    assert!(
        by_file["tests/quality.rs"]
            .iter()
            .any(|result| result.id() == "RS-TEST-08"),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_assertions_boundary_rules() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/runtime/src/lib.rs"), "pub fn smoke() {}\n");
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_demo() { assert_eq!(1, 1); }\n",
    );
    write(
        root.join("crates/runtime/tests/api.rs"),
        "use demo_assertions::assert_demo;\n#[test]\nfn api() { assert_demo(); }\n",
    );
    write(
        root.join("crates/runtime/src/feature_tests/mod.rs"),
        "#[test]\nfn sidecar() { let result = CheckResult::new(String::new(), Severity::Info, String::new(), String::new(), None, None); assert_eq!(result.id(), \"\"); }\n",
    );

    let results = run_ast_pipeline(root);
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-16"
                && result.file() == Some("crates/assertions/src/lib.rs")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-17"
                && result.file() == Some("crates/runtime/tests/api.rs")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-16"
                && result.file() == Some("crates/runtime/src/feature_tests/mod.rs")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_owned_source_as_rs_test_10() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("src/lib.rs"), "pub fn ok() {}\n");
    write(root.join("tests/broken.rs"), "#[test]\nfn broken( {\n");

    let results = run_ast_pipeline(root);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-TEST-10"
                && result.file() == Some("tests/broken.rs")
                && result.title() == "failed to read test input"
        }),
        "{results:#?}"
    );
}

#[test]
fn ingest_for_source_checks_classifies_root_files_by_role() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/runtime\", \"crates/assertions\"]\nresolver = \"2\"\n",
    );
    write(
        root.join("crates/runtime/Cargo.toml"),
        "[package]\nname = \"demo-runtime\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join("crates/runtime/src/lib.rs"), "pub fn smoke() {}\n");
    write(
        root.join("crates/runtime/src/feature_tests/mod.rs"),
        "#[test]\nfn sidecar() {}\n",
    );
    write(
        root.join("crates/runtime/src/feature_tests/helper.rs"),
        "pub fn helper() {}\n",
    );
    write(
        root.join("crates/runtime/src/feature_tests/fixtures/skip.rs"),
        "pub fn fixture() {}\n",
    );
    write(
        root.join("crates/runtime/tests/api.rs"),
        "#[test]\nfn api() {}\n",
    );
    write(
        root.join("crates/assertions/Cargo.toml"),
        "[package]\nname = \"demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("crates/assertions/src/lib.rs"),
        "pub fn assert_demo() { assert_eq!(1, 1); }\n",
    );
    write(
        root.join("crates/assertions/src/fixtures/skip.rs"),
        "pub fn fixture() {}\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

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
        file_kinds.get("crates/runtime/src/feature_tests/mod.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::InternalSidecarMod)
    );
    assert_eq!(
        file_kinds.get("crates/runtime/src/feature_tests/helper.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::InternalSidecarSupport)
    );
    assert_eq!(
        file_kinds.get("crates/runtime/tests/api.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::ExternalHarness)
    );
    assert_eq!(
        file_kinds.get("crates/assertions/src/lib.rs"),
        Some(&g3rs_test_types::G3RsTestFileKind::AssertionsModule)
    );
    assert!(!file_kinds.contains_key("crates/runtime/src/feature_tests/fixtures/skip.rs"));
    assert!(!file_kinds.contains_key("crates/assertions/src/fixtures/skip.rs"));
}
