use guardrail3_app_rs_family_hooks_shared_assertions::bootstrap::hook_shared_04_dispatcher_pattern as assertions;

use super::run_case;

#[test]
fn reports_inventory_when_modular_dir_is_absent() {
    let results = run_case("", false);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("monolithic hook mode"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_dispatcher_targets_lookalike_path() {
    let results = run_case(". .githooks/pre-commit.dummy/10-rust.sh\n", true);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("dispatcher pattern missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_dispatcher_targets_pre_commit_dir() {
    let results = run_case("run-parts .githooks/pre-commit.d\n", true);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("dispatcher pattern present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
