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
    assert_eq!(
        by_file["src/has_todo.rs"].len(),
        1,
        "todo fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        by_file["src/direct_std_fs.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-15"),
        "direct std::fs fixture should trigger RS-CODE-15: {results:#?}"
    );
    assert_eq!(
        by_file["src/direct_std_fs.rs"].len(),
        1,
        "direct std::fs fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        by_file["src/panic_probe.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-16"),
        "panic fixture should trigger RS-CODE-16: {results:#?}"
    );
    assert_eq!(
        by_file["src/panic_probe.rs"].len(),
        1,
        "panic fixture should emit exactly one finding: {results:#?}"
    );
    assert!(
        !by_file.contains_key("src/clean_file.rs"),
        "clean source should not produce findings: {results:#?}"
    );
}

#[test]
fn pipeline_reports_new_single_file_ast_rules() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("src/crate_allow.rs"),
        "#![allow(dead_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/unused_crate_deps.rs"),
        "#![allow(unused_crate_dependencies)]\nfn probe() {}\n",
    );
    write(
        root.join("src/item_allow_missing_reason.rs"),
        "#[allow(clippy::too_many_lines)]\nfn probe() {}\n",
    );
    write(
        root.join("src/item_allow_with_reason.rs"),
        "#[allow(clippy::too_many_lines)] // reason: generated ffi shim\nfn probe() {}\n",
    );
    write(
        root.join("src/garde_skip.rs"),
        "struct Form {\n    #[garde(skip)] // reason: validated upstream boundary\n    token: String,\n}\n",
    );
    write(
        root.join("src/garde_skip_no_comment.rs"),
        "struct Form {\n    #[garde(skip)]\n    token: String,\n}\n",
    );
    write(
        root.join("src/cfg_attr_unknown.rs"),
        "#[cfg_attr(feature = \"cli\", allow(dead_code))]\nfn probe() {}\n",
    );
    write(
        root.join("src/deny_without_reason.rs"),
        "#[deny(dead_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/impl_allow.rs"),
        "struct Foo;\n#[allow(clippy::too_many_lines)]\nimpl Foo { fn a(&self) {} fn b(&self) {} fn c(&self) {} fn d(&self) {} }\n",
    );
    write(
        root.join("src/forbid_inventory.rs"),
        "#![forbid(unsafe_code)]\nfn probe() {}\n",
    );
    write(
        root.join("src/cfg_attr.rs"),
        "#[cfg_attr(all(), allow(dead_code))]\nfn probe() {}\n",
    );
    write(
        root.join("src/ffi.rs"),
        "#[allow(improper_ctypes)]\nunsafe extern \"C\" { fn puts(s: *const i8); }\n",
    );
    write(
        root.join("src/fs_glob.rs"),
        "use std::fs::*;\nfn probe() {}\n",
    );
    write(
        root.join("src/include_probe.rs"),
        "include!(\"../generated.rs\");\n",
    );
    write(
        root.join("tests/expect_probe.rs"),
        "fn probe() { let _ = Some(1).expect(\"ok\"); }\n",
    );
    write(
        root.join("src/generic_probe.rs"),
        "pub fn build<A, B, C, D, E, F, G>() {}\n",
    );
    write(
        root.join("src/string_dispatch.rs"),
        "pub fn dispatch(value: &str) -> usize { if value == \"v0\" { 0 } else if value == \"v1\" { 1 } else if value == \"v2\" { 2 } else if value == \"v3\" { 3 } else if value == \"v4\" { 4 } else if value == \"v5\" { 5 } else if value == \"v6\" { 6 } else if value == \"v7\" { 7 } else if value == \"v8\" { 8 } else if value == \"v9\" { 9 } else if value == \"v10\" { 10 } else { 0 } }\n",
    );

    let results = run_pipeline(root);
    let by_file = findings_by_file(&results);

    assert_eq!(by_file["src/crate_allow.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/crate_allow.rs"][0].id(), "RS-CODE-01");

    assert_eq!(by_file["src/unused_crate_deps.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/unused_crate_deps.rs"][0].id(), "RS-CODE-02");

    assert_eq!(
        by_file["src/item_allow_missing_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/item_allow_missing_reason.rs"][0].id(),
        "RS-CODE-03"
    );

    assert_eq!(
        by_file["src/item_allow_with_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/item_allow_with_reason.rs"][0].id(),
        "RS-CODE-04"
    );

    assert_eq!(by_file["src/garde_skip.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/garde_skip.rs"][0].id(), "RS-CODE-06");

    assert_eq!(
        by_file["src/garde_skip_no_comment.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(
        by_file["src/garde_skip_no_comment.rs"][0].id(),
        "RS-CODE-05"
    );

    assert_eq!(by_file["src/cfg_attr_unknown.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/cfg_attr_unknown.rs"][0].id(), "RS-CODE-08");

    assert_eq!(
        by_file["src/deny_without_reason.rs"].len(),
        1,
        "{results:#?}"
    );
    assert_eq!(by_file["src/deny_without_reason.rs"][0].id(), "RS-CODE-22");

    assert_eq!(by_file["src/impl_allow.rs"].len(), 2, "{results:#?}");
    assert!(
        by_file["src/impl_allow.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-03"),
        "{results:#?}"
    );
    assert!(
        by_file["src/impl_allow.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-17"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/cfg_attr.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/cfg_attr.rs"][0].id(), "RS-CODE-18");

    assert_eq!(by_file["src/ffi.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/ffi.rs"][0].id(), "RS-CODE-20");

    assert_eq!(by_file["src/fs_glob.rs"].len(), 2, "{results:#?}");
    assert!(
        by_file["src/fs_glob.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-15"),
        "{results:#?}"
    );
    assert!(
        by_file["src/fs_glob.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-21"),
        "{results:#?}"
    );

    assert_eq!(by_file["src/include_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/include_probe.rs"][0].id(), "RS-CODE-23");

    assert_eq!(by_file["src/forbid_inventory.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/forbid_inventory.rs"][0].id(), "RS-CODE-22");
    assert!(
        by_file["src/forbid_inventory.rs"][0].inventory(),
        "{results:#?}"
    );

    assert_eq!(by_file["tests/expect_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["tests/expect_probe.rs"][0].id(), "RS-CODE-32");

    assert_eq!(by_file["src/generic_probe.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/generic_probe.rs"][0].id(), "RS-CODE-34");

    assert_eq!(by_file["src/string_dispatch.rs"].len(), 1, "{results:#?}");
    assert_eq!(by_file["src/string_dispatch.rs"][0].id(), "RS-CODE-36");
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

#[test]
fn pipeline_emits_explicit_input_failure_for_parse_error() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/broken.rs"), "fn broken( {");

    let results = run_pipeline(root);

    assert_eq!(
        results.len(),
        1,
        "broken source should emit one input failure"
    );
    let result = &results[0];
    assert_eq!(result.id(), "RS-CODE-30");
    assert_eq!(result.title(), "code-family input failure");
    assert_eq!(result.file(), Some("src/broken.rs"));
    assert!(
        result
            .message()
            .starts_with("Failed to parse Rust source file:"),
        "unexpected message: {result:#?}"
    );
}

#[test]
fn pipeline_stays_clean_on_small_workspace_baseline() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/clean_file.rs"), CLEAN_FILE);
    write(root.join("src/string_todo.rs"), STRING_TODO);
    write(root.join("src/comment_use_std_fs.rs"), COMMENT_USE_STD_FS);

    let results = run_pipeline(root);

    assert!(
        results.is_empty(),
        "clean baseline workspace should stay clean: {results:#?}"
    );
}

#[test]
fn pipeline_keeps_other_findings_when_one_file_fails_to_parse() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/broken.rs"), "fn broken( {");
    write(root.join("src/has_todo.rs"), HAS_TODO);

    let results = run_pipeline(root);
    let by_file = findings_by_file(&results);

    assert!(
        by_file["src/broken.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-30"),
        "broken file should still emit parse failure: {results:#?}"
    );
    assert!(
        by_file["src/has_todo.rs"]
            .iter()
            .any(|result| result.id() == "RS-CODE-13"),
        "valid file should still emit its finding: {results:#?}"
    );
}
