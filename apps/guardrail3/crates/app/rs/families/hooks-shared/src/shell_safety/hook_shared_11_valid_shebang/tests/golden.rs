use guardrail3_app_rs_family_hooks_shared_assertions::shell_safety::hook_shared_11_valid_shebang as assertions;

use crate::hook_shared_11_valid_shebang::run_case;

#[test]
fn warns_when_shebang_is_missing() {
    let results = run_case("guardrail3 rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("hook shebang missing"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_shebang_is_supported() {
    let results = run_case("#!/usr/bin/env bash\nguardrail3 rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("valid hook shebang present"),
            line: Some(1),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
