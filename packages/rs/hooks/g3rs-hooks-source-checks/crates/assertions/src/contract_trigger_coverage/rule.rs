crate::define_rule_assertions!("g3rs-hooks/contract-trigger-coverage");

pub fn assert_not_proven_warning(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-trigger-coverage"
                && result.severity() == Severity::Warn
                && result.title() == "hook contract trigger coverage is not proven"
        }),
        "contract trigger coverage should report not-proven warning; got {results:#?}"
    );
}
