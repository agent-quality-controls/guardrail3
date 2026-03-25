use guardrail3_domain_report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;
use super::check;

#[test]
fn warns_when_cargo_deny_only_appears_in_comment() {
    let parsed = parsed_hook("# cargo deny check\n");
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
fn passes_when_cargo_deny_command_exists() {
    let parsed = parsed_hook("cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_deny_hyphen_binary_exists() {
    let parsed = parsed_hook("cargo-deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_deny_check_exists() {
    let parsed = parsed_hook("cargo +nightly deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_cargo_deny_check() {
    let parsed = parsed_hook("env CARGO_TERM_COLOR=always cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_deny_check() {
    let parsed = parsed_hook("RUSTFLAGS='-D warnings' cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_cargo_runs_deny_check() {
    let parsed = parsed_hook("/Users/me/.cargo/bin/cargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_global_flag_precedes_deny_check() {
    let parsed = parsed_hook("cargo --locked deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_cargo_global_flag_deny_check() {
    let parsed = parsed_hook("env -i cargo --locked deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_wrapped_cargo_deny_check_exists() {
    let parsed =
        parsed_hook("if ! (cd \"$RUST_WORKSPACE\" && cargo deny check); then\n    exit 1\nfi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_valid_deny_check_appears_after_invalid_deny_line() {
    let parsed = parsed_hook("cargo deny init\ncargo deny check\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_cargo_deny_is_only_echoed() {
    let parsed = parsed_hook("echo \"cargo deny check\"\n");
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
fn warns_when_cargo_deny_only_appears_in_assignment_text() {
    let parsed = parsed_hook("DENY_LABEL=\"cargo deny check\"\n");
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
fn warns_when_cargo_deny_only_appears_in_export_assignment() {
    let parsed = parsed_hook("export DENY_CMD=\"cargo deny check\"\n");
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
fn warns_when_other_executable_commands_exist_but_not_deny_check() {
    let parsed = parsed_hook("cargo fmt --check\ncargo test --workspace\n");
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
fn warns_when_non_check_cargo_deny_command_is_used() {
    let parsed = parsed_hook("cargo deny init\n");
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
fn warns_when_cargo_deny_check_is_only_help() {
    let parsed = parsed_hook("cargo deny check --help\n");
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
fn warns_when_non_check_hyphen_binary_command_is_used() {
    let parsed = parsed_hook("cargo-deny --help\n");
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
fn warns_when_cargo_deny_binary_check_is_only_help() {
    let parsed = parsed_hook("cargo-deny check --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
