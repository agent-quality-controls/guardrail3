use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-04";

pub fn assert_golden(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "method ban present"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_garde_disabled(results: &[CheckResult], expected: &[&str], file: &str) {
    let expected_messages = expected
        .iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "method ban present"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let actual_errors = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(actual_errors, expected);
    assert!(results.iter().all(|result| result.id == ID));
}
