use guardrail3_app_rs_family_hooks_shared_assertions::shell_safety::hook_shared_19_real_dispatcher_syntax_only as assertions;

use crate::hook_shared_19_real_dispatcher_syntax_only::run_case;

#[test]
fn warns_when_modular_dir_exists_but_only_comment_mentions_dispatcher() {
    let results = run_case("# source .githooks/pre-commit.d/10-rust.sh\n", true);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("dispatcher syntax missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_real_dispatcher_command_exists() {
    let results = run_case(r#". ".githooks/pre-commit.d/10-rust.sh""#, true);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("dispatcher uses real executable syntax"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_dispatcher_targets_lookalike_path() {
    let results = run_case(r#". ".githooks/pre-commit.dummy/10-rust.sh""#, true);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("dispatcher syntax missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
