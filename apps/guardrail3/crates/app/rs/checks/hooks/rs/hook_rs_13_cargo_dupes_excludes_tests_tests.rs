use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;
use super::check;

#[test]
fn warns_when_cargo_dupes_is_only_prose() {
    let parsed = parsed_hook("echo \"cargo dupes check --exclude-tests\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn reports_non_inventory_when_exclude_tests_flag_missing() {
    let parsed = parsed_hook("cargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_subcommand_has_exclude_tests() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_uses_template_threshold_flags_with_exclude_tests() {
    let parsed =
        parsed_hook("cargo dupes check --max-exact 85 --max-exact-percent 10 --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_binary_has_exclude_tests() {
    let parsed = parsed_hook("cargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_cargo_dupes_subcommand_with_exclude_tests() {
    let parsed = parsed_hook("env -i cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_assignment_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("env CARGO_TERM_COLOR=always cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_unset_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("env -u NODE_OPTIONS cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_chdir_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("env --chdir /repo cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_split_string_wraps_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("env -S 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_long_split_string_wraps_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("env --split-string 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_split_string_assignment_only_payload_precedes_cargo_dupes() {
    let parsed = parsed_hook(
        "env --split-string 'RUSTFLAGS=-D warnings' cargo dupes check --exclude-tests\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_split_string_equals_wraps_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("env --split-string='cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_env_wrapper_uses_unknown_flag() {
    let parsed = parsed_hook("env --bogus cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_leading_env_assignment_precedes_cargo_dupes() {
    let parsed = parsed_hook("RUSTFLAGS='-D warnings' cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_unknown_cargo_global_flag_precedes_dupes_subcommand() {
    let parsed = parsed_hook("cargo --bogus dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_dupes_has_exclude_tests() {
    let parsed = parsed_hook("cargo +nightly dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_attached_manifest_path_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("cargo --manifest-path=Cargo.toml dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_subcommand_has_exclude_tests() {
    let parsed = parsed_hook("/usr/bin/cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_cargo_dupes_binary_has_exclude_tests() {
    let parsed = parsed_hook("/usr/local/bin/cargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_manifest_path_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("cargo --manifest-path Cargo.toml dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_attached_jobs_flag_precedes_cargo_dupes_subcommand() {
    let parsed = parsed_hook("cargo -j4 dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_shell_wrapper_uses_unknown_flag_before_c_script() {
    let parsed = parsed_hook("bash --bogus -lc 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_shell_wrapper_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("bash -lc 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_clustered_shell_short_flags_wrap_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("sh -euxc 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_wrapper_runs_multiline_cargo_dupes_script() {
    let parsed = parsed_hook("bash -lc 'echo setup\ncargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_wrapper_runs_helper_function_inside_payload() {
    let parsed = parsed_hook(
        "bash -lc 'run_dupes() {\n    cargo dupes check --exclude-tests\n}\nrun_dupes'\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_option_value_precedes_cargo_dupes_script() {
    let parsed = parsed_hook("sh -o errexit -c 'cargo dupes check --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_binary_starts_with_unknown_flag_before_subcommand() {
    let parsed = parsed_hook("cargo-dupes --bogus --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_binary_has_exclude_tests_but_no_subcommand() {
    let parsed = parsed_hook("cargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_subcommand_has_unknown_flag_after_subcommand() {
    let parsed = parsed_hook("cargo dupes check --bogus --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_binary_has_unknown_flag_after_subcommand() {
    let parsed = parsed_hook("cargo-dupes check --bogus --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_command_wrapper_runs_cargo_dupes_binary() {
    let parsed = parsed_hook("command cargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_command_wrapper_runs_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("command cargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_env_double_dash_runs_cargo_dupes() {
    let parsed = parsed_hook("env -- cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_nested_wrappers_run_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("env command cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_nested_wrappers_run_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("env command cargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_command_double_dash_runs_cargo_dupes() {
    let parsed = parsed_hook("command -- cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_command_p_runs_cargo_dupes() {
    let parsed = parsed_hook("command -p cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_exec_wrapper_uses_unknown_flag() {
    let parsed = parsed_hook("exec --bogus cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_exec_wrapper_runs_cargo_dupes_subcommand() {
    let parsed = parsed_hook("exec cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exec_wrapper_runs_cargo_dupes_binary() {
    let parsed = parsed_hook("exec cargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_exec_wrapper_runs_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("exec cargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_exec_double_dash_runs_cargo_dupes() {
    let parsed = parsed_hook("exec -- cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_called_function_runs_cargo_dupes_with_exclude_tests() {
    let parsed =
        parsed_hook("run_dupes() {\n    cargo dupes check --exclude-tests\n}\nrun_dupes\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_recursive_helper_never_reaches_cargo_dupes() {
    let parsed = parsed_hook("run_dupes() {\n    run_dupes\n}\nrun_dupes\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_called_helper_chain_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook(
        "run_dupes() {\n    cargo dupes check --exclude-tests\n}\nprecommit_checks() {\n    run_dupes\n}\nprecommit_checks\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_nested_helper_defined_inside_called_function_runs_cargo_dupes() {
    let parsed = parsed_hook(
        "precommit_checks() {\n    run_dupes() {\n        cargo dupes check --exclude-tests\n    }\n    run_dupes\n}\nprecommit_checks\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_same_line_helper_definition_runs_cargo_dupes() {
    let parsed = parsed_hook("run_dupes() { cargo dupes check --exclude-tests; }; run_dupes\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_top_level_helper_is_called_before_its_definition() {
    let parsed = parsed_hook(
        "precommit_checks\nprecommit_checks() {\n    cargo dupes check --exclude-tests\n}\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_nested_helper_is_called_before_its_definition_inside_called_function() {
    let parsed = parsed_hook(
        "precommit_checks() {\n    run_dupes\n    run_dupes() {\n        cargo dupes check --exclude-tests\n    }\n}\nprecommit_checks\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_nested_helper_shadows_compliant_root_helper() {
    let parsed = parsed_hook(
        "run_dupes() {\n    cargo dupes check --exclude-tests\n}\nprecommit_checks() {\n    run_dupes() {\n        cargo dupes check\n    }\n    run_dupes\n}\nprecommit_checks\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_left_side_of_chained_command_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests && echo ok\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_right_side_of_true_and_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("true && cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_left_side_of_pipeline_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests | tee /tmp/dupes.log\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_semicolon_precedes_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("echo setup; cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_backgrounded_cargo_dupes_runs_with_exclude_tests() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests & wait\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_dead_right_side_of_false_and_is_only_cargo_dupes_occurrence() {
    let parsed = parsed_hook("false && cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dead_right_side_of_true_or_is_only_cargo_dupes_occurrence() {
    let parsed = parsed_hook("true || cargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_exit_zero_prevents_later_and_command() {
    let parsed = parsed_hook("exit 0 && cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_exit_one_prevents_later_or_command() {
    let parsed = parsed_hook("exit 1 || cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_right_side_of_false_or_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("false || cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_if_wrapper_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo dupes check --exclude-tests); then\n    exit 1\nfi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_command_substitution_runs_cargo_dupes_with_exclude_tests() {
    let parsed = parsed_hook("OUTPUT=$(cargo dupes check --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_double_quoted_assignment_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("OUTPUT=\"$(cargo dupes check --exclude-tests)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_double_quoted_argument_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("echo \"$(cargo dupes check --exclude-tests)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_export_assignment_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("export DUPES_OUTPUT=$(cargo dupes check --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_declare_assignment_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("declare DUPES_OUTPUT=\"$(cargo dupes check --exclude-tests)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_dead_command_substitution_is_only_cargo_dupes_occurrence() {
    let parsed = parsed_hook("OUTPUT=$(false && cargo dupes check --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_hook_contains_mixed_compliant_and_noncompliant_cargo_dupes() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests\ncargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_single_quoted_command_substitution_literal_is_only_occurrence() {
    let parsed = parsed_hook("echo '$(cargo dupes check --exclude-tests)'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_escaped_command_substitution_literal_is_only_occurrence() {
    let parsed = parsed_hook("echo \\$(cargo dupes check --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_exclude_tests_only_appears_in_later_command() {
    let parsed = parsed_hook("cargo dupes check && echo --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_exclude_tests_only_appears_in_inline_comment() {
    let parsed = parsed_hook("cargo dupes check # --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_inline_comment_mentions_help_after_real_flag() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests # --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_env_wrapper_runs_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("env -i cargo dupes check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_env_wrapper_bypasses_shadowed_cargo_function() {
    let parsed =
        parsed_hook("cargo() {\n    echo fake\n}\nenv -i cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_shell_wrapper_runs_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("bash -lc 'cargo dupes check'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_command_wrapper_bypasses_shadowed_cargo_function() {
    let parsed =
        parsed_hook("cargo() {\n    echo fake\n}\ncommand cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_command_lookup_only_mentions_cargo_dupes() {
    let parsed = parsed_hook("command -v cargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_subcommand_is_only_help() {
    let parsed = parsed_hook("cargo dupes --help --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_global_help_precedes_dupes_subcommand() {
    let parsed = parsed_hook("cargo --help dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_global_version_precedes_dupes_subcommand() {
    let parsed = parsed_hook("cargo --version dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_binary_is_only_help() {
    let parsed = parsed_hook("cargo-dupes --help --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_help_only_wraps_cargo_dupes_command() {
    let parsed = parsed_hook("env --help cargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_subcommand_only_passes_exclude_tests_after_double_dash() {
    let parsed = parsed_hook("cargo dupes check -- --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_binary_only_passes_exclude_tests_after_double_dash() {
    let parsed = parsed_hook("cargo-dupes check -- --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_attached_exclude_tests_value_is_used() {
    let parsed = parsed_hook("cargo dupes check --exclude-tests=true\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_near_match_cargo_dupes_subcommand_is_used() {
    let parsed = parsed_hook("cargo dupesx check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_called_function_runs_cargo_dupes_without_exact_flag() {
    let parsed = parsed_hook("run_dupes() {\n    cargo dupes check\n}\nrun_dupes\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_function_shadows_real_binary() {
    let parsed = parsed_hook("cargo() {\n    echo fake\n}\ncargo dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_function_shadows_real_binary() {
    let parsed =
        parsed_hook("cargo-dupes() {\n    echo fake\n}\ncargo-dupes check --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_only_exists_inside_uncalled_function() {
    let parsed = parsed_hook("run_dupes() {\n    cargo dupes check --exclude-tests\n}\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}
