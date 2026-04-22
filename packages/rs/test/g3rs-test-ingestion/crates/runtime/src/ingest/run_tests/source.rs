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

fn run_ast_pipeline(root: &Path) -> Vec<guardrail3_check_types::G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        super::super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_test_source_checks::check)
        .collect()
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

    assert_file_has_result(&results, "src/lib.rs", "RS-TEST-SOURCE-01");
    assert_file_has_result(&results, "tests/quality.rs", "RS-TEST-SOURCE-04");
    assert_file_has_result(&results, "tests/quality.rs", "RS-TEST-SOURCE-05");
    assert_file_has_result(&results, "tests/quality.rs", "RS-TEST-SOURCE-08");
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
    write(
        root.join("crates/runtime/src/lib.rs"),
        "pub fn smoke() {}\n",
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
        root.join("crates/runtime/tests/api.rs"),
        "use demo_assertions::assert_demo;\n#[test]\nfn api() { assert_demo(); }\n",
    );
    write(
        root.join("crates/runtime/src/feature_tests/mod.rs"),
        "#[test]\nfn sidecar() { let result = CheckResult::new(String::new(), Severity::Info, String::new(), String::new(), None, None); assert_eq!(result.id(), \"\"); }\n",
    );

    let results = run_ast_pipeline(root);
    assert_file_has_result(
        &results,
        "crates/assertions/src/lib.rs",
        "RS-TEST-SOURCE-16",
    );
    assert_file_has_result(&results, "crates/runtime/tests/api.rs", "RS-TEST-SOURCE-17");
    assert_file_has_result(
        &results,
        "crates/runtime/src/feature_tests/mod.rs",
        "RS-TEST-SOURCE-16",
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

    assert_result(
        &results,
        "RS-TEST-SOURCE-10",
        "failed to read test input",
        Some("tests/broken.rs"),
    );
}

#[test]
fn pipeline_reports_all_parse_failures_and_still_checks_valid_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("tests/good.rs"),
        "#[test]\n#[ignore] // reason: blocked on external service\nfn good() {}\n",
    );
    write(root.join("tests/broken_a.rs"), "#[test]\nfn broken_a( {\n");
    write(root.join("tests/broken_b.rs"), "#[test]\nfn broken_b( {\n");

    let results = run_ast_pipeline(root);

    assert_result(
        &results,
        "RS-TEST-SOURCE-10",
        "failed to read test input",
        Some("tests/broken_a.rs"),
    );
    assert_result(
        &results,
        "RS-TEST-SOURCE-10",
        "failed to read test input",
        Some("tests/broken_b.rs"),
    );
    assert_file_has_result(&results, "tests/good.rs", "RS-TEST-SOURCE-04");
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
    write(
        root.join("crates/runtime/src/lib.rs"),
        "pub fn smoke() {}\n",
    );
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
    let inputs =
        super::super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

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

#[test]
fn ingest_for_source_checks_expects_package_style_assertions_after_nested_fix_attempt() {
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
        "#[cfg(test)]\n#[path = \"lib_tests/mod.rs\"]\nmod lib_tests;\n",
    );
    write(
        root.join("packages/demo/crates/runtime/src/lib_tests/mod.rs"),
        "#[test]\nfn owned() { assert!(true); }\n",
    );
    write(
        root.join("packages/demo/assertions/Cargo.toml"),
        "[package]\nname = \"wrong-demo-assertions\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("packages/demo/assertions/src/lib.rs"),
        "pub fn assert_runtime() { assert_eq!(1, 1); }\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        super::super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");

    assert_eq!(inputs.len(), 1, "{inputs:#?}");
    let input = &inputs[0];
    let component = &input.components[0];
    assert_eq!(
        component.assertions_rel_dir,
        "packages/demo/crates/assertions"
    );
    assert!(!component.assertions_exists);
    assert!(
        input
            .files
            .iter()
            .all(|file| file.rel_path != "packages/demo/assertions/src/lib.rs"),
        "{input:#?}"
    );
}
