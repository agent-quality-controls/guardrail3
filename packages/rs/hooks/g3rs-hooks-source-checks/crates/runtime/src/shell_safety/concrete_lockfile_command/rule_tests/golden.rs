use g3rs_hooks_source_checks_assertions::shell_safety::concrete_lockfile_command::rule as assertions;

use super::run_case;

#[test]
fn warns_when_lockfile_check_is_only_prose() {
    let results = run_case("echo \"run cargo metadata --locked\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing concrete lockfile integrity command in `.githooks/pre-commit`"),
            message_contains: Some("cargo metadata --locked"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_real_cargo_metadata_locked_command_exists() {
    let results = run_case("cargo metadata --locked --format-version 1\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` runs a concrete lockfile integrity command"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_non_metadata_cargo_command_uses_locked_flag() {
    let results = run_case("cargo check --locked\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing concrete lockfile integrity command in `.githooks/pre-commit`"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_env_wrapper_executes_real_cargo_metadata_locked_command() {
    let results = run_case("env -i cargo metadata --locked\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` runs a concrete lockfile integrity command"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_path_qualified_cargo_executes_real_metadata_locked_command() {
    let results = run_case("/usr/local/bin/cargo metadata --locked\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` runs a concrete lockfile integrity command"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_called_function_executes_real_cargo_metadata_locked_command() {
    let results =
        run_case("verify_lockfile() {\n    cargo metadata --locked\n}\nverify_lockfile\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            title: Some("`.githooks/pre-commit` runs a concrete lockfile integrity command"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_cargo_metadata_locked_command_is_echoed() {
    let results = run_case("echo \"cargo metadata --locked\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("missing concrete lockfile integrity command in `.githooks/pre-commit`"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
