use g3rs_test_types::{G3RsTestComponentSourceFacts, G3RsTestFileKind, G3RsTestSourceChecksInput, G3RsTestSourceFile};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn check(rel_path: &str, content: &str) -> Vec<G3CheckResult> {
    g3rs_test_source_checks_runtime::check(&G3RsTestSourceChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files: vec![G3RsTestSourceFile {
            rel_path: rel_path.to_owned(),
            kind: G3RsTestFileKind::ExternalHarness,
            owner_module_name: None,
            component_rel_dir: Some(String::new()),
            assertions_package_name: None,
            content: content.to_owned(),
        }],
        components: vec![G3RsTestComponentSourceFacts {
            rel_dir: String::new(),
            runtime_rel_dir: String::new(),
            runtime_package_name: Some("demo".to_owned()),
            assertions_rel_dir: "assertions".to_owned(),
            assertions_exists: false,
            assertions_package_name: None,
        }],
    })
}

pub fn assert_has_result(
    results: &[G3CheckResult],
    rule_id: &str,
    severity: G3Severity,
    title: &str,
    file: &str,
) {
    assert!(
        results.iter().any(|result| {
            result.id() == rule_id
                && result.severity() == severity
                && result.title() == title
                && result.file() == Some(file)
        }),
        "missing {rule_id} result: severity={severity:?} title={title:?} file={file:?}\nactual={results:#?}"
    );
}

pub fn assert_message_contains(results: &[G3CheckResult], needle: &str) {
    assert_eq!(results.len(), 1, "{results:#?}");
    assert!(
        results[0].message().contains(needle),
        "message {:?} did not contain {:?}",
        results[0].message(),
        needle
    );
}
