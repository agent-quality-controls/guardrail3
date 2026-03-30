use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-08";

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "ban entries use reasoned table format");
    assert_eq!(
        result.message(),
        "All managed ban entries use table format with a `reason` field."
    );
    assert_eq!(result.file(), Some(file));
}

pub fn assert_missing_reasons(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id() == ID
            && !result.inventory()
            && result.severity() == Severity::Warn
            && result.title() == "ban entry missing reason"
            && result.file() == Some(file)
    }));
}

pub fn assert_malformed_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id() == ID
            && !result.inventory()
            && result.severity() == Severity::Warn
            && result.title() == "ban section malformed"
            && result.file() == Some(file)
    }));
}
