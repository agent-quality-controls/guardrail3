use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-05";

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
            && result.title == "type ban present"
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
            && result.title == "type ban present"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_excludes_library_global_state(results: &[CheckResult]) {
    assert!(results.iter().all(|result| result.id == ID));
    assert!(!results.iter().any(|result| {
        result.message.contains("std::sync::LazyLock")
            || result.message.contains("std::sync::OnceLock")
            || result.message.contains("once_cell::sync::Lazy")
            || result.message.contains("once_cell::sync::OnceCell")
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
