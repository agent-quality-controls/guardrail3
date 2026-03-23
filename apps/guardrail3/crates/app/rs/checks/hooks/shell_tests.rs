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

    assert_eq!(parsed.shebang, Some("#!/usr/bin/env bash"));
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
    assert!(matches!(
        parsed.executable_lines[2].softened_by,
        Some(FailOpenWrapper::Echo(_))
    ));
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
