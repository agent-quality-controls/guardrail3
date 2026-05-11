use g3rs_hooks_contract_types::G3HookCommandRequirement;
use guardrail3_rs_app_types::SupportedFamily;
use guardrail3_rs_validate_command_assertions::cargo_gates as assertions;

use super::super::{
    any_rust_relevant, any_rust_source, cargo_gate_commands, is_rust_relevant_path,
    paths_under_workspace, suppress_gate_stdout,
};

/// Helper that resolves a requirement to its concrete argv. Tests assert against
/// these values rather than literal strings so the tests stay in sync with the
/// single source of truth (`G3HookCommandRequirement::concrete_command`).
fn argv_of(requirement: G3HookCommandRequirement) -> &'static [&'static str] {
    requirement
        .concrete_command()
        .expect("requirement should map to a runnable command in this test")
}

#[test]
fn cargo_gate_commands_full_set_with_all_families_enabled() {
    let enabled = vec![
        SupportedFamily::Cargo,
        SupportedFamily::Fmt,
        SupportedFamily::Clippy,
        SupportedFamily::Deny,
        SupportedFamily::Deps,
        SupportedFamily::Test,
        SupportedFamily::Code,
    ];
    let commands = cargo_gate_commands(&enabled, false, true);
    let expected: Vec<&[&str]> = vec![
        argv_of(G3HookCommandRequirement::ConcreteLockfileCommand),
        argv_of(G3HookCommandRequirement::CargoFmtCheck),
        argv_of(G3HookCommandRequirement::CargoClippyDenyWarnings),
        argv_of(G3HookCommandRequirement::CargoDenyCheck),
        argv_of(G3HookCommandRequirement::CargoMachete),
        argv_of(G3HookCommandRequirement::CargoDupes),
        argv_of(G3HookCommandRequirement::CargoDupesExcludeTests),
        argv_of(G3HookCommandRequirement::CargoTest),
    ];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_includes_fmt_when_fmt_enabled() {
    let enabled = vec![SupportedFamily::Fmt];
    let commands = cargo_gate_commands(&enabled, false, true);
    let expected: Vec<&[&str]> = vec![argv_of(G3HookCommandRequirement::CargoFmtCheck)];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_empty_when_no_families_enabled() {
    let enabled: Vec<SupportedFamily> = Vec::new();
    let commands = cargo_gate_commands(&enabled, false, true);
    let expected: Vec<&[&str]> = Vec::new();
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_skips_dupes_exclude_tests_when_staged_without_rust_source() {
    let enabled = vec![SupportedFamily::Deps];
    let commands = cargo_gate_commands(&enabled, true, false);
    let expected: Vec<&[&str]> = vec![
        argv_of(G3HookCommandRequirement::CargoMachete),
        argv_of(G3HookCommandRequirement::CargoDupes),
    ];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_runs_dupes_exclude_tests_when_workspace_mode() {
    let enabled = vec![SupportedFamily::Deps];
    let commands = cargo_gate_commands(&enabled, false, false);
    let expected: Vec<&[&str]> = vec![
        argv_of(G3HookCommandRequirement::CargoMachete),
        argv_of(G3HookCommandRequirement::CargoDupes),
        argv_of(G3HookCommandRequirement::CargoDupesExcludeTests),
    ];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_skips_disabled_families() {
    let enabled = vec![SupportedFamily::Fmt];
    let commands = cargo_gate_commands(&enabled, false, true);
    let expected: Vec<&[&str]> = vec![argv_of(G3HookCommandRequirement::CargoFmtCheck)];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_gate_commands_dedups_repeated_argv_across_families() {
    // `Cargo` (ConcreteLockfileCommand) and any other family that may also
    // declare ConcreteLockfileCommand should produce only one occurrence.
    let enabled = vec![SupportedFamily::Cargo, SupportedFamily::Cargo];
    let commands = cargo_gate_commands(&enabled, false, true);
    let expected: Vec<&[&str]> = vec![argv_of(G3HookCommandRequirement::ConcreteLockfileCommand)];
    assertions::assert_command_sequence(&commands, &expected);
}

#[test]
fn cargo_metadata_gate_stdout_is_suppressed() {
    assert!(
        suppress_gate_stdout(argv_of(G3HookCommandRequirement::ConcreteLockfileCommand)),
        "cargo metadata emits machine JSON on stdout and must not leak into g3rs output"
    );
}

/// One `is_rust_relevant_path` test case: input path and expected relevance.
type RelevanceCase = (&'static str, bool);

#[test]
fn rust_relevant_path_detection() {
    let cases: &[RelevanceCase] = &[
        ("foo/bar.rs", true),
        ("Cargo.toml", true),
        ("apps/x/Cargo.lock", true),
        ("rust-toolchain.toml", true),
        ("guardrail3-rs.toml", true),
        ("apps/x/.cargo/config.toml", true),
        ("apps/x/foo.ts", false),
        ("README.md", false),
    ];
    for (path, expected) in cases {
        assertions::assert_rust_relevance(path, is_rust_relevant_path(path), *expected);
    }
}

#[test]
fn paths_under_workspace_filters_correctly() {
    let staged = vec![
        "apps/guardrail3-rs/foo.rs".to_owned(),
        "apps/guardrail3-ts/bar.ts".to_owned(),
        "README.md".to_owned(),
    ];
    let result = paths_under_workspace(&staged, "apps/guardrail3-rs");
    assert_eq!(
        result,
        vec!["apps/guardrail3-rs/foo.rs"],
        "filter should keep only paths under apps/guardrail3-rs"
    );

    let result_root = paths_under_workspace(&staged, ".");
    assert_eq!(
        result_root.len(),
        3,
        "filter with `.` workspace should keep all paths"
    );
}

#[test]
fn rust_relevance_helpers() {
    let rust = vec!["apps/x/foo.rs".to_owned()];
    let not_rust = vec!["apps/x/foo.ts".to_owned()];
    assert!(any_rust_relevant(&rust), "rust .rs path should be relevant");
    assert!(any_rust_source(&rust), "rust .rs path should be source");
    assert!(
        !any_rust_relevant(&not_rust),
        "non-rust path should not be relevant"
    );
    assert!(
        !any_rust_source(&not_rust),
        "non-rust path should not be source"
    );
}
