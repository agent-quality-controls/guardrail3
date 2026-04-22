use hook_shell_parser_runtime_assertions::parser::{
    CommandExpectation, ExpectedFailOpen, FunctionExpectation, ScriptExpectation,
    assert_script_matches,
};

use super::super::parse_script;

#[test]
fn records_shebang_and_executable_lines_only() {
    let parsed = parse_script(
        r#"#!/usr/bin/env bash

# guardrail3 rs validate --staged .
echo "guardrail3 rs validate --staged ."
guardrail3 rs validate --staged .
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            Some("#!/usr/bin/env bash"),
            &[
                CommandExpectation::new("echo", None, None, None, None),
                CommandExpectation::new("guardrail3", None, None, None, None),
            ],
            &[],
        ),
    );
}

#[test]
fn extracts_wrapped_guardrail_command_from_if_block() {
    let parsed = parse_script(
        r#"if ! guardrail3 rs validate --staged .; then
    exit 1
fi
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new(
                    "guardrail3",
                    Some("guardrail3 rs validate --staged ."),
                    None,
                    None,
                    None,
                ),
                CommandExpectation::new("exit", None, None, None, Some(false)),
            ],
            &[],
        ),
    );
}

#[test]
fn extracts_cargo_command_from_cd_and_and_chain() {
    let parsed = parse_script(
        r#"if ! (cd "$RUST_WORKSPACE" && cargo clippy --workspace --all-targets --all-features -- -D warnings); then
    exit 1
fi
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new(
                    "cargo",
                    Some("cargo clippy --workspace --all-targets --all-features -- -D warnings"),
                    None,
                    None,
                    None,
                ),
                CommandExpectation::new("exit", None, None, None, None),
            ],
            &[],
        ),
    );
}

#[test]
fn detects_fail_open_wrappers() {
    let parsed = parse_script(
        r#"guardrail3 rs validate --staged . || true
cargo test --workspace || :
gitleaks protect --staged --no-banner || echo "warning"
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new(
                    "guardrail3",
                    None,
                    None,
                    Some(ExpectedFailOpen::True),
                    None,
                ),
                CommandExpectation::new("cargo", None, None, Some(ExpectedFailOpen::NoOp), None),
                CommandExpectation::new("gitleaks", None, None, Some(ExpectedFailOpen::Echo), None),
            ],
            &[],
        ),
    );
}

#[test]
fn detects_exit_zero_as_executable_bypass() {
    let parsed = parse_script("exit 0\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "exit",
                None,
                None,
                None,
                Some(true),
            )],
            &[],
        ),
    );
}

#[test]
fn detects_dispatcher_syntax_from_source_lines() {
    let parsed = parse_script(
        r#". ".githooks/pre-commit.d/10-rust.sh"
source ".githooks/pre-commit.d/20-ts.sh"
run-parts .githooks/pre-commit.d
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new(".", None, Some(true), None, None),
                CommandExpectation::new("source", None, Some(true), None, None),
                CommandExpectation::new("run-parts", None, Some(true), None, None),
            ],
            &[],
        ),
    );
}

