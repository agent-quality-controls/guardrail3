use g3rs_hooks_source_checks_assertions::shell_safety::shell_error_handling::rule as assertions;

use super::run_case;

#[test]
fn warns_when_set_e_only_appears_in_comment() {
    let results = run_case("# set -euo pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing fail-closed shell options in `.githooks/pre-commit`"),
            message_contains: Some("set -euo pipefail"),
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
            title: Some("missing fail-closed shell options in `.githooks/pre-commit`"),
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
            title: Some("`.githooks/pre-commit` enables fail-closed shell options"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_equivalent_set_eu_o_pipefail_line_exists() {
    let results = run_case("#!/usr/bin/env bash\nset -eu -o pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("`.githooks/pre-commit` enables fail-closed shell options"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_equivalent_split_short_flags_line_exists() {
    let results = run_case("#!/usr/bin/env bash\nset -e -u -o pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("`.githooks/pre-commit` enables fail-closed shell options"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_equivalent_reordered_short_flags_line_exists() {
    let results = run_case("#!/usr/bin/env bash\nset -ue -o pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("`.githooks/pre-commit` enables fail-closed shell options"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_equivalent_long_errexit_spelling_exists() {
    let results = run_case("#!/usr/bin/env bash\nset -o errexit -u -o pipefail\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("`.githooks/pre-commit` enables fail-closed shell options"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
