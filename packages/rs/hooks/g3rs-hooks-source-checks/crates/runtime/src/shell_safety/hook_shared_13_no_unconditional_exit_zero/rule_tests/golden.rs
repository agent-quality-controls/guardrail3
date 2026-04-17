use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_13_no_unconditional_exit_zero::rule as assertions;

use super::run_case;

#[test]
fn warns_when_exit_zero_is_executable() {
    let results = run_case("exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("unconditional exit 0 bypass present"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_exit_zero_only_appears_in_comment() {
    let results = run_case("# exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional exit 0 bypass"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_called_function_contains_exit_zero() {
    let results = run_case("finish() {\n    exit 0\n}\nfinish\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("unconditional exit 0 bypass present"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