#[test]
fn does_not_mark_echoed_dispatcher_text_as_dispatcher_syntax() {
    let parsed = parse_script("echo run-parts .githooks/pre-commit.d\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "echo",
                None,
                Some(false),
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn ignores_standalone_shell_assignments() {
    let parsed = parse_script(
        r#"RUST_WORKSPACE="${GUARDRAIL3_RUST_WORKSPACE:-.}"
MAX_FILE_SIZE=1048576
guardrail3 rs validate --staged .
"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn extracts_command_substitution_inside_assignment() {
    let parsed = parse_script(
        r#"CARGO_CHANGED=$(echo "$STAGED_FILES" | grep -cE '(Cargo\.toml|Cargo\.lock)$' || true)"#,
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "echo",
                Some("echo \"$STAGED_FILES\" | grep -cE '(Cargo\\.toml|Cargo\\.lock)$'"),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn extracts_command_substitution_inside_export_assignment() {
    let parsed = parse_script("export DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "cargo",
                Some("cargo dupes --exclude-tests"),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn extracts_command_substitution_inside_local_assignment() {
    let parsed = parse_script("local JSCPD_OUTPUT=$(jscpd .)\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "jscpd",
                Some("jscpd ."),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn extracts_quoted_command_substitution_inside_assignment() {
    let parsed = parse_script("OUT=\"$(cargo dupes --exclude-tests)\"\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "cargo",
                Some("cargo dupes --exclude-tests"),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn ignores_single_quoted_command_substitution_literal() {
    let parsed = parse_script("OUT='$(cargo dupes --exclude-tests)'\n");

    assert_script_matches(&parsed, ScriptExpectation::new(None, &[], &[]));
}

#[test]
fn strips_inline_comments_from_executable_commands() {
    let parsed = parse_script("guardrail3 rs validate --staged . # trailing note\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                Some("guardrail3 rs validate --staged ."),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn ignores_heredoc_body_command_text() {
    let parsed = parse_script("cat <<'EOF'\nguardrail3 rs validate --staged .\nEOF\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("cat", None, None, None, None)],
            &[],
        ),
    );
}

#[test]
fn ignores_tab_stripped_heredoc_body_command_text() {
    let parsed = parse_script(
        "cat <<-EOF\n\tg3rs rs validate --staged .\n\tcargo test --workspace\n\tEOF\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("cat", None, None, None, None)],
            &[],
        ),
    );
}

#[test]
fn extracts_command_substitution_inside_declare_assignment() {
    let parsed = parse_script("declare DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "cargo",
                Some("cargo dupes --exclude-tests"),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn extracts_command_substitution_inside_readonly_assignment() {
    let parsed = parse_script("readonly JSCPD_OUTPUT=\"$(jscpd .)\"\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "jscpd",
                Some("jscpd ."),
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn ignores_uncalled_function_body_commands() {
    let parsed = parse_script(
        "guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\necho noop\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("echo", None, None, None, None)],
            &[FunctionExpectation::new("guardrail_validate", None, None)],
        ),
    );
}

#[test]
fn records_called_function_body_for_later_resolution() {
    let parsed = parse_script(
        "guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\nguardrail_validate\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail_validate",
                None,
                None,
                None,
                None,
            )],
            &[FunctionExpectation::new(
                "guardrail_validate",
                Some("guardrail3 rs validate --staged ."),
                Some(&["guardrail3"]),
            )],
        ),
    );
}

#[test]
fn keeps_inline_command_after_single_line_function_definition() {
    let parsed =
        parse_script("guardrail_validate() { guardrail3 rs validate --staged .; }; echo done\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("echo", None, None, None, None)],
            &[FunctionExpectation::new("guardrail_validate", None, None)],
        ),
    );
}

#[test]
fn keeps_inline_command_after_function_definition_when_comment_contains_brace() {
    let parsed = parse_script("finish() { echo ok; }; exit 0 # }\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "exit",
                None,
                None,
                None,
                Some(true),
            )],
            &[FunctionExpectation::new("finish", Some("echo ok"), None)],
        ),
    );
}

#[test]
fn keeps_inline_command_after_function_definition_when_string_contains_brace() {
    let parsed = parse_script("finish() { echo ok; }; exit 0; echo \"}\"\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "exit",
                None,
                None,
                None,
                Some(true),
            )],
            &[FunctionExpectation::new("finish", Some("echo ok"), None)],
        ),
    );
}

#[test]
fn ignores_dead_if_body_commands() {
    let parsed =
        parse_script("if false; then\n    guardrail3 rs validate --staged .\nfi\necho noop\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("echo", None, None, None, None)],
            &[],
        ),
    );
}

#[test]
fn keeps_else_body_after_dead_if_condition() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelse\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn keeps_elif_body_after_dead_if_condition() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelif true; then\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn keeps_all_semicolon_separated_commands_in_single_line_true_if_branch() {
    let parsed = parse_script("if true; then echo ok; g3rs rs validate --staged .; fi\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new("echo", None, None, None, None),
                CommandExpectation::new(
                    "g3rs",
                    Some("g3rs rs validate --staged ."),
                    None,
                    None,
                    None,
                ),
            ],
            &[],
        ),
    );
}

#[test]
fn ignores_dead_elif_body_after_live_if_condition() {
    let parsed = parse_script(
        "if true; then\n    echo ok\nelif true; then\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("echo", None, None, None, None)],
            &[],
        ),
    );
}

#[test]
fn keeps_taken_else_body_from_single_line_dead_if() {
    let parsed =
        parse_script("if false; then echo skip; else guardrail3 rs validate --staged .; fi\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn keeps_taken_elif_body_from_single_line_dead_if() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 rs validate --staged .; fi\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn keeps_taken_elif_body_from_single_line_dead_if_with_else_fallback() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 rs validate --staged .; else echo no; fi\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new(
                "guardrail3",
                None,
                None,
                None,
                None,
            )],
            &[],
        ),
    );
}

#[test]
fn ignores_dead_while_body_commands() {
    let parsed =
        parse_script("while false; do\n    guardrail3 rs validate --staged .\ndone\necho noop\n");

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[CommandExpectation::new("echo", None, None, None, None)],
            &[],
        ),
    );
}

#[test]
fn keeps_commands_after_here_string_loop() {
    let parsed = parse_script(
        "while IFS= read -r file; do\n    [ -n \"$file\" ] || continue\n    if ! (cd \"$file\" && cargo fmt --all -- --check); then\n        exit 1\n    fi\ndone <<< \"$STAGED_FILES\"\necho after\ncargo test --workspace\n",
    );

    assert_script_matches(
        &parsed,
        ScriptExpectation::new(
            None,
            &[
                CommandExpectation::new("[", None, None, None, None),
                CommandExpectation::new(
                    "cargo",
                    Some("cargo fmt --all -- --check"),
                    None,
                    None,
                    None,
                ),
                CommandExpectation::new("exit", None, None, None, None),
                CommandExpectation::new(
                    "done",
                    Some("done <<< \"$STAGED_FILES\""),
                    None,
                    None,
                    None,
                ),
                CommandExpectation::new("echo", None, None, None, None),
                CommandExpectation::new("cargo", Some("cargo test --workspace"), None, None, None),
            ],
            &[],
        ),
    );
}
