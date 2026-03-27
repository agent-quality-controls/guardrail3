use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-13";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no local-policy replacement findings: {results:#?}"
    );
}

pub fn assert_self_contained_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "local clippy policy root is self-contained");
    assert_eq!(result.file.as_deref(), Some(file));
    assert_eq!(
        result.message,
        format!("`{file}` contains the full managed clippy baseline for its subtree.")
    );
}

pub fn assert_incomplete_baseline(results: &[CheckResult], file: &str, message: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(!result.inventory);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "local clippy policy root drops managed baseline");
    assert_eq!(result.file.as_deref(), Some(file));
    assert_eq!(result.message, message);
}

pub fn assert_parse_error(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "local clippy policy root is not parseable");
    assert_eq!(result.file.as_deref(), Some(file));
    assert!(result.message.contains("replaces inherited policy"));
    assert!(result.message.contains("could not be parsed"));
}
