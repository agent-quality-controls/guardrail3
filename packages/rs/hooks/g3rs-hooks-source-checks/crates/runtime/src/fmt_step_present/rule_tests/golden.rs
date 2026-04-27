use g3rs_hooks_source_checks_assertions::fmt_step_present::rule as assertions;

use super::super::run_case;

#[test]
fn warns_when_fmt_only_appears_in_comment() {
    let results = run_case("# cargo fmt --all -- --check\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("cargo fmt --check step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_fmt_check_command_exists() {
    let results = run_case("cargo fmt --all -- --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_plain_fmt_check_command_exists() {
    let results = run_case("cargo fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_fmt_check_is_only_echoed() {
    let results = run_case("echo \"cargo fmt --all -- --check\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_fmt_command_is_missing_check_flag() {
    let results = run_case("cargo fmt --all\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_subcommand_is_used() {
    let results = run_case("cargo fmtx --check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_for_toolchain_prefixed_fmt_check() {
    let results = run_case("cargo +nightly fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_fmt_check_command_exists() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo fmt --all -- --check); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_fmt_check_appears_after_unrelated_commands() {
    let results = run_case("cargo test --workspace\necho warmup\ncargo fmt --all -- --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_fmt_check_appears_after_invalid_fmt_line() {
    let results = run_case("cargo fmt --all\ncargo fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_fmt_check() {
    let results = run_case("env RUSTFLAGS=-Dwarnings cargo fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_fmt_check() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_fmt_check() {
    let results = run_case("/Users/me/.cargo/bin/cargo fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_fmt() {
    let results = run_case("cargo --locked fmt --check\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_fmt_command_is_only_help() {
    let results = run_case("cargo fmt --help --check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_env_wraps_path_qualified_cargo_fmt_check() {
    let results = run_case("env -i /Users/me/.cargo/bin/cargo fmt --check\n");
    assertions::assert_present(&results);
}
