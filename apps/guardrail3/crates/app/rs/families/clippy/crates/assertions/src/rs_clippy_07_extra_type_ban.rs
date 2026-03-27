use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-07";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no extra type-ban findings: {results:#?}"
    );
}

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_project_specific(results: &[CheckResult], path: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "extra type ban");
    assert_eq!(
        result.message,
        format!("Additional type ban `{path}` beyond baseline.")
    );
}
