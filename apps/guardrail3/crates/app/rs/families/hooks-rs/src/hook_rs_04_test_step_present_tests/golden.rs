use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_04_test_step_present as assertions;

use crate::hook_rs_04_test_step_present::run_case;

#[test]
fn warns_when_test_only_appears_in_comment() {
    let results = run_case("# cargo test --workspace\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            inventory: Some(false),
            title: Some("cargo test step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_test_command_exists() {
    let results = run_case("cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_plain_test_command_exists() {
    let results = run_case("cargo test\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_test_command_exists() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo test --workspace); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_test_command_exists() {
    let results = run_case("cargo +nightly test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_test() {
    let results = run_case("env CARGO_TERM_COLOR=always cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_flag_wraps_toolchain_prefixed_cargo_test() {
    let results = run_case("env -i cargo +nightly test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_test() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_test() {
    let results = run_case("/usr/bin/cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_test() {
    let results = run_case("cargo --locked test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_path_qualified_cargo_test() {
    let results = run_case("env -i /usr/bin/cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_toolchain_prefixed_cargo_runs_test() {
    let results = run_case("/Users/me/.cargo/bin/cargo +nightly test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_test_is_only_echoed() {
    let results = run_case("echo \"cargo test --workspace\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_executable_commands_exist_but_not_test() {
    let results = run_case("cargo fmt --check\ncargo clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_cargo_subcommand_exists_but_not_test() {
    let results = run_case("cargo nextest run\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_test_command_is_used() {
    let results = run_case("cargo testx --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_valid_test_command_appears_after_near_match() {
    let results = run_case("cargo testx --workspace\ncargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_test_command_appears_after_unrelated_commands() {
    let results = run_case("cargo fmt --check\necho 'running tests'\ncargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_test_only_appears_in_assignment_text() {
    let results = run_case("TEST_CMD='cargo test --workspace'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_test_command_is_only_help() {
    let results = run_case("cargo test --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_test() {
    let results = run_case("cargo --help test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_test_only_appears_in_export_assignment() {
    let results = run_case("export TEST_CMD='cargo test --workspace'\n");
    assertions::assert_missing(&results);
}
