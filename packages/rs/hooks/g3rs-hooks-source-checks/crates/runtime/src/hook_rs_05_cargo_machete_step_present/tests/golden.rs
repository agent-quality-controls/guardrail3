use g3rs_hooks_source_checks_assertions::hook_rs_05_cargo_machete_step_present as assertions;

use crate::hook_rs_05_cargo_machete_step_present::run_case;

#[test]
fn warns_when_machete_only_appears_in_comment() {
    let results = run_case("# cargo machete\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("cargo machete step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_machete_command_exists() {
    let results = run_case("cargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_machete_binary_exists() {
    let results = run_case("cargo-machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_machete_command_exists() {
    let results =
        run_case("if ! (cd \"$RUST_WORKSPACE\" && cargo machete); then\n    exit 1\nfi\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_machete_command_exists() {
    let results = run_case("cargo +nightly machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_machete() {
    let results = run_case("env CARGO_TERM_COLOR=always cargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_machete() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_machete() {
    let results = run_case("/usr/bin/cargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_hyphen_binary_runs() {
    let results = run_case("/opt/homebrew/bin/cargo-machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_machete() {
    let results = run_case("cargo --locked machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_path_qualified_machete_binary() {
    let results = run_case("env -i /opt/homebrew/bin/cargo-machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_toolchain_prefixed_cargo_runs_machete() {
    let results = run_case("/Users/me/.cargo/bin/cargo +nightly machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_assignment_and_toolchain_wrap_cargo_machete() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo +nightly machete\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_machete_is_only_echoed() {
    let results = run_case("echo \"cargo machete\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_executable_commands_exist_but_not_machete() {
    let results = run_case("cargo fmt --check\ncargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_cargo_subcommand_exists_but_not_machete() {
    let results = run_case("cargo nextest run\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_machete_command_is_used() {
    let results = run_case("cargo machetex\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_valid_machete_command_appears_after_near_match() {
    let results = run_case("cargo machetex\ncargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_machete_command_appears_after_unrelated_commands() {
    let results = run_case("cargo fmt --check\necho 'running machete'\ncargo machete\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_machete_only_appears_in_assignment_text() {
    let results = run_case("MACHETE_CMD='cargo machete'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_machete_only_appears_in_export_assignment() {
    let results = run_case("export MACHETE_CMD='cargo-machete'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_machete_command_is_only_help() {
    let results = run_case("cargo machete --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_machete() {
    let results = run_case("cargo --help machete\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_machete_binary_is_only_help() {
    let results = run_case("cargo-machete --help\n");
    assertions::assert_missing(&results);
}
