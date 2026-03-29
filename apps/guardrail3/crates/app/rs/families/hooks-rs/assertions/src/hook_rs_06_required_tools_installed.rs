crate::define_rule_assertions!("HOOK-RS-06");

pub fn assert_tool_present(results: &[guardrail3_domain_report::CheckResult], tool: &str) {
    let actual = findings(results);
    assert!(
        actual.iter().any(|result| {
            result.severity == Severity::Error
                && result.inventory
                && result.title == format!("{tool} installed")
        }),
        "missing installed result for {tool}: {actual:#?}"
    );
}

pub fn assert_tool_missing(results: &[guardrail3_domain_report::CheckResult], tool: &str) {
    let actual = findings(results);
    assert!(
        actual.iter().any(|result| {
            result.severity == Severity::Error
                && !result.inventory
                && result.title == format!("{tool} missing")
        }),
        "missing error result for {tool}: {actual:#?}"
    );
}
