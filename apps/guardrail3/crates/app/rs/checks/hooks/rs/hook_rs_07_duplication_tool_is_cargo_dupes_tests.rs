use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;
use super::check;

#[test]
fn warns_when_only_jscpd_exists() {
    let parsed = parsed_hook("jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_exists() {
    let parsed = parsed_hook("cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_binary_exists() {
    let parsed = parsed_hook("cargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_wrapped_cargo_dupes_exists() {
    let parsed = parsed_hook(
        "if ! (cd \"$RUST_WORKSPACE\" && cargo dupes --exclude-tests); then\n    exit 1\nfi\n",
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
fn passes_when_dupes_is_followed_by_same_line_command_chain() {
    let parsed = parsed_hook("cargo dupes --exclude-tests && echo ok\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_dupes_precedes_same_line_jscpd_chain() {
    let parsed = parsed_hook("cargo dupes --exclude-tests && jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_dupes_follows_semicolon_chain() {
    let parsed = parsed_hook("cd \"$RUST_WORKSPACE\"; cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exported_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("export DUPES_OUTPUT=$(cargo dupes --exclude-tests)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_quoted_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_declare_command_substitution_runs_cargo_dupes() {
    let parsed = parsed_hook("declare DUPES_OUTPUT=\"$(cargo dupes --exclude-tests)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_toolchain_prefixed_cargo_dupes_exists() {
    let parsed = parsed_hook("cargo +nightly dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_cargo_dupes() {
    let parsed = parsed_hook("env CARGO_TERM_COLOR=always cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_flag_with_value_wraps_cargo_dupes() {
    let parsed = parsed_hook("env -u NODE_OPTIONS cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_chdir_wraps_cargo_dupes() {
    let parsed = parsed_hook("env --chdir /repo cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_split_string_env_runs_cargo_dupes() {
    let parsed = parsed_hook("env -S 'cargo dupes --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_long_form_split_string_env_runs_cargo_dupes() {
    let parsed = parsed_hook("env --split-string 'cargo dupes --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_long_form_split_string_assignments_precede_cargo_dupes() {
    let parsed =
        parsed_hook("env --split-string 'RUSTFLAGS=-D warnings' cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_quoted_env_assignment_wraps_cargo_dupes() {
    let parsed = parsed_hook("RUSTFLAGS='-D warnings' cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_cargo_runs_dupes() {
    let parsed = parsed_hook("/usr/bin/cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_dupes_binary_runs() {
    let parsed = parsed_hook("/opt/homebrew/bin/cargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_path_qualified_dupes_binary() {
    let parsed = parsed_hook("env -i /opt/homebrew/bin/cargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_toolchain_prefixed_cargo_runs_dupes() {
    let parsed = parsed_hook("/Users/me/.cargo/bin/cargo +nightly dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_assignment_and_toolchain_wrap_cargo_dupes() {
    let parsed = parsed_hook("RUSTFLAGS='-D warnings' cargo +nightly dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_global_flag_precedes_dupes() {
    let parsed = parsed_hook("cargo --locked dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_global_flag_with_value_precedes_dupes() {
    let parsed = parsed_hook("cargo --manifest-path Cargo.toml dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_short_cargo_global_flag_with_value_precedes_dupes() {
    let parsed = parsed_hook("cargo -Z unstable-options dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_dupes_wins_over_jscpd_if_both_exist() {
    let parsed = parsed_hook("jscpd .\ncargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_dupes_binary_wins_over_jscpd_if_both_exist() {
    let parsed = parsed_hook("jscpd .\ncargo-dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_env_wrapped_dupes_wins_over_jscpd_if_both_exist() {
    let parsed = parsed_hook("jscpd .\nenv CARGO_TERM_COLOR=always cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_jscpd_precedes_same_line_dupes_chain() {
    let parsed = parsed_hook("jscpd . && cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_brace_group_runs_cargo_dupes() {
    let parsed = parsed_hook("{ cargo dupes --exclude-tests; }\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_dupes_runs_on_right_hand_or_chain() {
    let parsed = parsed_hook("cargo fmt --check || cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_dupes_wins_on_right_hand_or_chain_after_jscpd() {
    let parsed = parsed_hook("jscpd . || cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "cargo-dupes selected for Rust duplication checks"
    );
}

#[test]
fn passes_when_valid_dupes_appears_after_unrelated_commands() {
    let parsed = parsed_hook("cargo fmt --check\necho 'dupes'\ncargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_no_duplication_tool_is_present() {
    let parsed = parsed_hook("cargo fmt --check\ncargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_is_only_echoed() {
    let parsed = parsed_hook("echo \"cargo dupes --exclude-tests\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_dupes_is_only_commented() {
    let parsed = parsed_hook("# cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_near_match_dupes_command_is_used() {
    let parsed = parsed_hook("cargo dupesx --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_other_cargo_subcommand_exists_but_not_dupes() {
    let parsed = parsed_hook("cargo nextest run\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dupes_command_is_only_help() {
    let parsed = parsed_hook("cargo dupes --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_global_help_precedes_dupes() {
    let parsed = parsed_hook("cargo --help dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_cargo_global_version_precedes_dupes() {
    let parsed = parsed_hook("cargo --version dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_help_wraps_dupes() {
    let parsed = parsed_hook("env --help cargo dupes --exclude-tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_split_string_env_wraps_dupes_help() {
    let parsed = parsed_hook("env -S 'cargo dupes --help'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dupes_binary_is_only_help() {
    let parsed = parsed_hook("cargo-dupes --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_long_form_split_string_wraps_dupes_version() {
    let parsed = parsed_hook("env --split-string 'cargo-dupes --version'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dupes_only_appears_in_assignment_text() {
    let parsed = parsed_hook("DUPES_CMD='cargo dupes --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_dupes_only_appears_in_export_assignment() {
    let parsed = parsed_hook("export DUPES_CMD='cargo-dupes --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_only_echoed_dupes_exists_but_real_jscpd_runs() {
    let parsed = parsed_hook("echo \"cargo dupes --exclude-tests\"\njscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_path_qualified_jscpd_exists() {
    let parsed = parsed_hook("/usr/bin/jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_wraps_jscpd() {
    let parsed = parsed_hook("env NODE_OPTIONS=--no-deprecation jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_jscpd_is_only_echoed() {
    let parsed = parsed_hook("echo \"jscpd .\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_quoted_separator_precedes_dupes_text() {
    let parsed = parsed_hook("echo \"note; cargo dupes --exclude-tests\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_jscpd_is_only_commented() {
    let parsed = parsed_hook("# jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_quoted_separator_precedes_jscpd_text() {
    let parsed = parsed_hook("echo \"note; jscpd .\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_jscpd_only_appears_in_assignment_text() {
    let parsed = parsed_hook("JSCPD_CMD='jscpd .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_near_match_jscpd_command_is_used() {
    let parsed = parsed_hook("jscpdx .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_flag_with_value_wraps_jscpd() {
    let parsed = parsed_hook("env -u NODE_OPTIONS jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_chdir_wraps_jscpd() {
    let parsed = parsed_hook("env --chdir /repo jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_split_string_env_wraps_jscpd() {
    let parsed = parsed_hook("env --split-string 'NODE_OPTIONS=--trace-warnings' jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_split_string_env_runs_jscpd() {
    let parsed = parsed_hook("env --split-string 'jscpd .'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_split_string_env_wraps_jscpd_help() {
    let parsed = parsed_hook("env -S 'jscpd --help'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_wrapped_jscpd_exists() {
    let parsed = parsed_hook("if ! jscpd .; then\n    exit 1\nfi\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_command_substitution_runs_jscpd() {
    let parsed = parsed_hook("JSCPD_OUTPUT=$(jscpd .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_local_command_substitution_runs_jscpd() {
    let parsed = parsed_hook("local JSCPD_OUTPUT=$(jscpd .)\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_brace_group_runs_jscpd() {
    let parsed = parsed_hook("{ jscpd .; }\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_quoted_command_substitution_runs_jscpd() {
    let parsed = parsed_hook("JSCPD_OUTPUT=\"$(jscpd .)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_readonly_command_substitution_runs_jscpd() {
    let parsed = parsed_hook("readonly JSCPD_OUTPUT=\"$(jscpd .)\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_jscpd_is_only_help() {
    let parsed = parsed_hook("jscpd --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_env_help_wraps_jscpd() {
    let parsed = parsed_hook("env --help jscpd .\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "Rust duplication tool missing");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_real_jscpd_and_help_only_dupes_binary_both_exist() {
    let parsed = parsed_hook("jscpd .\ncargo-dupes --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_real_jscpd_and_assignment_only_dupes_both_exist() {
    let parsed = parsed_hook("jscpd .\nDUPES_CMD='cargo dupes --exclude-tests'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "wrong Rust duplication tool");
    assert!(!results[0].inventory);
}
