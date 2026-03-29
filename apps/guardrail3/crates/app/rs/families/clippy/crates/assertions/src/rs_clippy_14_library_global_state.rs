use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-14";

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no library-global-state evaluation results: {results:#?}"
    );
}

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, ID);
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "library global-state bans present");
    assert_eq!(
        result.message,
        "Library profile includes all managed global-state type bans."
    );
    assert_eq!(result.file.as_deref(), Some(file));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let actual_messages = results
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = expected
        .iter()
        .map(|message| (*message).to_owned())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_messages, expected_messages);
    assert_eq!(results.len(), expected_messages.len());
    assert!(results.iter().all(|result| {
        result.id == ID
            && !result.inventory
            && result.severity == Severity::Error
            && result.title == "library clippy.toml missing global-state type ban"
            && result.file.as_deref() == Some(file)
    }));
}
