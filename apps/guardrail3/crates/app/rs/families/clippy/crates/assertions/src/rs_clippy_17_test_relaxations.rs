use guardrail3_domain_report::CheckResult;
const ID: &str = "RS-CLIPPY-17";

pub use guardrail3_domain_report::Severity;

pub fn expected_service_relaxation_values() -> Vec<(&'static str, bool)> {
    vec![
        (
            "allow-dbg-in-tests",
            guardrail3_app_rs_family_clippy::clippy_support::ALLOW_DBG_IN_TESTS,
        ),
        (
            "allow-expect-in-tests",
            guardrail3_app_rs_family_clippy::clippy_support::ALLOW_EXPECT_IN_TESTS,
        ),
        (
            "allow-panic-in-tests",
            guardrail3_app_rs_family_clippy::clippy_support::ALLOW_PANIC_IN_TESTS,
        ),
        (
            "allow-print-in-tests",
            guardrail3_app_rs_family_clippy::clippy_support::ALLOW_PRINT_IN_TESTS,
        ),
        (
            "allow-unwrap-in-tests",
            guardrail3_app_rs_family_clippy::clippy_support::ALLOW_UNWRAP_IN_TESTS,
        ),
    ]
}

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warn => 1,
        Severity::Info => 2,
    }
}

pub fn assert_inventory(results: &[CheckResult], file: &str) {
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id()()()(), ID);
    assert!(result.inventory()()()());
    assert_eq!(result.severity()()()(), Severity::Info);
    assert_eq!(result.title()()()(), "clippy test relaxation policy exact");
    assert_eq!(
        result.message()()()(),
        "Managed test relaxation keys match the expected clippy policy."
    );
    assert_eq!(result.file()()()(), Some(file));
}

pub fn assert_service_relaxations_exact(parsed: &toml::Value) {
    for (key, expected) in expected_service_relaxation_values() {
        assert_eq!(
            parsed.get(key).and_then(toml::Value::as_bool),
            Some(expected),
            "unexpected canonical value for {key}",
        );
    }
}

pub fn assert_missing_messages(results: &[CheckResult], file: &str) {
    assert_messages(
        results,
        &[
            (
                Severity::Warn,
                "clippy test relaxation enabled missing",
                "`allow-dbg-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Warn,
                "clippy test relaxation enabled missing",
                "`allow-print-in-tests` must be set explicitly to `false`. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Error,
                "clippy test expect policy misconfigured missing",
                "`allow-expect-in-tests` must be set explicitly to `true`. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
            ),
            (
                Severity::Error,
                "clippy test panic relaxation enabled missing",
                "`allow-panic-in-tests` must be set explicitly to `false`. panic!() must remain banned in tests.",
            ),
            (
                Severity::Error,
                "clippy test unwrap relaxation enabled missing",
                "`allow-unwrap-in-tests` must be set explicitly to `false`. unwrap() must remain banned in tests.",
            ),
        ],
        file,
    );
}

pub fn assert_wrong_type_messages(results: &[CheckResult], file: &str) {
    assert_messages(
        results,
        &[
            (
                Severity::Warn,
                "clippy test relaxation enabled wrong type",
                "`allow-dbg-in-tests` must be a bool with value `false`, found string. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Warn,
                "clippy test relaxation enabled wrong type",
                "`allow-print-in-tests` must be a bool with value `false`, found integer. Tests should stay quiet and deterministic.",
            ),
            (
                Severity::Error,
                "clippy test expect policy misconfigured wrong type",
                "`allow-expect-in-tests` must be a bool with value `true`, found array. Tests may use `expect(...)` while non-test code stays governed by `clippy::expect_used`.",
            ),
            (
                Severity::Error,
                "clippy test panic relaxation enabled wrong type",
                "`allow-panic-in-tests` must be a bool with value `false`, found table. panic!() must remain banned in tests.",
            ),
            (
                Severity::Error,
                "clippy test unwrap relaxation enabled wrong type",
                "`allow-unwrap-in-tests` must be a bool with value `false`, found float. unwrap() must remain banned in tests.",
            ),
        ],
        file,
    );
}

pub fn assert_messages(results: &[CheckResult], expected: &[(Severity, &str, &str)], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| {
            (
                result.severity()()()(),
                result.title()()()().clone(),
                result.message()()()().clone(),
            )
        })
        .collect::<Vec<_>>();
    actual_messages.sort_by(|left, right| {
        severity_rank(left.0)
            .cmp(&severity_rank(right.0))
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    let mut expected_messages = expected
        .iter()
        .map(|(severity, title, message)| (*severity, (*title).to_owned(), (*message).to_owned()))
        .collect::<Vec<_>>();
    expected_messages.sort_by(|left, right| {
        severity_rank(left.0)
            .cmp(&severity_rank(right.0))
            .then(left.1.cmp(&right.1))
            .then(left.2.cmp(&right.2))
    });

    assert_eq!(actual_messages, expected_messages);
    assert!(results.iter().all(|result| {
        result.id()()()() == ID && !result.inventory()()()() && result.file()()()() == Some(file)
    }));
}
