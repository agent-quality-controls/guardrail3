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
