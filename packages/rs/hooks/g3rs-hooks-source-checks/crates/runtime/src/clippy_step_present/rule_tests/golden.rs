use g3rs_hooks_source_checks_assertions::clippy_step_present::rule as assertions;

use super::super::run_case;

#[test]
fn warns_when_clippy_only_appears_in_comment() {
    let results = run_case("# cargo clippy --workspace\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("cargo clippy step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_clippy_command_exists() {
    let results = run_case("cargo clippy --workspace --all-targets\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_clippy_command_exists() {
    let results = run_case("cargo +nightly clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_clippy() {
    let results = run_case("env RUSTFLAGS=-Dwarnings cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_clippy() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_with_flag_wraps_cargo_clippy() {
    let results = run_case("env -i cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_clippy() {
    let results = run_case("/Users/me/.cargo/bin/cargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_toolchain_prefixed_cargo_runs_clippy() {
    let results = run_case("/Users/me/.cargo/bin/cargo +nightly clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_clippy() {
    let results = run_case("cargo --locked clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_global_flag_clippy() {
    let results = run_case("env -i cargo --locked clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_clippy_command_exists() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo clippy --workspace --all-targets); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_clippy_is_only_echoed() {
    let results = run_case("echo \"cargo clippy --workspace\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_only_appears_in_assignment_text() {
    let results = run_case("CLIPPY_LABEL=\"cargo clippy --workspace\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_only_appears_in_export_assignment() {
    let results = run_case("export CLIPPY_CMD=\"cargo clippy --workspace\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_cargo_command_exists_but_not_clippy() {
    let results = run_case("cargo fmt --all -- --check\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_clippy_subcommand_is_used() {
    let results = run_case("cargo clippy-driver --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_clippy_command_is_only_help() {
    let results = run_case("cargo clippy --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_clippy() {
    let results = run_case("cargo --help clippy --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_clippy_appears_after_unrelated_commands() {
    let results = run_case("cargo test --workspace\necho warmup\ncargo clippy --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_clippy_appears_after_near_match_line() {
    let results = run_case("cargo clippy-driver --workspace\ncargo clippy --workspace\n");
    assertions::assert_present(&results);
}
