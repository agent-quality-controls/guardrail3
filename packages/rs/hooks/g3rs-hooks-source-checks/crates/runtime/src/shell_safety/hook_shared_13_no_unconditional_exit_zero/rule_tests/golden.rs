use g3rs_hooks_source_checks_assertions::shell_safety::hook_shared_13_no_unconditional_exit_zero::rule as assertions;

use super::run_case;

#[test]
fn warns_when_exit_zero_is_executable() {
    let results = run_case("exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            message_contains: Some("force the hook to succeed"),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_exit_zero_only_appears_in_comment() {
    let results = run_case("# exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_exit_zero_is_only_in_no_staged_files_branch() {
    let results = run_case(
        "if [ -z \"$STAGED_FILES\" ]; then\n    echo \"No staged files.\"\n    exit 0\nfi\ncargo test --workspace\n",
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_prefixed_loop_opener_keeps_exit_zero_scoped() {
    let results = run_case("time while false; do\n    exit 0\ndone\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_called_function_contains_exit_zero() {
    let results = run_case("finish() {\n    exit 0\n}\nfinish\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_multi_hop_called_function_contains_exit_zero() {
    let results =
        run_case("finish_leaf() {\n    exit 0\n}\nfinish_mid() {\n    finish_leaf\n}\nfinish_mid\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_chained_helper_uses_later_redefinition_with_exit_zero() {
    let results = run_case(
        "finish_leaf() {\n    echo ok\n}\nfinish_leaf() {\n    exit 0\n}\nfinish_mid() {\n    finish_leaf\n}\nfinish_mid\n",
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(5),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_function_definition_has_exit_zero_tail() {
    let results = run_case("finish() { exit 0; }; exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_loop_terminator_has_exit_zero_tail() {
    let results = run_case("while true; do\n    :\ndone; exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(3),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_if_terminator_has_and_tail_exit_zero() {
    let results = run_case("if true; then\n    :\nfi && exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(3),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_same_line_if_body_contains_exit_zero() {
    let results = run_case("if true; then exit 0; fi\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_same_line_case_body_contains_exit_zero() {
    let results = run_case("case \"$x\" in a) exit 0 ;; esac\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_tab_delimited_same_line_if_body_contains_exit_zero() {
    let results = run_case("if\ttrue; then exit 0; fi\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_tab_delimited_same_line_case_body_contains_exit_zero() {
    let results = run_case("case\t\"$x\" in a) exit 0 ;; esac\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn passes_when_tab_delimited_case_opener_contains_exit_zero() {
    let results = run_case("case\t\"$x\" in\n    a) exit 0 ;;\nesac\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("no unconditional `exit 0` bypass in `.githooks/pre-commit`"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_loop_terminator_has_and_tail_exit_zero() {
    let results = run_case("while true; do\n    :\ndone && exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(3),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_case_scope_has_exit_zero_tail() {
    let results = run_case("case \"$x\" in a) : ;; esac && exit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_later_function_redefinition_contains_exit_zero() {
    let results = run_case("finish() {\n    echo ok\n}\nfinish() {\n    exit 0\n}\nfinish\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(5),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_shell_wrapper_executes_exit_zero() {
    let sh_results = run_case("sh -c 'exit 0'\n");
    assertions::assert_rule_results(
        &sh_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );

    let bash_results = run_case("bash -c 'exit 0'\n");
    assertions::assert_rule_results(
        &bash_results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_loop_close_has_trailing_redirection_before_called_function() {
    let results = run_case(
        "while IFS= read -r file; do\n    echo \"$file\"\ndone <<< \"$STAGED_FILES\"\nfinish() {\n    exit 0\n}\nfinish\n",
    );
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(5),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_if_opener_closer_suffix_leaks_later_exit_zero() {
    let results = run_case("if true; then :; fi && echo after\nexit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_same_line_loop_opener_closer_suffix_leaks_later_exit_zero() {
    let results = run_case("while true; do :; done && echo after\nexit 0\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(2),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn warns_when_function_definition_tail_has_exit_zero_after_late_brace() {
    let results = run_case("finish() { echo ok; }; exit 0 # }\n");
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::G3Severity::Warn),
            title: Some("remove unconditional `exit 0` from `.githooks/pre-commit`"),
            line: Some(1),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
