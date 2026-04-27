use g3rs_hooks_source_checks_assertions::shared_target_dir_present::rule as assertions;

use super::super::run_case;

#[test]
fn warns_when_cargo_runs_without_shared_target_dir() {
    let results = run_case("cargo test --workspace\ncargo clippy --workspace -- -D warnings\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_exported_target_dir_precedes_cargo() {
    let results =
        run_case("export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\ncargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_export_happens_after_cargo() {
    let results =
        run_case("cargo test --workspace\nexport CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_standalone_shell_variable_is_not_exported() {
    let results =
        run_case("CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\ncargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_inline_assignment_wraps_cargo() {
    let results =
        run_case("CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_target_dir_for_cargo() {
    let results =
        run_case("env -i CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_target_dir_for_cargo_with_unrelated_assignment() {
    let results = run_case(
        "env -i FOO=bar CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_target_dir_assignment_only_appears_after_cargo_args() {
    let results =
        run_case("cargo test --workspace CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_same_line_export_is_canceled_before_cargo() {
    let results = run_case(
        "export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"; unset CARGO_TARGET_DIR; cargo test --workspace\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_same_line_unset_is_followed_by_inline_assignment() {
    let results = run_case(
        "unset CARGO_TARGET_DIR; CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_function_exports_target_dir_before_cargo() {
    let results = run_case(
        "run_checks() {\n    export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n    cargo test --workspace\n}\nrun_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_nested_function_sets_target_dir_before_cargo() {
    let results = run_case(
        "outer() {\n    inner\n}\ninner() {\n    export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n    cargo test --workspace\n}\nouter\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_wrapper_function_sets_target_dir() {
    let results = run_case(
        "cargo() {\n    export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n    command cargo \"$@\"\n}\ncargo test --workspace\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_target_dir_only_appears_in_comment_or_echo() {
    let results = run_case(
        "# export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\necho \"export CARGO_TARGET_DIR=foo\"\ncargo test --workspace\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_echoed_target_dir_shares_line_with_cargo() {
    let results =
        run_case("echo \"CARGO_TARGET_DIR=$REPO_ROOT/.cargo-target\" && cargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_only_some_cargo_commands_are_covered_inline() {
    let results = run_case(
        "CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace\ncargo clippy --workspace -- -D warnings\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_sh_wrapper_payload_sets_target_dir() {
    let results = run_case(
        "sh -c 'export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"; cargo test --workspace'\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_payload_sets_target_dir() {
    let results = run_case(
        "bash -lc 'export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"; cargo test --workspace'\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_piped_cargo_runs_without_target_dir() {
    let results = run_case("cargo test --workspace | tee /tmp/hook.log\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_background_cargo_runs_without_target_dir() {
    let results = run_case("cargo test --workspace &\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_backtick_substitution_runs_uncovered_cargo() {
    let results = run_case("echo `cargo test --workspace`\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_backtick_substitution_runs_covered_cargo() {
    let results =
        run_case("echo `CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\" cargo test --workspace`\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_export_then_unset_before_later_cargo() {
    let results = run_case(
        "export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\nunset CARGO_TARGET_DIR\ncargo test --workspace\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_path_qualified_cargo_uses_exported_target_dir() {
    let results = run_case(
        "export CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\n/Users/me/.cargo/bin/cargo test --workspace\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn stays_quiet_when_hook_has_no_cargo_commands() {
    let results = run_case("gitleaks protect --staged --no-banner\n");
    assertions::assert_rule_quiet(&results);
}
