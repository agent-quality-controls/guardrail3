use g3rs_hooks_source_checks_assertions::hook_rs_03_cargo_deny_step_present::rule as assertions;

use super::super::run_case;

#[test]
fn warns_when_cargo_deny_only_appears_in_comment() {
    let results = run_case("# cargo deny check\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("cargo deny check step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_cargo_deny_command_exists() {
    let results = run_case("cargo deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_deny_hyphen_binary_exists() {
    let results = run_case("cargo-deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_deny_check_exists() {
    let results = run_case("cargo +nightly deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_deny_check() {
    let results = run_case("env CARGO_TERM_COLOR=always cargo deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_deny_check() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_deny_check() {
    let results = run_case("/Users/me/.cargo/bin/cargo deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_deny_check() {
    let results = run_case("cargo --locked deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_global_flag_deny_check() {
    let results = run_case("env -i cargo --locked deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_cargo_deny_check_exists() {
    let results =
        run_case("if ! (cd \"$RUST_WORKSPACE\" && cargo deny check); then\n    exit 1\nfi\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_deny_check_appears_after_invalid_deny_line() {
    let results = run_case("cargo deny init\ncargo deny check\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_cargo_deny_is_only_echoed() {
    let results = run_case("echo \"cargo deny check\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_deny_only_appears_in_assignment_text() {
    let results = run_case("DENY_LABEL=\"cargo deny check\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_deny_only_appears_in_export_assignment() {
    let results = run_case("export DENY_CMD=\"cargo deny check\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_executable_commands_exist_but_not_deny_check() {
    let results = run_case("cargo fmt --check\ncargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_non_check_cargo_deny_command_is_used() {
    let results = run_case("cargo deny init\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_deny_check_is_only_help() {
    let results = run_case("cargo deny check --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_non_check_hyphen_binary_command_is_used() {
    let results = run_case("cargo-deny --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_deny_binary_check_is_only_help() {
    let results = run_case("cargo-deny check --help\n");
    assertions::assert_missing(&results);
}
