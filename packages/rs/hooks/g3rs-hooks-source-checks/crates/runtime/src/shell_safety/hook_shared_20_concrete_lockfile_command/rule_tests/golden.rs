use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_20_concrete_lockfile_command::rule as assertions;

use super::run_case;

#[test]
fn warns_when_lockfile_check_is_only_prose() {
    let results = run_case("echo \"run pnpm install --frozen-lockfile\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
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
            severity: Some(assertions::G3Severity::Info),
            title: Some("concrete lockfile integrity command present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_non_install_pnpm_command_uses_frozen_lockfile_flag() {
    let results = run_case("pnpm info --frozen-lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("concrete lockfile integrity command missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_env_wrapper_executes_real_frozen_lockfile_command() {
    let results = run_case("env -i pnpm install --frozen-lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("concrete lockfile integrity command present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_path_qualified_pnpm_executes_real_frozen_lockfile_command() {
    let results = run_case("/usr/local/bin/pnpm install --frozen-lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("concrete lockfile integrity command present"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_called_function_executes_real_frozen_lockfile_command() {
    let results =
        run_case("verify_lockfile() {\n    pnpm i --frozen-lockfile\n}\nverify_lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
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
            severity: Some(assertions::G3Severity::Warn),
            title: Some("concrete lockfile integrity command missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
