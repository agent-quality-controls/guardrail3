use crate::domain::report::Severity;

use super::super::inputs::RustHookCommandInput;
use super::super::test_support::parsed_hook;
use super::check;

#[test]
fn reports_info_when_workspace_flag_missing() {
    let parsed = parsed_hook("cargo test\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn passes_when_workspace_flag_exists() {
    let parsed = parsed_hook("cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_toolchain_prefixed_workspace_test_exists() {
    let parsed = parsed_hook("cargo +nightly test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_wraps_workspace_test() {
    let parsed = parsed_hook("env -i cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_env_split_string_wraps_workspace_test() {
    let parsed = parsed_hook("env -S 'cargo test --workspace'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_path_qualified_cargo_runs_workspace_test() {
    let parsed = parsed_hook("/usr/bin/cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_manifest_path_precedes_workspace_test() {
    let parsed = parsed_hook("cargo --manifest-path Cargo.toml test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_cargo_chdir_precedes_workspace_test() {
    let parsed = parsed_hook("cargo -C tools test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_attached_jobs_flag_precedes_workspace_test() {
    let parsed = parsed_hook("cargo -j4 test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_wrapper_runs_workspace_test() {
    let parsed = parsed_hook("bash -lc 'cargo test --workspace'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_shell_option_value_precedes_workspace_test_script() {
    let parsed = parsed_hook("sh -o errexit -c 'cargo test --workspace'\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_command_wrapper_runs_workspace_test() {
    let parsed = parsed_hook("command cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_exec_wrapper_runs_workspace_test() {
    let parsed = parsed_hook("exec cargo test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_called_function_runs_workspace_test() {
    let parsed = parsed_hook("run_tests() {\n    cargo test --workspace\n}\nrun_tests\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn passes_when_called_helper_chain_runs_workspace_test() {
    let parsed = parsed_hook(
        "run_tests() {\n    cargo test --workspace\n}\nprecommit_checks() {\n    run_tests\n}\nprecommit_checks\n",
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
fn passes_when_workspace_test_is_left_side_of_chained_command() {
    let parsed = parsed_hook("cargo test --workspace && echo ok\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}

#[test]
fn does_not_count_echoed_workspace_command() {
    let parsed = parsed_hook("echo \"cargo test --workspace\"\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_near_match_test_subcommand_is_used() {
    let parsed = parsed_hook("cargo testx --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_workspace_flag_is_only_forwarded_to_test_binary() {
    let parsed = parsed_hook("cargo test -- --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_workspace_flag_is_only_near_match() {
    let parsed = parsed_hook("cargo test --workspace-hack\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_workspace_test_command_is_only_help() {
    let parsed = parsed_hook("cargo test --workspace --help\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}

#[test]
fn warns_when_global_help_precedes_workspace_test() {
    let parsed = parsed_hook("cargo --help test --workspace\n");
    let input = RustHookCommandInput {
        rel_path: ".githooks/pre-commit",
        parsed: &parsed,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(!results[0].inventory);
}
