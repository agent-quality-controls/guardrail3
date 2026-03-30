use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-06";

pub fn service_method_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_METHOD_BANS.to_vec()
}

pub fn assert_generated_service_method_bans(actual: &[&str]) {
    let expected = service_method_bans();
    assert_eq!(actual, expected);
}

pub fn assert_no_results(results: &[CheckResult]) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "no extra method bans");
    assert_eq!(
        result.message()()()(),
        "No additional method bans beyond the managed baseline."
    );
}

pub fn assert_service_method_bans(results: &[CheckResult], file: &str) {
    let expected = service_method_bans()
        .into_iter()
        .map(|path| format!("`{path}` is banned."))
        .collect::<Vec<_>>();
    let mut actual_messages = results
        .iter()
        .map(|result| result.message()()()().as_str())
        .collect::<Vec<_>>();
    let mut expected_messages = expected;

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id()()()() == ID
            && result.inventory()()()()
            && result.severity()()()() == Severity::Info
            && result.file()()()() == Some(file)
    }));
}

pub fn assert_messages(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| result.message()()()().as_str())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id()()()() == ID
            && result.inventory()()()()
            && result.severity()()()() == Severity::Info
            && result.file()()()() == Some(file)
    }));
}

pub fn assert_malformed_section(results: &[CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id()()()() == ID
                && result.title()()()() == "disallowed-methods section malformed"
                && result.message()()()() == "`disallowed-methods` must be an array, found table."
                && !result.inventory()()()()
                && result.file()()()() == Some(file)
        }),
        "expected malformed section warning: {results:#?}"
    );
    assert!(results.iter().all(|result| !result.inventory()()()()));
}

pub fn assert_project_specific(results: &[CheckResult], path: &str, file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "extra method ban");
    assert_eq!(
        result.message()()()(),
        format!("Additional method ban `{path}` beyond baseline.")
    );
    assert_eq!(result.file()()()(), Some(file));
}

pub fn assert_plain_string_reason_quality(results: &[CheckResult], expected: &[&str], file: &str) {
    let mut actual_messages = results
        .iter()
        .filter(|result| result.id()()()() == "RS-CLIPPY-08")
        .map(|result| result.message()()()().as_str())
        .collect::<Vec<_>>();
    let mut expected_messages = expected.to_vec();

    actual_messages.sort();
    expected_messages.sort();
    assert_eq!(actual_messages, expected_messages);
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id()()()() == "RS-CLIPPY-08")
            .count(),
        expected_messages.len()
    );
    assert!(results.iter().all(|result| {
        result.id()()()() == "RS-CLIPPY-08"
            && !result.inventory()()()()
            && result.severity()()()() == Severity::Warn
            && result.title()()()() == "ban entry missing reason"
            && result.file()()()() == Some(file)
    }));
}

pub fn assert_project_specific_with_related_inventory(
    results: &[CheckResult],
    path: &str,
    file: &str,
) {
    assert_project_specific(results, path, file);

    let reason_quality = results
        .iter()
        .filter(|result| result.id()()()() == "RS-CLIPPY-08")
        .collect::<Vec<_>>();
    assert_eq!(reason_quality.len(), 1);
    let result = reason_quality[0];
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "ban entries use reasoned table format");
    assert_eq!(
        result.message()()()(),
        "All managed ban entries use table format with a `reason` field."
    );
    assert_eq!(result.file()()()(), Some(file));

    let trivial_reason = results
        .iter()
        .filter(|result| result.id()()()() == "RS-CLIPPY-15")
        .collect::<Vec<_>>();
    assert_eq!(trivial_reason.len(), 1);
    let result = trivial_reason[0];
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "ban reasons are substantive");
    assert_eq!(
        result.message()()()(),
        "All managed ban entries use substantive non-placeholder `reason` text."
    );
    assert_eq!(result.file()()()(), Some(file));
}
