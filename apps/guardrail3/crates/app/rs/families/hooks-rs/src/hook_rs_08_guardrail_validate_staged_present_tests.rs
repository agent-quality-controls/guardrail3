use guardrail3_app_rs_family_hooks_shared::hook_shell::parse_script;
use guardrail3_domain_report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::check;

#[test]
fn warns_when_only_comment_mentions_guardrail_validation() {
    let parsed = parse_script("# guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_executable_rs_guardrail_validation_exists() {
    let parsed = parse_script("guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_top_level_guardrail_validation_exists() {
    let parsed = parse_script("guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_wrapped_guardrail_validation_exists() {
    let parsed = parse_script(
        "if ! (cd \"$RUST_WORKSPACE\" && guardrail3 rs validate --staged .); then\n    exit 1\nfi\n",
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
fn passes_when_one_line_if_then_runs_guardrail_validation() {
    let parsed =
        parse_script("if test -n \"$RUST_CHANGED\"; then guardrail3 validate --staged .; fi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_backslash_continued_guardrail_validation_exists() {
    let parsed = parse_script("guardrail3 rs validate \\\n  --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_is_backgrounded() {
    let parsed = parse_script("guardrail3 validate --staged . &\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_is_piped() {
    let parsed = parse_script("guardrail3 validate --staged . | cat\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_precedes_same_line_and_chain() {
    let parsed = parse_script("guardrail3 rs validate --staged . && echo ok\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exec_wraps_shell_wrapper_guardrail_validation() {
    let parsed = parse_script("exec bash -lc 'guardrail3 validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exec_wraps_env_guardrail_validation() {
    let parsed = parse_script("exec env guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_flag_with_value_wraps_guardrail_validation() {
    let parsed = parse_script("env --chdir /repo guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_attached_flag_value_wraps_guardrail_validation() {
    let parsed = parse_script("env --chdir=/repo guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_split_string_wraps_guardrail_validation() {
    let parsed = parse_script("env -S 'guardrail3 validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_double_dash_wraps_guardrail_validation() {
    let parsed = parse_script("env -- guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_guardrail_validation() {
    let parsed = parse_script("env RUST_LOG=info guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_shell_wrapper_guardrail_validation() {
    let parsed = parse_script("env RUST_LOG=info bash -lc 'guardrail3 validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_command_wrapper_runs_guardrail_validation() {
    let parsed = parse_script("command -- guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exec_wrapper_runs_guardrail_validation() {
    let parsed = parse_script("exec guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_guardrail_validation_exists() {
    let parsed = parse_script("/usr/local/bin/guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_wrapper_runs_guardrail_after_setup() {
    let parsed = parse_script("bash -lc 'cd /repo && guardrail3 validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_runs_inside_command_substitution() {
    let parsed = parse_script("STATUS=$(guardrail3 rs validate --staged .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_runs_inside_exported_command_substitution() {
    let parsed = parse_script("export STATUS=$(guardrail3 validate --staged .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_global_flag_precedes_validate() {
    let parsed = parse_script("guardrail3 --config guardrail3.toml validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_attached_guardrail_global_flag_precedes_validate() {
    let parsed = parse_script("guardrail3 --config=guardrail3.toml validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_path_qualified_guardrail_validation() {
    let parsed = parse_script("env -i /usr/local/bin/guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_validate_and_staged_are_split_across_commands() {
    let parsed = parse_script("guardrail3 rs validate .\nguardrail3 fmt --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validate_is_only_echoed() {
    let parsed = parse_script("echo \"guardrail3 rs validate --staged .\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_inline_comment_only_mentions_staged_flag() {
    let parsed = parse_script("guardrail3 rs validate . # --staged\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_inline_comment_mentions_help_after_real_command() {
    let parsed = parse_script("guardrail3 rs validate --staged . # --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_other_executable_commands_exist_but_not_guardrail_validation() {
    let parsed = parse_script("cargo fmt --check\ncargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_only_exists_in_heredoc_body() {
    let parsed = parse_script("cat <<'EOF'\nguardrail3 rs validate --staged .\nEOF\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_wrong_guardrail_binary_name_is_used() {
    let parsed = parse_script("guardrail3x rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_dead_right_hand_and_branch() {
    let parsed = parse_script("false && guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_dead_right_hand_or_branch() {
    let parsed = parse_script("true || guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_transitive_dead_and_chain() {
    let parsed = parse_script("false && echo no && guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_transitive_dead_or_chain() {
    let parsed = parse_script("true || echo no || guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_dead_inside_command_substitution_chain() {
    let parsed = parse_script("STATUS=$(false && guardrail3 rs validate --staged .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dead_outer_branch_only_contains_guardrail_substitution() {
    let parsed = parse_script("true || echo $(guardrail3 validate --staged .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_dead_negated_and_branch() {
    let parsed = parse_script("! true && guardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_validate_is_on_dead_negated_or_branch() {
    let parsed = parse_script("! false || guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validate_near_match_is_used() {
    let parsed = parse_script("guardrail3 rs validatex --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validate_is_only_help() {
    let parsed = parse_script("guardrail3 rs validate --help --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_global_help_precedes_validate() {
    let parsed = parse_script("guardrail3 --help validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_global_version_precedes_validate() {
    let parsed = parse_script("guardrail3 --version validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_help_wraps_guardrail_validation() {
    let parsed = parse_script("env --help guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_top_level_guardrail_validate_is_only_help() {
    let parsed = parse_script("guardrail3 validate --help --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_valid_guardrail_validation_appears_after_invalid_line() {
    let parsed =
        parse_script("guardrail3 rs validatex --staged .\nguardrail3 rs validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_valid_guardrail_validation_appears_after_unrelated_lines() {
    let parsed =
        parse_script("cargo fmt --check\necho 'validate'\nguardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_assignment_text() {
    let parsed = parse_script("CMD='guardrail3 rs validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_export_assignment() {
    let parsed = parse_script("export CMD='guardrail3 rs validate --staged .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_appears_in_single_quoted_substitution_literal() {
    let parsed = parse_script("OUT='$(guardrail3 rs validate --staged .)'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_command_lookup_only_mentions_guardrail_validation() {
    let parsed = parse_script("command -v guardrail3 validate --staged .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_uncalled_function() {
    let parsed = parse_script("guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_called_function_runs_guardrail_validation() {
    let parsed = parse_script(
        "guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\nguardrail_validate\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_called_helper_chain_runs_guardrail_validation() {
    let parsed = parse_script(
        "run_guardrail() {\n    guardrail3 rs validate --staged .\n}\nprecommit_checks() {\n    run_guardrail\n}\nprecommit_checks\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_if_body() {
    let parsed = parse_script("if false; then\n    guardrail3 rs validate --staged .\nfi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_else_body() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelse\n    guardrail3 rs validate --staged .\nfi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_elif_body() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelif true; then\n    guardrail3 rs validate --staged .\nfi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_else_body() {
    let parsed =
        parse_script("if false; then echo skip; else guardrail3 validate --staged .; fi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_elif_body() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 validate --staged .; fi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_guardrail_validation_exists_inside_taken_single_line_elif_body_with_else_fallback() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 validate --staged .; else echo no; fi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_else_body() {
    let parsed =
        parse_script("if true; then\n    echo ok\nelse\n    guardrail3 validate --staged .\nfi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_elif_body() {
    let parsed = parse_script(
        "if true; then\n    echo ok\nelif true; then\n    guardrail3 validate --staged .\nfi\n",
    );
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_guardrail_validation_only_exists_inside_dead_while_body() {
    let parsed = parse_script("while false; do\n    guardrail3 rs validate --staged .\ndone\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
