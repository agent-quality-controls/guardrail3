use g3rs_hooks_rs_source_checks_assertions::hook_rs_10_test_uses_workspace as assertions;

use crate::hook_rs_10_test_uses_workspace::{run_case, run_case_with_workspace};

#[test]
fn reports_info_when_workspace_flag_missing() {
    let results = run_case("cargo test\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Info),
            inventory: Some(false),
            title: Some("cargo test missing --workspace"),
            ..Default::default()
        }],
    );
}

#[test]
fn stays_inventory_only_when_repo_is_not_a_workspace_project() {
    let results = run_case_with_workspace("cargo test\n", false);
    assertions::assert_not_required(&results);
}

#[test]
fn passes_when_workspace_flag_exists() {
    let results = run_case("cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_workspace_test_exists() {
    let results = run_case("cargo +nightly test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_workspace_test() {
    let results = run_case("env -i cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_wraps_workspace_test() {
    let results = run_case("env -S 'cargo test --workspace'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_workspace_test() {
    let results = run_case("/usr/bin/cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_manifest_path_precedes_workspace_test() {
    let results = run_case("cargo --manifest-path Cargo.toml test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_chdir_precedes_workspace_test() {
    let results = run_case("cargo -C tools test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_attached_jobs_flag_precedes_workspace_test() {
    let results = run_case("cargo -j4 test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_workspace_test() {
    let results = run_case("bash -lc 'cargo test --workspace'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_option_value_precedes_workspace_test_script() {
    let results = run_case("sh -o errexit -c 'cargo test --workspace'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_wrapper_runs_workspace_test() {
    let results = run_case("command cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_workspace_test() {
    let results = run_case("exec cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_function_runs_workspace_test() {
    let results = run_case("run_tests() {\n    cargo test --workspace\n}\nrun_tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_helper_chain_runs_workspace_test() {
    let results = run_case(
        "run_tests() {\n    cargo test --workspace\n}\nprecommit_checks() {\n    run_tests\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_workspace_test_is_left_side_of_chained_command() {
    let results = run_case("cargo test --workspace && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn does_not_count_echoed_workspace_command() {
    let results = run_case("echo \"cargo test --workspace\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_test_subcommand_is_used() {
    let results = run_case("cargo testx --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_workspace_flag_is_only_forwarded_to_test_binary() {
    let results = run_case("cargo test -- --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_workspace_flag_is_only_near_match() {
    let results = run_case("cargo test --workspace-hack\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_workspace_test_command_is_only_help() {
    let results = run_case("cargo test --workspace --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_global_help_precedes_workspace_test() {
    let results = run_case("cargo --help test --workspace\n");
    assertions::assert_missing(&results);
}
