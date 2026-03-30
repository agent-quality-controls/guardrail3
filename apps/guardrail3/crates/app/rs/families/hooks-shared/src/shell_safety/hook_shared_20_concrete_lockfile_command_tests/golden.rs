use guardrail3_app_rs_family_hooks_shared_assertions::hook_shared_20_concrete_lockfile_command as assertions;

use crate::hook_shared_20_concrete_lockfile_command::run_case;

#[test]
fn warns_when_lockfile_check_is_only_prose() {
    let results = run_case("echo \"run pnpm install --frozen-lockfile\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("concrete lockfile integrity command missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_real_frozen_lockfile_command_exists() {
    let results = run_case("pnpm install --frozen-lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            title: Some("concrete lockfile integrity command present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_frozen_lockfile_command_is_echoed() {
    let results = run_case("echo \"pnpm install --frozen-lockfile\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            title: Some("concrete lockfile integrity command missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
