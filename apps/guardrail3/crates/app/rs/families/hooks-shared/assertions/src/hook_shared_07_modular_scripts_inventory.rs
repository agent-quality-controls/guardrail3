crate::define_rule_assertions!("HOOK-SHARED-07");

pub fn assert_message_contains_all(results: &[guardrail3_domain_report::CheckResult], needles: &[&str]) {
    let finding = findings(results)
        .into_iter()
        .next()
        .expect("expected HOOK-SHARED-07 finding");
    for needle in needles {
        assert!(
            finding.message.contains(needle),
            "expected HOOK-SHARED-07 message to contain `{needle}`, got `{}`",
            finding.message
        );
    }
}
