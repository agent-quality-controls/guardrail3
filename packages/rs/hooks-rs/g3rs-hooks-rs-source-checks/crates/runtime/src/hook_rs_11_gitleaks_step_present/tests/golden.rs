use g3rs_hooks_rs_source_checks_assertions::hook_rs_11_gitleaks_step_present as assertions;

use crate::hook_rs_11_gitleaks_step_present::run_case;

#[test]
fn warns_when_gitleaks_only_appears_in_comment() {
    let results = run_case("# gitleaks protect --staged --no-banner\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("gitleaks step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_gitleaks_command_exists() {
    let results = run_case("gitleaks protect --staged --no-banner\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_gitleaks_command() {
    let results = run_case("env -i gitleaks protect --staged --no-banner\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_wraps_gitleaks_command() {
    let results = run_case("env -S 'gitleaks protect --staged --no-banner'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_gitleaks_command_exists() {
    let results = run_case("/usr/local/bin/gitleaks protect --staged --no-banner\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_option_value_precedes_gitleaks_script() {
    let results = run_case("sh -o errexit -c 'gitleaks protect --staged --no-banner'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_gitleaks() {
    let results = run_case("bash -lc 'gitleaks protect --staged --no-banner'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_wrapper_runs_gitleaks() {
    let results = run_case("command gitleaks protect --staged --no-banner\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_gitleaks() {
    let results = run_case("exec gitleaks protect --staged --no-banner\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_function_runs_gitleaks() {
    let results =
        run_case("run_gitleaks() {\n    gitleaks protect --staged --no-banner\n}\nrun_gitleaks\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_helper_chain_runs_gitleaks() {
    let results = run_case(
        "run_gitleaks() {\n    gitleaks protect --staged --no-banner\n}\nprecommit_checks() {\n    run_gitleaks\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_left_side_of_chained_command_runs_gitleaks() {
    let results = run_case("gitleaks protect --staged --no-banner && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_gitleaks_only_appears_in_echo() {
    let results = run_case("echo \"gitleaks protect --staged --no-banner\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_gitleaks_command_is_only_help() {
    let results = run_case("gitleaks --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_gitleaks_command_is_only_version() {
    let results = run_case("gitleaks --version\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_gitleaks_only_exists_inside_uncalled_function() {
    let results = run_case("run_gitleaks() {\n    gitleaks protect --staged --no-banner\n}\n");
    assertions::assert_missing(&results);
}
