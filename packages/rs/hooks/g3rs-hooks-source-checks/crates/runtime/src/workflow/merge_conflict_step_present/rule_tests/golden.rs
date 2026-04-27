use g3rs_hooks_source_checks_assertions::workflow::merge_conflict_step_present::rule as assertions;

use super::run_case;

#[test]
fn warns_when_conflict_check_only_appears_in_comment() {
    let results = run_case("# grep -qE '^(<{7} |={7}$|>{7} )' \"$file\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing merge-conflict marker scan in `.githooks/pre-commit`"),
            message_contains: Some("<<<<<<<"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_real_grep_conflict_command_exists() {
    let results = run_case("grep -qE '^(<{7} |={7}$|>{7} )' \"$file\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` scans for merge-conflict markers"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_called_function_runs_path_qualified_conflict_check() {
    let results =
        run_case("check_conflicts() {\n    /usr/bin/rg '<<<<<<<' .\n}\ncheck_conflicts\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` scans for merge-conflict markers"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_only_echo_mentions_conflict_markers() {
    let results = run_case("echo \"Checking for merge conflict markers...\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing merge-conflict marker scan in `.githooks/pre-commit`"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_grep_only_mentions_merge_conflict_prose() {
    let results = run_case("grep -q \"merge conflict\" README.md\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing merge-conflict marker scan in `.githooks/pre-commit`"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
