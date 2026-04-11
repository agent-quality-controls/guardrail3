use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_21_no_fail_open_wrappers as assertions;

use super::run_case;

#[test]
fn reports_fail_open_wrapper_on_critical_command() {
    let results = run_case("g3rs rs validate --staged . || true\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("critical hook command is fail-open"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_fail_open_wrapper_on_env_wrapped_critical_command() {
    let results = run_case("env -i cargo test --workspace || true\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("critical hook command is fail-open"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_fail_open_wrapper_on_path_qualified_critical_command() {
    let results = run_case("/usr/local/bin/g3rs rs validate --staged . || :\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("critical hook command is fail-open"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_fail_open_wrapper_on_called_function_that_runs_critical_command() {
    let results = run_case(
        "run_tests() {\n    cargo test --workspace\n}\nrun_tests || true\n",
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("critical hook command is fail-open"),
            line: Some(4),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_fail_open_wrapper_on_non_critical_command() {
    let results = run_case("grep -q needle file || true\n");
    assertions::assert_rule_quiet(&results);
}

#[test]
fn ignores_echoed_critical_command_with_literal_fail_open_text() {
    let results = run_case("echo \"g3rs rs validate --staged . || true\"\n");
    assertions::assert_rule_quiet(&results);
}
