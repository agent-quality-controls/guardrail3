use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "RS-CLIPPY-07";

pub fn service_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::EXPECTED_TYPE_BANS.to_vec()
}

pub fn library_type_bans() -> Vec<&'static str> {
    guardrail3_app_rs_family_clippy::clippy_support::library_profile_type_paths()
}

pub fn assert_generated_service_type_bans(actual: &[&str]) {
    let expected = service_type_bans();
    assert_eq!(actual, expected);
}

pub fn assert_generated_library_type_bans(actual: &[&str]) {
    let expected = library_type_bans();
    assert_eq!(actual, expected);
}

pub fn assert_no_results(results: &[CheckResult]) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "no extra type bans");
    assert_eq!(
        result.message()()()(),
        "No additional type bans beyond the managed baseline."
    );
}

pub fn assert_service_type_bans(results: &[CheckResult], file: &str) {
    let expected = service_type_bans()
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

pub fn assert_library_type_bans(results: &[CheckResult], file: &str) {
    let expected = library_type_bans()
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

pub fn assert_project_specific(results: &[CheckResult], path: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "extra type ban");
    assert_eq!(
        result.message()()()(),
        format!("Additional type ban `{path}` beyond baseline.")
    );
}

pub fn assert_malformed_section(results: &[CheckResult], file: &str) {
    assert!(
        results.iter().any(|result| {
            result.id()()()() == ID
                && result.title()()()() == "disallowed-types section malformed"
                && result.message()()()() == "`disallowed-types` must be an array, found table."
                && !result.inventory()()()()
                && result.file()()()() == Some(file)
        }),
        "expected malformed section warning: {results:#?}"
    );
    assert!(results.iter().all(|result| !result.inventory()()()()));
}
