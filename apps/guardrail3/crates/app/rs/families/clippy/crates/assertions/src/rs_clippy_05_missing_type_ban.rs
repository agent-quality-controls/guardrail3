use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-05";

pub fn assert_golden(results: &[CheckResult], file: &str) {
    assert!(!results.is_empty());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "type ban present"
            && result.file.as_deref() == Some(file)
    }));
    assert!(results.iter().any(|result| result.message == "`std::collections::HashMap` is banned."));
    assert!(results.iter().any(|result| result.message == "`std::any::Any` is banned."));
}

pub fn assert_garde_disabled(results: &[CheckResult], file: &str) {
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some(file)
    }));
    assert!(!results.iter().any(|result| result.message.contains("axum::extract::Json")));
    assert!(!results.iter().any(|result| result.message.contains("axum::extract::Form")));
}

pub fn assert_library_global_state_inventory(results: &[CheckResult]) {
    assert!(results.iter().all(|result| result.id == ID));
    assert!(results.iter().any(|result| {
        result.severity == Severity::Info
            && result.inventory
            && result.message == "`std::sync::LazyLock` is banned."
    }));
    assert!(results.iter().any(|result| {
        result.severity == Severity::Info
            && result.inventory
            && result.message == "`once_cell::sync::OnceCell` is banned."
    }));
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
