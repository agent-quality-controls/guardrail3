use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_12_cargo_dupes_step_present as assertions;

use crate::hook_rs_12_cargo_dupes_step_present::run_case;

#[test]
fn warns_when_cargo_dupes_is_only_prose() {
    let results = run_case("echo \"cargo dupes check\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            inventory: Some(false),
            title: Some("cargo dupes step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_cargo_dupes_subcommand_exists() {
    let results = run_case("cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_dupes_binary_exists() {
    let results = run_case("cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_dupes_subcommand() {
    let results = run_case("env -i cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_wraps_cargo_dupes_subcommand() {
    let results = run_case("env -S 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_dupes_exists() {
    let results = run_case("cargo +nightly dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_subcommand_exists() {
    let results = run_case("/usr/bin/cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_binary_exists() {
    let results = run_case("/usr/local/bin/cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_manifest_path_precedes_cargo_dupes_subcommand() {
    let results = run_case("cargo --manifest-path Cargo.toml dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_attached_jobs_flag_precedes_cargo_dupes_subcommand() {
    let results = run_case("cargo -j4 dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_cargo_dupes() {
    let results = run_case("bash -lc 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_option_value_precedes_cargo_dupes_script() {
    let results = run_case("sh -o errexit -c 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_wrapper_runs_cargo_dupes_binary() {
    let results = run_case("command cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_cargo_dupes_binary() {
    let results = run_case("exec cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_function_runs_cargo_dupes() {
    let results = run_case("run_dupes() {\n    cargo dupes check --exclude-tests\n}\nrun_dupes\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_helper_chain_runs_cargo_dupes() {
    let results = run_case(
        "run_dupes() {\n    cargo dupes check --exclude-tests\n}\nprecommit_checks() {\n    run_dupes\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_left_side_of_chained_command_runs_cargo_dupes() {
    let results = run_case("cargo dupes check --exclude-tests && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_cargo_dupes_subcommand_is_only_help() {
    let results = run_case("cargo dupes --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_binary_is_only_help() {
    let results = run_case("cargo-dupes --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_cargo_dupes_subcommand_is_used() {
    let results = run_case("cargo dupesx check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_only_exists_inside_uncalled_function() {
    let results = run_case("run_dupes() {\n    cargo dupes check --exclude-tests\n}\n");
    assertions::assert_missing(&results);
}
