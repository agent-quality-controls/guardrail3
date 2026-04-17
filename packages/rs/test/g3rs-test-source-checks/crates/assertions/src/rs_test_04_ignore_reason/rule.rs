use g3rs_test_types::{G3RsTestComponentSourceFacts, G3RsTestFileKind, G3RsTestSourceChecksInput, G3RsTestSourceFile};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn check(input: &G3RsTestSourceChecksInput) -> Vec<G3CheckResult> {
    g3rs_test_source_checks_runtime::check(input)
}

pub fn file(
    rel_path: &str,
    kind: G3RsTestFileKind,
    assertions_package_name: Option<&str>,
    content: &str,
) -> G3RsTestSourceFile {
    G3RsTestSourceFile {
        rel_path: rel_path.to_owned(),
        kind,
        owner_module_name: None,
        component_rel_dir: Some(String::new()),
        assertions_package_name: assertions_package_name.map(str::to_owned),
        content: content.to_owned(),
    }
}

pub fn input(
    files: Vec<G3RsTestSourceFile>,
    assertions_package_name: Option<&str>,
) -> G3RsTestSourceChecksInput {
    G3RsTestSourceChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files,
        components: vec![G3RsTestComponentSourceFacts {
            rel_dir: String::new(),
            runtime_rel_dir: String::new(),
            runtime_package_name: Some("demo".to_owned()),
            assertions_rel_dir: "assertions".to_owned(),
            assertions_exists: assertions_package_name.is_some(),
            assertions_package_name: assertions_package_name.map(str::to_owned),
        }],
    }
}

pub fn assert_has_result(
    results: &[G3CheckResult],
    rule_id: &str,
    severity: G3Severity,
    title: &str,
    file: &str,
    line: Option<usize>,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.severity() == severity
                && result.title() == title
                && result.file() == Some(file)
                && result.line() == line
        }),
        "missing {rule_id} result: severity={severity:?} title={title:?} file={file:?} line={line:?}\nactual={results:#?}"
    );
}

pub fn assert_has_inventory(results: &[G3CheckResult], rule_id: &str, title: &str, file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.title() == title
                && result.file() == Some(file)
                && result.inventory()
        }),
        "missing inventory {rule_id} result: title={title:?} file={file:?}\nactual={results:#?}"
    );
}

pub fn assert_title_count(results: &[G3CheckResult], rule_id: &str, title: &str, count: usize) {
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == rule_id && result.title() == title)
            .count(),
        count,
        "{results:#?}"
    );
}

pub fn assert_message_count(results: &[G3CheckResult], rule_id: &str, title: &str, message: &str, count: usize) {
    assert_eq!(
        results
            .iter()
            .filter(|result| {
                result.id() == rule_id && result.title() == title && result.message() == message
            })
            .count(),
        count,
        "{results:#?}"
    );
}
