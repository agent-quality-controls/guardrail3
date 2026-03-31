use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-15";

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id(), ID);
    assert!(result.inventory());
    assert_eq!(result.severity(), Severity::Info);
    assert_eq!(result.title(), "no documented ban entries");
    assert_eq!(
        result.message(),
        "No managed ban entries are present, so there are no documented clippy escape hatches to review."
    );
    assert_eq!(result.file(), Some(file));
}

pub fn assert_weak_reason_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .filter(|result| result.file().is_some())
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(actual_messages.len(), expected_messages.len());
    assert!(
        results
            .iter()
            .filter(|result| result.file().is_some())
            .all(|result| {
                result.id() == ID
                    && !result.inventory()
                    && result.severity() == Severity::Error
                    && result.title() == "ban entry reason too weak"
                    && result.file() == Some(file)
            })
    );
}

pub fn assert_documented_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .filter(|result| result.file().is_some())
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(actual_messages.len(), expected_messages.len());
    assert!(
        results
            .iter()
            .filter(|result| result.file().is_some())
            .all(|result| {
                result.id() == ID
                    && !result.inventory()
                    && result.severity() == Severity::Warn
                    && result.title() == "ban entry uses documented escape hatch"
                    && result.file() == Some(file)
            })
    );
}

pub fn assert_count_summary(results: &[CheckResult], expected: &str) {
    let summaries = results
        .iter()
        .filter(|result| result.id() == ID && result.file().is_none())
        .collect::<Vec<_>>();
    assert_eq!(summaries.len(), 1);
    let result = summaries[0];
    assert_eq!(result.severity(), Severity::Warn);
    assert_eq!(result.title(), "ban entry count");
    assert_eq!(result.message(), expected);
    assert!(!result.inventory());
}

pub fn assert_placeholder_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    assert_weak_reason_messages(results, expected, file);
}

pub fn assert_malformed_messages(results: &[CheckResult], _expected: &[&str], file: &str) {
    assert_inventory(results, file);
}
