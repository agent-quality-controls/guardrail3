use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-20";

pub fn expected_macro_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_MACRO_BANS.to_vec()
}

pub fn assert_generated_macro_bans(actual: &[&str]) {
    let expected = expected_macro_bans();
    assert_eq!(actual, expected);
}

pub fn macro_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_MACRO_BANS.to_vec()
}

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
        .map(|result| result.message())
        .collect::<Vec<_>>();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id() == ID
            && result.inventory()
            && result.severity() == Severity::Info
            && result.title() == "macro ban present"
            && result.file() == Some(file)
    }));
}

pub fn assert_malformed_section(results: &[CheckResult], expected_title: &str, file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id() == ID
                && matches!(result.severity(), Severity::Warn | Severity::Error)
                && result.title() == expected_title
                && result.message().contains("must be an array, found table.")
                && !result.inventory()
                && result.file() == Some(file)
        }),
        "expected malformed section error: {results:#?}"
    );
    assert!(results.iter().all(|result| !result.inventory()));
}

pub fn assert_expected_macro_bans(results: &[CheckResult], file: &str) {
    let expected = expected_macro_bans();
    assert_golden(results, &expected, file);
}

pub fn assert_missing_messages(results: &[CheckResult], expected: &[&str]) {
    let mut error_messages = results
        .iter()
        .filter(|result| result.severity() == Severity::Error)
        .map(|result| result.message())
        .collect::<Vec<_>>();
    let mut expected_error_messages = expected.to_vec();

    error_messages.sort();
    expected_error_messages.sort();
    assert_eq!(error_messages, expected_error_messages);
    assert!(results.iter().all(|result| result.id() == ID));
}
