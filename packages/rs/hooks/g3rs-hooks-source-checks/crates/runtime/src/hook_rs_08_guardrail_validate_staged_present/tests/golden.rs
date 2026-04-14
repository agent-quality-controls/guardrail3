use g3rs_hooks_source_checks_assertions::hook_rs_08_guardrail_validate_staged_present as assertions;

use crate::hook_rs_08_guardrail_validate_staged_present::run_case;

#[test]
fn warns_when_only_comment_mentions_guardrail_validation() {
    let results = run_case("# g3rs rs validate --staged .\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            inventory: Some(false),
            title: Some("Rust guardrail validate step missing"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_executable_rs_guardrail_validation_exists() {
    let results = run_case("g3rs rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_top_level_guardrail_validation_exists() {
    let results = run_case("g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_wrapped_guardrail_validation_exists() {
    let results = run_case(
        "if ! (cd \"$RUST_WORKSPACE\" && g3rs rs validate --staged .); then\n    exit 1\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_one_line_if_then_runs_guardrail_validation() {
    let results =
        run_case("if test -n \"$RUST_CHANGED\"; then g3rs validate --staged .; fi\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_backslash_continued_guardrail_validation_exists() {
    let results = run_case("g3rs rs validate \\\n  --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_guardrail_validation_is_backgrounded() {
    let results = run_case("g3rs validate --staged . &\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_is_piped() {
    let results = run_case("g3rs validate --staged . | cat\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_guardrail_validation_precedes_same_line_and_chain() {
    let results = run_case("g3rs rs validate --staged . && echo ok\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wraps_shell_wrapper_guardrail_validation() {
    let results = run_case("exec bash -lc 'g3rs validate --staged .'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wraps_env_guardrail_validation() {
    let results = run_case("exec env g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_flag_with_value_wraps_guardrail_validation() {
    let results = run_case("env --chdir /repo g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_attached_flag_value_wraps_guardrail_validation() {
    let results = run_case("env --chdir=/repo g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_split_string_wraps_guardrail_validation() {
    let results = run_case("env -S 'g3rs validate --staged .'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_double_dash_wraps_guardrail_validation() {
    let results = run_case("env -- g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_guardrail_validation() {
    let results = run_case("env RUST_LOG=info g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_env_wraps_shell_wrapper_guardrail_validation() {
    let results = run_case("env RUST_LOG=info bash -lc 'g3rs validate --staged .'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_command_wrapper_runs_guardrail_validation() {
    let results = run_case("command -- g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_exec_wrapper_runs_guardrail_validation() {
    let results = run_case("exec g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_path_qualified_guardrail_validation_exists() {
    let results = run_case("/usr/local/bin/g3rs rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_shell_wrapper_runs_guardrail_after_setup() {
    let results = run_case("bash -lc 'cd /repo && g3rs validate --staged .'\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_runs_inside_command_substitution() {
    let results = run_case("STATUS=$(g3rs rs validate --staged .)\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_runs_inside_exported_command_substitution() {
    let results = run_case("export STATUS=$(g3rs validate --staged .)\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_dead_global_flag_precedes_validate() {
    let results = run_case("g3rs --config guardrail3-rs.toml validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dead_attached_global_flag_precedes_validate() {
    let results = run_case("g3rs --config=guardrail3-rs.toml validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_env_wraps_path_qualified_guardrail_validation() {
    let results = run_case("env -i /usr/local/bin/g3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_validate_and_staged_are_split_across_commands() {
    let results = run_case("g3rs rs validate .\ng3rs fmt --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validate_is_only_echoed() {
    let results = run_case("echo \"g3rs rs validate --staged .\"\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_inline_comment_only_mentions_staged_flag() {
    let results = run_case("g3rs rs validate . # --staged\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_inline_comment_mentions_help_after_real_command() {
    let results = run_case("g3rs rs validate --staged . # --help\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_other_executable_commands_exist_but_not_guardrail_validation() {
    let results = run_case("cargo fmt --check\ncargo test --workspace\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_only_exists_in_heredoc_body() {
    let results = run_case("cat <<'EOF'\ng3rs rs validate --staged .\nEOF\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_wrong_guardrail_binary_name_is_used() {
    let results = run_case("g3rsx rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_dead_right_hand_and_branch() {
    let results = run_case("false && g3rs rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_dead_right_hand_or_branch() {
    let results = run_case("true || g3rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_transitive_dead_and_chain() {
    let results = run_case("false && echo no && g3rs rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_transitive_dead_or_chain() {
    let results = run_case("true || echo no || g3rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_dead_inside_command_substitution_chain() {
    let results = run_case("STATUS=$(false && g3rs rs validate --staged .)\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_dead_outer_branch_only_contains_guardrail_substitution() {
    let results = run_case("true || echo $(g3rs validate --staged .)\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_dead_negated_and_branch() {
    let results = run_case("! true && g3rs rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_validate_is_on_dead_negated_or_branch() {
    let results = run_case("! false || g3rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validate_near_match_is_used() {
    let results = run_case("g3rs rs validatex --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validate_is_only_help() {
    let results = run_case("g3rs rs validate --help --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_global_help_precedes_validate() {
    let results = run_case("g3rs --help validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_global_version_precedes_validate() {
    let results = run_case("g3rs --version validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_env_help_wraps_guardrail_validation() {
    let results = run_case("env --help g3rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_top_level_guardrail_validate_is_only_help() {
    let results = run_case("g3rs validate --help --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_valid_guardrail_validation_appears_after_invalid_line() {
    let results =
        run_case("g3rs rs validatex --staged .\ng3rs rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_valid_guardrail_validation_appears_after_unrelated_lines() {
    let results = run_case("cargo fmt --check\necho 'validate'\ng3rs validate --staged .\n");
    assertions::assert_present(&results);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_assignment_text() {
    let results = run_case("CMD='g3rs rs validate --staged .'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_export_assignment() {
    let results = run_case("export CMD='g3rs rs validate --staged .'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_single_quoted_substitution_literal() {
    let results = run_case("OUT='$(g3rs rs validate --staged .)'\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_command_lookup_only_mentions_guardrail_validation() {
    let results = run_case("command -v g3rs validate --staged .\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_uncalled_function() {
    let results = run_case("guardrail_validate() {\n    g3rs rs validate --staged .\n}\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_called_function_runs_guardrail_validation() {
    let results = run_case(
        "guardrail_validate() {\n    g3rs rs validate --staged .\n}\nguardrail_validate\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_called_helper_chain_runs_guardrail_validation() {
    let results = run_case(
        "run_guardrail() {\n    g3rs rs validate --staged .\n}\nprecommit_checks() {\n    run_guardrail\n}\nprecommit_checks\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_if_body() {
    let results = run_case("if false; then\n    g3rs rs validate --staged .\nfi\n");
    assertions::assert_missing(&results);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_else_body() {
    let results = run_case(
        "if false; then\n    echo skip\nelse\n    g3rs rs validate --staged .\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_elif_body() {
    let results = run_case(
        "if false; then\n    echo skip\nelif true; then\n    g3rs rs validate --staged .\nfi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_else_body() {
    let results = run_case("if false; then echo skip; else g3rs validate --staged .; fi\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_elif_body() {
    let results =
        run_case("if false; then echo skip; elif true; then g3rs validate --staged .; fi\n");
    assertions::assert_present(&results);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_elif_body_with_else_fallback() {
    let results = run_case(
        "if false; then echo skip; elif true; then g3rs validate --staged .; else echo no; fi\n",
    );
    assertions::assert_present(&results);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_else_body() {
    let results =
        run_case("if true; then\n    echo ok\nelse\n    g3rs validate --staged .\nfi\n");
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_elif_body() {
    let results = run_case(
        "if true; then\n    echo ok\nelif true; then\n    g3rs validate --staged .\nfi\n",
    );
    assertions::assert_missing(&results);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_while_body() {
    let results = run_case("while false; do\n    g3rs rs validate --staged .\ndone\n");
    assertions::assert_missing(&results);
}
