use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-07";

pub fn assert_no_results(results: &[CheckResult]) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "no extra type bans");
    assert_eq!(
        result.message,
        "No additional type bans beyond the managed baseline."
    );
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
