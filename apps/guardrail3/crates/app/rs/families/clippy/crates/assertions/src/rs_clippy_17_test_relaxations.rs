use guardrail3_domain_modules::clippy::{
    ALLOW_DBG_IN_TESTS, ALLOW_EXPECT_IN_TESTS, ALLOW_PANIC_IN_TESTS, ALLOW_PRINT_IN_TESTS,
    ALLOW_UNWRAP_IN_TESTS,
};
use guardrail3_domain_report::CheckResult;

const ID: &str = "RS-CLIPPY-17";

pub use guardrail3_domain_report::Severity;

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 0,
        Severity::Warn => 1,
        Severity::Info => 2,
    }
}

pub fn assert_no_results(results: &[CheckResult]) {
    assert!(
        results.is_empty(),
        "expected no test-relaxation findings: {results:#?}"
    );
}

pub fn assert_messages(results: &[CheckResult], expected: &[(Severity, &str, &str)], file: &str) {
    let mut actual_messages = results
        .iter()
        .map(|result| {
            (
                result.severity,
                result.title.clone(),
                result.message.clone(),
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
        result.id == ID && !result.inventory && result.file.as_deref() == Some(file)
    }));
}

pub fn assert_generated_service_baseline_keeps_policy_exact(clippy_toml: &str) {
    let parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");

    for key in [
        "allow-dbg-in-tests",
        "allow-expect-in-tests",
        "allow-panic-in-tests",
        "allow-print-in-tests",
        "allow-unwrap-in-tests",
    ] {
        assert_eq!(
            parsed.get(key).and_then(toml::Value::as_bool),
            Some(expected_bool_value(key)),
            "unexpected canonical value for {key}"
        );
    }
}

fn expected_bool_value(key: &str) -> bool {
    match key {
        "allow-dbg-in-tests" => ALLOW_DBG_IN_TESTS,
        "allow-expect-in-tests" => ALLOW_EXPECT_IN_TESTS,
        "allow-panic-in-tests" => ALLOW_PANIC_IN_TESTS,
        "allow-print-in-tests" => ALLOW_PRINT_IN_TESTS,
        "allow-unwrap-in-tests" => ALLOW_UNWRAP_IN_TESTS,
        _ => unreachable!("unsupported key"),
    }
}
