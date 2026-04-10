use g3rs_hooks_shared_source_checks_assertions::workflow::hook_shared_15_merge_conflict_step_present as assertions;

use super::run_case;

#[test]
fn warns_when_conflict_check_only_appears_in_comment() {
    let results = run_case("# grep -qE '^(<{7} |={7}$|>{7} )' \"$file\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("merge-conflict check step missing"),
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
            title: Some("merge-conflict check step present"),
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
            title: Some("merge-conflict check step missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
