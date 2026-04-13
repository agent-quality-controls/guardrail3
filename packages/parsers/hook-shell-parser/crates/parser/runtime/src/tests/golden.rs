use crate::command_query::{ResolvedCommand, any_resolved_command};

use super::{FailOpenWrapper, parse_script};

#[test]
fn records_shebang_and_executable_lines_only() {
    let parsed = parse_script(
        r#"#!/usr/bin/env bash

# guardrail3 rs validate --staged .
echo "guardrail3 rs validate --staged ."
guardrail3 rs validate --staged .
"#,
    );

    assert_eq!(parsed.shebang(), Some("#!/usr/bin/env bash"));
    assert_eq!(parsed.executable_lines.len(), 2);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
    assert_eq!(parsed.executable_lines[1].command_name, "guardrail3");
}

#[test]
fn extracts_wrapped_guardrail_command_from_if_block() {
    let parsed = parse_script(
        r#"if ! guardrail3 rs validate --staged .; then
    exit 1
fi
"#,
    );

    assert_eq!(parsed.executable_lines.len(), 2);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "guardrail3 rs validate --staged ."
    );
    assert!(!parsed.executable_lines[1].is_exit_zero);
}

#[test]
fn extracts_cargo_command_from_cd_and_and_chain() {
    let parsed = parse_script(
        r#"if ! (cd "$RUST_WORKSPACE" && cargo clippy --workspace --all-targets --all-features -- -D warnings); then
    exit 1
fi
"#,
    );

    assert_eq!(parsed.executable_lines[0].command_name, "cargo");
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "cargo clippy --workspace --all-targets --all-features -- -D warnings"
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

    assert_eq!(
        parsed.executable_lines[0].softened_by,
        Some(FailOpenWrapper::True)
    );
    assert_eq!(
        parsed.executable_lines[1].softened_by,
        Some(FailOpenWrapper::NoOp)
    );
    assert!(
        matches!(
            parsed.executable_lines[2].softened_by,
            Some(FailOpenWrapper::Echo(_))
        ),
        "expected echo fail-open wrapper on the third executable line"
    );
}

