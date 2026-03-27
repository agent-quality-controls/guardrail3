use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-04";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    assert!(!results.is_empty());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "method ban present"
            && result.file.as_deref() == Some(file)
    }));
    assert!(results.iter().any(|result| result.message == "`std::env::var` is banned."));
    assert!(results.iter().any(|result| result.message == "`std::process::abort` is banned."));
}

pub fn assert_garde_disabled(results: &[CheckResult], file: &str) {
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some(file)
    }));
    assert!(!results.iter().any(|result| result.message.contains("serde_json::from_str")));
    assert!(!results.iter().any(|result| result.message.contains("reqwest::Response::json")));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let actual_errors = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_errors = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_errors, expected_errors);
    assert!(results.iter().all(|result| result.id == ID));
}
