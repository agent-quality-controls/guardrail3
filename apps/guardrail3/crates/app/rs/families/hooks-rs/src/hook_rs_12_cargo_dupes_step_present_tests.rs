use guardrail3_domain_report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;
use super::check;

#[test]
fn warns_when_cargo_dupes_is_only_prose() {
    let parsed = parsed_hook("echo \"cargo dupes check\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_cargo_dupes_subcommand_exists() {
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
fn passes_when_cargo_dupes_binary_exists() {
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
fn passes_when_env_wraps_cargo_dupes_subcommand() {
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
fn passes_when_env_split_string_wraps_cargo_dupes_subcommand() {
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
fn passes_when_toolchain_prefixed_cargo_dupes_exists() {
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
fn passes_when_path_qualified_cargo_dupes_subcommand_exists() {
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
fn passes_when_path_qualified_cargo_dupes_binary_exists() {
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
fn passes_when_shell_wrapper_runs_cargo_dupes() {
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
fn passes_when_called_function_runs_cargo_dupes() {
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
fn passes_when_called_helper_chain_runs_cargo_dupes() {
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
fn passes_when_left_side_of_chained_command_runs_cargo_dupes() {
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
fn warns_when_cargo_dupes_subcommand_is_only_help() {
    let parsed = parsed_hook("cargo dupes --help\n");
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
    let parsed = parsed_hook("cargo-dupes --help\n");
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
fn warns_when_cargo_dupes_only_exists_inside_uncalled_function() {
    let parsed = parsed_hook("run_dupes() {\n    cargo dupes check --exclude-tests\n}\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
}
