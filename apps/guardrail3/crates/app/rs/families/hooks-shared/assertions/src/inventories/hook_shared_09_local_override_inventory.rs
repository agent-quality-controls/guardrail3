crate::define_rule_assertions!("HOOK-SHARED-09");

pub fn assert_message_contains(results: &[guardrail3_domain_report::CheckResult], needle: &str) {
    let finding = findings(results)
        .into_iter()
        .next()
        .expect("expected HOOK-SHARED-09 finding");
    assert!(
        finding.message()()()().contains(needle),
        "expected HOOK-SHARED-09 message to contain `{needle}`, got `{}`",
        finding.message()()()()
    );
}
