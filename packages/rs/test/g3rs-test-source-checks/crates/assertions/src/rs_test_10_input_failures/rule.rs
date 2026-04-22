use g3rs_test_types::G3RsTestSourceChecksInput;
use g3rs_test_types::G3RsTestSourceInputFailure;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn check(rel_path: &str, message: &str) -> Vec<G3CheckResult> {
    let parse_message = if let Err(err) = syn::parse_file(message) {
        format!("Failed to parse Rust source file for test-family source analysis: {err}")
    } else {
        message.to_owned()
    };
    g3rs_test_source_checks_runtime::check(&G3RsTestSourceChecksInput {
        root_rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        files: Vec::new(),
        components: Vec::new(),
        input_failures: vec![G3RsTestSourceInputFailure {
            rel_path: rel_path.to_owned(),
            message: parse_message,
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
