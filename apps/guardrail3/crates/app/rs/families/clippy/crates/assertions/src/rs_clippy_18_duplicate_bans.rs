use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-18";

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "ban entries are duplicate-free");
    assert_eq!(
        result.message,
        "Managed ban sections contain no duplicate paths."
    );
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.severity == Severity::Warn
            && result.title == "duplicate ban entry"
            && !result.inventory
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_malformed_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.severity == Severity::Warn
            && result.title == "ban section malformed"
            && !result.inventory
            && result.file.as_deref() == Some(file)
    }));
}
