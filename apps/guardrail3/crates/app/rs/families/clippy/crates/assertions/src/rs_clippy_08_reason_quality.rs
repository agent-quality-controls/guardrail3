use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-08";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no missing-reason findings: {results:#?}"
    );
}

pub fn assert_missing_reasons(results: &[CheckResult], expected: &[&str], file: &str) {
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
            && !result.inventory
            && result.severity == Severity::Warn
            && result.title == "ban entry missing reason"
            && result.file.as_deref() == Some(file)
    }));
}