#[test]
fn detects_exit_zero_as_executable_bypass() {
    let parsed = parse_script("exit 0\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "exit");
    assert!(parsed.executable_lines[0].is_exit_zero);
}

#[test]
fn detects_dispatcher_syntax_from_source_lines() {
    let parsed = parse_script(
        r#". ".githooks/pre-commit.d/10-rust.sh"
source ".githooks/pre-commit.d/20-ts.sh"
run-parts .githooks/pre-commit.d
"#,
    );

    assert_eq!(parsed.executable_lines.len(), 3);
    assert!(
        parsed
            .executable_lines
            .iter()
            .all(|line| line.is_dispatcher_syntax)
    );
}

#[test]
fn does_not_mark_echoed_dispatcher_text_as_dispatcher_syntax() {
    let parsed = parse_script("echo run-parts .githooks/pre-commit.d\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
    assert!(!parsed.executable_lines[0].is_dispatcher_syntax);
}

#[test]
fn ignores_standalone_shell_assignments() {
    let parsed = parse_script(
        r#"RUST_WORKSPACE="${GUARDRAIL3_RUST_WORKSPACE:-.}"
MAX_FILE_SIZE=1048576
guardrail3 rs validate --staged .
"#,
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn extracts_command_substitution_inside_assignment() {
    let parsed = parse_script(
        r#"CARGO_CHANGED=$(echo "$STAGED_FILES" | grep -cE '(Cargo\.toml|Cargo\.lock)$' || true)"#,
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
    assert!(parsed.executable_lines[0].raw.contains("Cargo\\.toml"));
}

#[test]
fn extracts_command_substitution_inside_export_assignment() {
    let parsed = parse_script("export DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "cargo");
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "cargo dupes --exclude-tests"
    );
}

#[test]
fn extracts_command_substitution_inside_local_assignment() {
    let parsed = parse_script("local JSCPD_OUTPUT=$(jscpd .)\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "jscpd");
    assert_eq!(parsed.executable_lines[0].command_text, "jscpd .");
}

#[test]
fn extracts_quoted_command_substitution_inside_assignment() {
    let parsed = parse_script("OUT=\"$(cargo dupes --exclude-tests)\"\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "cargo");
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "cargo dupes --exclude-tests"
    );
}

#[test]
fn ignores_single_quoted_command_substitution_literal() {
    let parsed = parse_script("OUT='$(cargo dupes --exclude-tests)'\n");

    assert!(parsed.executable_lines.is_empty());
}

#[test]
fn strips_inline_comments_from_executable_commands() {
    let parsed = parse_script("guardrail3 rs validate --staged . # trailing note\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "guardrail3 rs validate --staged ."
    );
}

#[test]
fn ignores_heredoc_body_command_text() {
    let parsed = parse_script("cat <<'EOF'\nguardrail3 rs validate --staged .\nEOF\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "cat");
}

#[test]
fn ignores_tab_stripped_heredoc_body_command_text() {
    let parsed = parse_script(
        "cat <<-EOF\n\tg3rs rs validate --staged .\n\tcargo test --workspace\n\tEOF\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "cat");
}

#[test]
fn extracts_command_substitution_inside_declare_assignment() {
    let parsed = parse_script("declare DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "cargo");
    assert_eq!(
        parsed.executable_lines[0].command_text,
        "cargo dupes --exclude-tests"
    );
}

#[test]
fn extracts_command_substitution_inside_readonly_assignment() {
    let parsed = parse_script("readonly JSCPD_OUTPUT=\"$(jscpd .)\"\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "jscpd");
    assert_eq!(parsed.executable_lines[0].command_text, "jscpd .");
}

#[test]
fn ignores_uncalled_function_body_commands() {
    let parsed = parse_script(
        "guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\necho noop\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
    assert_eq!(parsed.functions.len(), 1);
    assert_eq!(parsed.functions[0].name, "guardrail_validate");
}

#[test]
fn records_called_function_body_for_later_resolution() {
    let parsed = parse_script(
        "guardrail_validate() {\n    guardrail3 rs validate --staged .\n}\nguardrail_validate\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(
        parsed.executable_lines[0].command_name,
        "guardrail_validate"
    );
    assert_eq!(parsed.functions.len(), 1);
    assert!(
        parsed.functions[0]
            .body
            .contains("guardrail3 rs validate --staged .")
    );
}

#[test]
fn keeps_inline_command_after_single_line_function_definition() {
    let parsed =
        parse_script("guardrail_validate() { guardrail3 rs validate --staged .; }; echo done\n");

    assert_eq!(parsed.functions.len(), 1);
    assert_eq!(parsed.functions[0].name, "guardrail_validate");
    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
}

fn is_rg_conflict_scan(command: &ResolvedCommand) -> bool {
    command.command_name() == "rg" && command.command_text().contains("<<<<<<<")
}

#[test]
fn resolves_called_function_with_path_qualified_command() {
    let parsed = parse_script("check_conflicts() {\n    /usr/bin/rg '<<<<<<<' .\n}\ncheck_conflicts\n");

    assert_eq!(parsed.functions.len(), 1);
    let function_body = parse_script(parsed.functions[0].body());
    assert_eq!(function_body.executable_lines.len(), 1);
    assert_eq!(function_body.executable_lines[0].command_name, "/usr/bin/rg");
    assert!(any_resolved_command(&parsed, is_rg_conflict_scan));
}

#[test]
fn ignores_dead_if_body_commands() {
    let parsed =
        parse_script("if false; then\n    guardrail3 rs validate --staged .\nfi\necho noop\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
}

#[test]
fn keeps_else_body_after_dead_if_condition() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelse\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn keeps_elif_body_after_dead_if_condition() {
    let parsed = parse_script(
        "if false; then\n    echo skip\nelif true; then\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn keeps_all_semicolon_separated_commands_in_single_line_true_if_branch() {
    let parsed = parse_script(
        "if true; then echo ok; g3rs rs validate --staged .; fi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 2);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
    assert_eq!(parsed.executable_lines[1].command_name, "g3rs");
    assert_eq!(
        parsed.executable_lines[1].command_text,
        "g3rs rs validate --staged ."
    );
}

#[test]
fn ignores_dead_elif_body_after_live_if_condition() {
    let parsed = parse_script(
        "if true; then\n    echo ok\nelif true; then\n    guardrail3 rs validate --staged .\nfi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
}

#[test]
fn keeps_taken_else_body_from_single_line_dead_if() {
    let parsed =
        parse_script("if false; then echo skip; else guardrail3 rs validate --staged .; fi\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn keeps_taken_elif_body_from_single_line_dead_if() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 rs validate --staged .; fi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn keeps_taken_elif_body_from_single_line_dead_if_with_else_fallback() {
    let parsed = parse_script(
        "if false; then echo skip; elif true; then guardrail3 rs validate --staged .; else echo no; fi\n",
    );

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "guardrail3");
}

#[test]
fn ignores_dead_while_body_commands() {
    let parsed =
        parse_script("while false; do\n    guardrail3 rs validate --staged .\ndone\necho noop\n");

    assert_eq!(parsed.executable_lines.len(), 1);
    assert_eq!(parsed.executable_lines[0].command_name, "echo");
}
