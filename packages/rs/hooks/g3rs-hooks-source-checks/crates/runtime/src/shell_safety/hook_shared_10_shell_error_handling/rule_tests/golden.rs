use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_10_shell_error_handling::rule as assertions;

use super::run_case;

#[test]
fn warns_when_set_e_only_appears_in_comment() {
    let results = run_case("# set -euo pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("shell error handling missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_set_e_only_appears_in_echo() {
    let results = run_case("echo \"set -euo pipefail\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("shell error handling missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_real_shell_error_handling_line_exists() {
    let results = run_case("#!/usr/bin/env bash\nset -euo pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("shell error handling present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
