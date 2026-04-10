use g3rs_hooks_rs_source_checks_assertions::hook_rs_07_duplication_tool_is_cargo_dupes as assertions;

use crate::hook_rs_07_duplication_tool_is_cargo_dupes::run_case;

#[test]
fn warns_when_only_jscpd_exists() {
    let results = run_case("jscpd .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("wrong Rust duplication tool"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_cargo_dupes_exists() {
    let results = run_case("cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_dupes_binary_exists() {
    let results = run_case("cargo-dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_cargo_dupes_exists() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo dupes --exclude-tests); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_is_followed_by_same_line_command_chain() {
    let results = run_case("cargo dupes --exclude-tests && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_precedes_same_line_jscpd_chain() {
    let results = run_case("cargo dupes --exclude-tests && jscpd .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_follows_semicolon_chain() {
    let results = run_case("cd \"$RUST_WORKSPACE\"; cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_substitution_runs_cargo_dupes() {
    let results = run_case("DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exported_command_substitution_runs_cargo_dupes() {
    let results = run_case("export DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_command_substitution_runs_cargo_dupes() {
    let results = run_case("DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_declare_command_substitution_runs_cargo_dupes() {
    let results = run_case("declare DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_dupes_exists() {
    let results = run_case("cargo +nightly dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_dupes() {
    let results = run_case("env CARGO_TERM_COLOR=always cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_flag_with_value_wraps_cargo_dupes() {
    let results = run_case("env -u NODE_OPTIONS cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_chdir_wraps_cargo_dupes() {
    let results = run_case("env --chdir /repo cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_split_string_env_runs_cargo_dupes() {
    let results = run_case("env -S 'cargo dupes --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_long_form_split_string_env_runs_cargo_dupes() {
    let results = run_case("env --split-string 'cargo dupes --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_long_form_split_string_assignments_precede_cargo_dupes() {
    let results =
        run_case("env --split-string 'RUSTFLAGS=-D warnings' cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_dupes() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_runs_dupes() {
    let results = run_case("/usr/bin/cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_dupes_binary_runs() {
    let results = run_case("/opt/homebrew/bin/cargo-dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_path_qualified_dupes_binary() {
    let results = run_case("env -i /opt/homebrew/bin/cargo-dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_toolchain_prefixed_cargo_runs_dupes() {
    let results = run_case("/Users/me/.cargo/bin/cargo +nightly dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_assignment_and_toolchain_wrap_cargo_dupes() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo +nightly dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_precedes_dupes() {
    let results = run_case("cargo --locked dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_global_flag_with_value_precedes_dupes() {
    let results = run_case("cargo --manifest-path Cargo.toml dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_short_cargo_global_flag_with_value_precedes_dupes() {
    let results = run_case("cargo -Z unstable-options dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_wins_over_jscpd_if_both_exist() {
    let results = run_case("jscpd .\ncargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_binary_wins_over_jscpd_if_both_exist() {
    let results = run_case("jscpd .\ncargo-dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wrapped_dupes_wins_over_jscpd_if_both_exist() {
    let results = run_case("jscpd .\nenv CARGO_TERM_COLOR=always cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_jscpd_precedes_same_line_dupes_chain() {
    let results = run_case("jscpd . && cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_brace_group_runs_cargo_dupes() {
    let results = run_case("{ cargo dupes --exclude-tests; }\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_runs_on_right_hand_or_chain() {
    let results = run_case("cargo fmt --check || cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_dupes_wins_on_right_hand_or_chain_after_jscpd() {
    let results = run_case("jscpd . || cargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_dupes_appears_after_unrelated_commands() {
    let results = run_case("cargo fmt --check\necho 'dupes'\ncargo dupes --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_no_duplication_tool_is_present() {
    let results = run_case("cargo fmt --check\ncargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_is_only_echoed() {
    let results = run_case("echo \"cargo dupes --exclude-tests\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_is_only_commented() {
    let results = run_case("# cargo dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_dupes_command_is_used() {
    let results = run_case("cargo dupesx --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_other_cargo_subcommand_exists_but_not_dupes() {
    let results = run_case("cargo nextest run\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dupes_command_is_only_help() {
    let results = run_case("cargo dupes --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_dupes() {
    let results = run_case("cargo --help dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_version_precedes_dupes() {
    let results = run_case("cargo --version dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_help_wraps_dupes() {
    let results = run_case("env --help cargo dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_split_string_env_wraps_dupes_help() {
    let results = run_case("env -S 'cargo dupes --help'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dupes_binary_is_only_help() {
    let results = run_case("cargo-dupes --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_long_form_split_string_wraps_dupes_version() {
    let results = run_case("env --split-string 'cargo-dupes --version'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dupes_only_appears_in_assignment_text() {
    let results = run_case("DUPES_CMD='cargo dupes --exclude-tests'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dupes_only_appears_in_export_assignment() {
    let results = run_case("export DUPES_CMD='cargo-dupes --exclude-tests'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_only_echoed_dupes_exists_but_real_jscpd_runs() {
    let results = run_case("echo \"cargo dupes --exclude-tests\"\njscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_path_qualified_jscpd_exists() {
    let results = run_case("/usr/bin/jscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_env_wraps_jscpd() {
    let results = run_case("env NODE_OPTIONS=--no-deprecation jscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_jscpd_is_only_echoed() {
    let results = run_case("echo \"jscpd .\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_quoted_separator_precedes_dupes_text() {
    let results = run_case("echo \"note; cargo dupes --exclude-tests\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_jscpd_is_only_commented() {
    let results = run_case("# jscpd .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_quoted_separator_precedes_jscpd_text() {
    let results = run_case("echo \"note; jscpd .\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_jscpd_only_appears_in_assignment_text() {
    let results = run_case("JSCPD_CMD='jscpd .'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_jscpd_command_is_used() {
    let results = run_case("jscpdx .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_flag_with_value_wraps_jscpd() {
    let results = run_case("env -u NODE_OPTIONS jscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_env_chdir_wraps_jscpd() {
    let results = run_case("env --chdir /repo jscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_split_string_env_wraps_jscpd() {
    let results = run_case("env --split-string 'NODE_OPTIONS=--trace-warnings' jscpd .\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_split_string_env_runs_jscpd() {
    let results = run_case("env --split-string 'jscpd .'\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_split_string_env_wraps_jscpd_help() {
    let results = run_case("env -S 'jscpd --help'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_wrapped_jscpd_exists() {
    let results = run_case("if ! jscpd .; then\n    exit 1\nfi\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_command_substitution_runs_jscpd() {
    let results = run_case("JSCPD_OUTPUT=$(jscpd .)\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_local_command_substitution_runs_jscpd() {
    let results = run_case("local JSCPD_OUTPUT=$(jscpd .)\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_brace_group_runs_jscpd() {
    let results = run_case("{ jscpd .; }\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_quoted_command_substitution_runs_jscpd() {
    let results = run_case("JSCPD_OUTPUT=\"$(jscpd .)\"\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_readonly_command_substitution_runs_jscpd() {
    let results = run_case("readonly JSCPD_OUTPUT=\"$(jscpd .)\"\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_jscpd_is_only_help() {
    let results = run_case("jscpd --help\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_help_wraps_jscpd() {
    let results = run_case("env --help jscpd .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_real_jscpd_and_help_only_dupes_binary_both_exist() {
    let results = run_case("jscpd .\ncargo-dupes --help\n");
    assertions::assert_wrong_tool(&results);
}

#[test]
fn warns_when_real_jscpd_and_assignment_only_dupes_both_exist() {
    let results = run_case("jscpd .\nDUPES_CMD='cargo dupes --exclude-tests'\n");
    assertions::assert_wrong_tool(&results);
}
