use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_11_valid_shebang as assertions;

use super::run_case;

#[test]
fn warns_when_shebang_is_missing() {
    let results = run_case("g3rs rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("hook shebang missing"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_shebang_is_supported() {
    let results = run_case("#!/usr/bin/env bash\ng3rs rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("valid hook shebang present"),
            line: Some(1),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
