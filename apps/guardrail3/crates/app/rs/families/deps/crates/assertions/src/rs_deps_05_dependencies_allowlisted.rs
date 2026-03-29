crate::define_rule_assertions!("RS-DEPS-05");

pub fn assert_broad_dependency_attack_summary(results: &[guardrail3_domain_report::CheckResult]) {
    let relevant = results
        .iter()
        .filter(|result| {
            matches!(
                result.id.as_str(),
                "RS-DEPS-05" | "RS-DEPS-06" | "RS-DEPS-07"
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        relevant.len(),
        3,
        "unexpected dependency allowlist summary: {results:#?}"
    );
    assert_rule_results(
        results,
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            message: Some(
                "Dependency `tokio` in `[dependencies]` is not allowlisted for crate `api`.",
            ),
            ..Default::default()
        }],
    );
    crate::common::assert_rule_results(
        results,
        "RS-DEPS-06",
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            message: Some(
                "Dependency `bindgen` in `[build-dependencies]` is not allowlisted for crate `api`.",
            ),
            ..Default::default()
        }],
    );
    crate::common::assert_rule_results(
        results,
        "RS-DEPS-07",
        &[ExpectedRuleResult {
            severity: Some(Severity::Warn),
            message: Some(
                "Dependency `tempfile` in `[dev-dependencies]` is not allowlisted for crate `api`.",
            ),
            ..Default::default()
        }],
    );
}
