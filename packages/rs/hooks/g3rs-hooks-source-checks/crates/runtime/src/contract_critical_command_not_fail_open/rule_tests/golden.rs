use g3rs_hooks_contract_types::{G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern};
use g3rs_hooks_source_checks_assertions::contract_critical_command_not_fail_open::rule as assertions;

use super::super::run_case;

#[test]
fn contract_binary_critical_command_cannot_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo-deny check || true\n",
        vec![requirement(G3HookCriticalCommand::Binary(
            "cargo-deny".to_owned(),
        ))],
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn contract_cargo_subcommand_cannot_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy -- -D warnings || echo soft\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn non_critical_command_can_fail_open_for_this_rule() {
    let results = run_case(
        "#!/bin/sh\nnpm test || true\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn universal_g3rs_cannot_fail_open_without_contract_requirements() {
    let results = run_case("#!/bin/sh\ng3rs validate --path . || true\n", Vec::new());

    assertions::assert_fail_open_error(&results);
}

#[test]
fn universal_gitleaks_cannot_fail_open_without_contract_requirements() {
    let results = run_case("#!/bin/sh\ngitleaks detect || true\n", Vec::new());

    assertions::assert_fail_open_error(&results);
}

#[test]
fn negated_if_without_failure_exit_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif ! g3rs validate --path .; then echo skip; fi\n",
        Vec::new(),
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn negated_if_with_failure_exit_is_not_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif ! g3rs validate --path .; then\n    echo failed\n    exit 1\nfi\n",
        Vec::new(),
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn or_exit_zero_is_fail_open() {
    let results = run_case("#!/bin/sh\ng3rs validate --path . || exit 0\n", Vec::new());

    assertions::assert_fail_open_error(&results);
}

#[test]
fn or_printf_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy -- -D warnings || printf 'ignored\\n'\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn or_return_zero_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\nrun() { g3rs validate --path . || return 0; }\nrun\n",
        Vec::new(),
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn exported_command_substitution_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\nexport STATUS=$(g3rs validate --path .)\n",
        Vec::new(),
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn positive_availability_guard_without_failing_else_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif command -v g3rs >/dev/null; then\n    g3rs validate --path .\nelse\n    echo missing\nfi\n",
        Vec::new(),
    );

    assertions::assert_fail_open_error(&results);
}

#[test]
fn positive_availability_guard_with_failing_else_is_not_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif command -v g3rs >/dev/null; then\n    g3rs validate --path .\nelse\n    echo missing\n    exit 1\nfi\n",
        Vec::new(),
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn negated_if_with_failure_helper_is_not_fail_open() {
    let results = run_case(
        "#!/bin/sh\ndie() { exit 1; }\nif ! g3rs validate --path .; then die; fi\n",
        Vec::new(),
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn called_function_with_contract_critical_fail_open_is_reported() {
    let results = run_case(
        r#"#!/bin/sh
run_clippy() {
    cargo clippy -- -D warnings || true
}
run_clippy
"#,
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assertions::assert_fail_open_error(&results);
}

fn requirement(command: G3HookCriticalCommand) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: vec![G3HookTriggerPattern::Glob("**/*.rs".to_owned())],
        required_commands: Vec::new(),
        critical_commands: vec![command],
    }
}
