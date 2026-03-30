use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-20";

pub fn assert_golden(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut expected_messages = expected
        .iter()
        .map(|path| {
            let macro_name = path.rsplit("::").next().expect("macro segment");
            format!("`{macro_name}!` is banned.")
        })
        .collect::<Vec<_>>();
    let mut actual_messages = results
        .iter()
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id == ID
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "macro ban present"
            && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let mut error_messages = results
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .map(|result| result.message.as_str())
        .collect::<Vec<_>>();
    let mut expected_error_messages = expected.to_vec();

    error_messages.sort();
    expected_error_messages.sort();
    assert_eq!(error_messages, expected_error_messages);
    assert!(results.iter().all(|result| result.id == ID));
}
