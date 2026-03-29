use guardrail3_app_rs_family_hooks_rs_assertions::hook_rs_13_cargo_dupes_excludes as assertions;

use crate::hook_rs_13_cargo_dupes_excludes::run_case;

#[test]
fn warns_when_cargo_dupes_is_only_prose() {
    let results = run_case("echo \"cargo dupes check --exclude-tests\"\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            inventory: Some(false),
            title: Some("cargo dupes step does not exclude tests"),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_non_inventory_when_exclude_tests_flag_missing() {
    let results = run_case("cargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_cargo_dupes_subcommand_has_exclude_tests() {
    let results = run_case("cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_dupes_uses_template_threshold_flags_with_exclude_tests() {
    let results =
        run_case("cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_cargo_dupes_binary_has_exclude_tests() {
    let results = run_case("cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_cargo_dupes_subcommand_with_exclude_tests() {
    let results = run_case("env -i cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_assignment_precedes_cargo_dupes_subcommand() {
    let results = run_case("env CARGO_TERM_COLOR=always cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_unset_precedes_cargo_dupes_subcommand() {
    let results = run_case("env -u NODE_OPTIONS cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_chdir_precedes_cargo_dupes_subcommand() {
    let results = run_case("env --chdir /repo cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_wraps_cargo_dupes_with_exclude_tests() {
    let results = run_case("env -S 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_long_split_string_wraps_cargo_dupes_with_exclude_tests() {
    let results = run_case("env --split-string 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_assignment_only_payload_precedes_cargo_dupes() {
    let results = run_case(
        "env --split-string 'RUSTFLAGS=-D warnings' cargo dupes check --exclude-tests\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_equals_wraps_cargo_dupes_with_exclude_tests() {
    let results = run_case("env --split-string='cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_env_wrapper_uses_unknown_flag() {
    let results = run_case("env --bogus cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_leading_env_assignment_precedes_cargo_dupes() {
    let results = run_case("RUSTFLAGS='-D warnings' cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_unknown_cargo_global_flag_precedes_dupes_subcommand() {
    let results = run_case("cargo --bogus dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_dupes_has_exclude_tests() {
    let results = run_case("cargo +nightly dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_attached_manifest_path_precedes_cargo_dupes_subcommand() {
    let results = run_case("cargo --manifest-path=Cargo.toml dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_subcommand_has_exclude_tests() {
    let results = run_case("/usr/bin/cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_binary_has_exclude_tests() {
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
fn warns_when_shell_wrapper_uses_unknown_flag_before_c_script() {
    let results = run_case("bash --bogus -lc 'cargo dupes check --exclude-tests'\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("bash -lc 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_clustered_shell_short_flags_wrap_cargo_dupes_with_exclude_tests() {
    let results = run_case("sh -euxc 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_multiline_cargo_dupes_script() {
    let results = run_case("bash -lc 'echo setup\ncargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_helper_function_inside_payload() {
    let results = run_case(
        "bash -lc 'run_dupes() {\n    cargo dupes check --exclude-tests\n}\nrun_dupes'\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_option_value_precedes_cargo_dupes_script() {
    let results = run_case("sh -o errexit -c 'cargo dupes check --exclude-tests'\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_binary_starts_with_unknown_flag_before_subcommand() {
    let results = run_case("cargo-dupes --bogus --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_binary_has_exclude_tests_but_no_subcommand() {
    let results = run_case("cargo-dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_subcommand_has_unknown_flag_after_subcommand() {
    let results = run_case("cargo dupes check --bogus --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_binary_has_unknown_flag_after_subcommand() {
    let results = run_case("cargo-dupes check --bogus --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_command_wrapper_runs_cargo_dupes_binary() {
    let results = run_case("command cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_command_wrapper_runs_cargo_dupes_without_exact_flag() {
    let results = run_case("command cargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_env_double_dash_runs_cargo_dupes() {
    let results = run_case("env -- cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_nested_wrappers_run_cargo_dupes_with_exclude_tests() {
    let results = run_case("env command cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_nested_wrappers_run_cargo_dupes_without_exact_flag() {
    let results = run_case("env command cargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_command_double_dash_runs_cargo_dupes() {
    let results = run_case("command -- cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_p_runs_cargo_dupes() {
    let results = run_case("command -p cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_exec_wrapper_uses_unknown_flag() {
    let results = run_case("exec --bogus cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_cargo_dupes_subcommand() {
    let results = run_case("exec cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_cargo_dupes_binary() {
    let results = run_case("exec cargo-dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_exec_wrapper_runs_cargo_dupes_without_exact_flag() {
    let results = run_case("exec cargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_exec_double_dash_runs_cargo_dupes() {
    let results = run_case("exec -- cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_function_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("run_dupes() {\n    cargo dupes check --exclude-tests\n}\nrun_dupes\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_recursive_helper_never_reaches_cargo_dupes() {
    let results = run_case("run_dupes() {\n    run_dupes\n}\nrun_dupes\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_called_helper_chain_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case(
        "run_dupes() {\n    cargo dupes check --exclude-tests\n}\nprecommit_checks() {\n    run_dupes\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_nested_helper_defined_inside_called_function_runs_cargo_dupes() {
    let results = run_case(
        "precommit_checks() {\n    run_dupes() {\n        cargo dupes check --exclude-tests\n    }\n    run_dupes\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_same_line_helper_definition_runs_cargo_dupes() {
    let results = run_case("run_dupes() { cargo dupes check --exclude-tests; }; run_dupes\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_top_level_helper_is_called_before_its_definition() {
    let results = run_case(
        "precommit_checks\nprecommit_checks() {\n    cargo dupes check --exclude-tests\n}\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_nested_helper_is_called_before_its_definition_inside_called_function() {
    let results = run_case(
        "precommit_checks() {\n    run_dupes\n    run_dupes() {\n        cargo dupes check --exclude-tests\n    }\n}\nprecommit_checks\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_nested_helper_shadows_compliant_root_helper() {
    let results = run_case(
        "run_dupes() {\n    cargo dupes check --exclude-tests\n}\nprecommit_checks() {\n    run_dupes() {\n        cargo dupes check\n    }\n    run_dupes\n}\nprecommit_checks\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_left_side_of_chained_command_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("cargo dupes check --exclude-tests && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_right_side_of_true_and_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("true && cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_left_side_of_pipeline_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("cargo dupes check --exclude-tests | tee /tmp/dupes.log\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_semicolon_precedes_cargo_dupes_with_exclude_tests() {
    let results = run_case("echo setup; cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_backgrounded_cargo_dupes_runs_with_exclude_tests() {
    let results = run_case("cargo dupes check --exclude-tests & wait\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_dead_right_side_of_false_and_is_only_cargo_dupes_occurrence() {
    let results = run_case("false && cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dead_right_side_of_true_or_is_only_cargo_dupes_occurrence() {
    let results = run_case("true || cargo-dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_exit_zero_prevents_later_and_command() {
    let results = run_case("exit 0 && cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_exit_one_prevents_later_or_command() {
    let results = run_case("exit 1 || cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_right_side_of_false_or_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("false || cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_if_wrapper_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo dupes check --exclude-tests); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_substitution_runs_cargo_dupes_with_exclude_tests() {
    let results = run_case("OUTPUT=$(cargo dupes check --exclude-tests)\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_double_quoted_assignment_substitution_runs_cargo_dupes() {
    let results = run_case("OUTPUT=\"$(cargo dupes check --exclude-tests)\"\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_double_quoted_argument_substitution_runs_cargo_dupes() {
    let results = run_case("echo \"$(cargo dupes check --exclude-tests)\"\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_export_assignment_command_substitution_runs_cargo_dupes() {
    let results = run_case("export DUPES_OUTPUT=$(cargo dupes check --exclude-tests)\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_declare_assignment_command_substitution_runs_cargo_dupes() {
    let results = run_case("declare DUPES_OUTPUT=\"$(cargo dupes check --exclude-tests)\"\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_dead_command_substitution_is_only_cargo_dupes_occurrence() {
    let results = run_case("OUTPUT=$(false && cargo dupes check --exclude-tests)\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_hook_contains_mixed_compliant_and_noncompliant_cargo_dupes() {
    let results = run_case("cargo dupes check --exclude-tests\ncargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_single_quoted_command_substitution_literal_is_only_occurrence() {
    let results = run_case("echo '$(cargo dupes check --exclude-tests)'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_escaped_command_substitution_literal_is_only_occurrence() {
    let results = run_case("echo \\$(cargo dupes check --exclude-tests)\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_exclude_tests_only_appears_in_later_command() {
    let results = run_case("cargo dupes check && echo --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_exclude_tests_only_appears_in_inline_comment() {
    let results = run_case("cargo dupes check # --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_inline_comment_mentions_help_after_real_flag() {
    let results = run_case("cargo dupes check --exclude-tests # --help\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_env_wrapper_runs_cargo_dupes_without_exact_flag() {
    let results = run_case("env -i cargo dupes check\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_env_wrapper_bypasses_shadowed_cargo_function() {
    let results =
        run_case("cargo() {\n    echo fake\n}\nenv -i cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_shell_wrapper_runs_cargo_dupes_without_exact_flag() {
    let results = run_case("bash -lc 'cargo dupes check'\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_command_wrapper_bypasses_shadowed_cargo_function() {
    let results =
        run_case("cargo() {\n    echo fake\n}\ncommand cargo dupes check --exclude-tests\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_command_lookup_only_mentions_cargo_dupes() {
    let results = run_case("command -v cargo-dupes --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_subcommand_is_only_help() {
    let results = run_case("cargo dupes --help --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_help_precedes_dupes_subcommand() {
    let results = run_case("cargo --help dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_global_version_precedes_dupes_subcommand() {
    let results = run_case("cargo --version dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_binary_is_only_help() {
    let results = run_case("cargo-dupes --help --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_help_only_wraps_cargo_dupes_command() {
    let results = run_case("env --help cargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_subcommand_only_passes_exclude_tests_after_double_dash() {
    let results = run_case("cargo dupes check -- --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_binary_only_passes_exclude_tests_after_double_dash() {
    let results = run_case("cargo-dupes check -- --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_attached_exclude_tests_value_is_used() {
    let results = run_case("cargo dupes check --exclude-tests=true\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_near_match_cargo_dupes_subcommand_is_used() {
    let results = run_case("cargo dupesx check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_called_function_runs_cargo_dupes_without_exact_flag() {
    let results = run_case("run_dupes() {\n    cargo dupes check\n}\nrun_dupes\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_function_shadows_real_binary() {
    let results = run_case("cargo() {\n    echo fake\n}\ncargo dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_function_shadows_real_binary() {
    let results =
        run_case("cargo-dupes() {\n    echo fake\n}\ncargo-dupes check --exclude-tests\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_cargo_dupes_only_exists_inside_uncalled_function() {
    let results = run_case("run_dupes() {\n    cargo dupes check --exclude-tests\n}\n");
    assertions::assert_missing(&results);
}
