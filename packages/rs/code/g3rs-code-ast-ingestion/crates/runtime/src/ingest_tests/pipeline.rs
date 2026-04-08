use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_code_ast_checks_types::G3RsCodeAstChecksInput;
use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

const HAS_TODO: &str =
    include_str!("../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/has_todo.rs");
const DIRECT_STD_FS: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/direct_std_fs.rs"
);
const CLEAN_FILE: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/adversarial/clean_file.rs"
);
const COMMENT_USE_STD_FS: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/grep-attacks/rust-structural/comment_use_std_fs.rs"
);
const STRING_TODO: &str = include_str!(
    "../../../../../../../../apps/guardrail3/tests/fixtures/grep-attacks/rust-code-quality/string_todo.rs"
);

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
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

fn run_pipeline(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_ast_checks(&crawl).expect("ingestion should succeed");
    flatten_results(&inputs)
}

fn flatten_results(inputs: &[G3RsCodeAstChecksInput]) -> Vec<G3CheckResult> {
    inputs
        .iter()
        .flat_map(g3rs_code_ast_checks::check)
        .collect::<Vec<_>>()
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
fn pipeline_reports_expected_findings_on_real_source_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/has_todo.rs"), HAS_TODO);
    write(root.join("src/direct_std_fs.rs"), DIRECT_STD_FS);
    write(
        root.join("src/panic_probe.rs"),
        "pub fn run() { panic!(\"boom\"); }\n",
    );
    write(root.join("src/clean_file.rs"), CLEAN_FILE);

    let results = run_pipeline(root);
    let by_file = findings_by_file(&results);

    assert!(
        by_file["src/has_todo.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-13"),
        "todo fixture should trigger RS-CODE-13: {results:#?}"
    );
    assert!(
        by_file["src/direct_std_fs.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-15"),
        "direct std::fs fixture should trigger RS-CODE-15: {results:#?}"
    );
    assert!(
        by_file["src/panic_probe.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-16"),
        "panic fixture should trigger RS-CODE-16: {results:#?}"
    );
    assert!(
        !by_file.contains_key("src/clean_file.rs"),
        "clean source should not produce findings: {results:#?}"
    );
}

#[test]
fn pipeline_rejects_known_false_positive_fixture_patterns() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/comment_use_std_fs.rs"), COMMENT_USE_STD_FS);
    write(root.join("src/string_todo.rs"), STRING_TODO);

    let results = run_pipeline(root);

    assert!(
        results.is_empty(),
        "comment/string fixtures should stay clean under AST pipeline: {results:#?}"
    );
}

#[test]
fn pipeline_preserves_current_test_owned_rule_behavior() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("tests/smoke.rs"),
        "#[test]\nfn smoke() { todo!(); panic!(\"boom\"); let _ = std::fs::read_to_string(\"f\"); }\n",
    );
    write(
        root.join("src/helpers_tests.rs"),
        "pub fn helper() { todo!(); panic!(\"boom\"); let _ = std::fs::read_to_string(\"f\"); }\n",
    );

    let results = run_pipeline(root);

    assert_eq!(
        results.len(),
        2,
        "only todo! should currently fire in test-owned files: {results:#?}"
    );
    assert!(
        results.iter().all(|result| result.id() == "RS-CODE-13"),
        "test-owned files should currently suppress RS-CODE-15 and RS-CODE-16 only: {results:#?}"
    );
}
