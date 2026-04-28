use g3rs_hooks_contract_types::{G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern};
use guardrail3_check_types::G3Severity;

use crate::contract_critical_command_not_fail_open::rule::run_case;

#[test]
fn contract_binary_critical_command_cannot_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo-deny check || true\n",
        vec![requirement(G3HookCriticalCommand::Binary(
            "cargo-deny".to_owned(),
        ))],
    );

    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-critical-command-not-fail-open"
                && result.severity() == G3Severity::Warn
                && result.title() == "contract-critical hook command is fail-open"
        }),
        "fail-open contract-critical binary command should be reported"
    );
}

#[test]
fn contract_cargo_subcommand_cannot_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy -- -D warnings || echo soft\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "fail-open contract-critical cargo subcommand should be reported"
    );
}

#[test]
fn non_critical_command_can_fail_open_for_this_rule() {
    let results = run_case(
        "#!/bin/sh\nnpm test || true\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assert!(
        results.is_empty(),
        "non-critical command should not be reported by contract-critical rule"
    );
}

#[test]
fn universal_g3rs_cannot_fail_open_without_contract_requirements() {
    let results = run_case("#!/bin/sh\ng3rs validate --path . || true\n", Vec::new());

    assert!(
        results.iter().any(|result| !result.inventory()),
        "universal g3rs critical command should be reported without contract requirements"
    );
}

#[test]
fn universal_gitleaks_cannot_fail_open_without_contract_requirements() {
    let results = run_case("#!/bin/sh\ngitleaks detect || true\n", Vec::new());

    assert!(
        results.iter().any(|result| !result.inventory()),
        "universal gitleaks critical command should be reported without contract requirements"
    );
}

#[test]
fn negated_if_without_failure_exit_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif ! g3rs validate --path .; then echo skip; fi\n",
        Vec::new(),
    );

    assert!(
        results.iter().any(|result| !result.inventory()),
        "negated if branch that only echoes should be reported as fail-open"
    );
}

#[test]
fn negated_if_with_failure_exit_is_not_fail_open() {
    let results = run_case(
        "#!/bin/sh\nif ! g3rs validate --path .; then\n    echo failed\n    exit 1\nfi\n",
        Vec::new(),
    );

    assert!(
        results.is_empty(),
        "negated if branch with explicit non-zero exit should not be reported"
    );
}

#[test]
fn or_exit_zero_is_fail_open() {
    let results = run_case("#!/bin/sh\ng3rs validate --path . || exit 0\n", Vec::new());

    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-critical-command-not-fail-open"
                && result.severity() == G3Severity::Warn
        }),
        "critical command followed by `|| exit 0` should be reported"
    );
}

#[test]
fn or_printf_is_fail_open() {
    let results = run_case(
        "#!/bin/sh\ncargo clippy -- -D warnings || printf 'ignored\\n'\n",
        vec![requirement(G3HookCriticalCommand::CargoSubcommand(
            "clippy".to_owned(),
        ))],
    );

    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-critical-command-not-fail-open"
                && result.severity() == G3Severity::Warn
        }),
        "critical command followed by `|| printf` should be reported"
    );
}

#[test]
fn negated_if_with_failure_helper_is_not_fail_open() {
    let results = run_case(
        "#!/bin/sh\ndie() { exit 1; }\nif ! g3rs validate --path .; then die; fi\n",
        Vec::new(),
    );

    assert!(
        results.is_empty(),
        "negated if branch calling a helper that exits non-zero should not be reported"
    );
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

    assert!(
        results.iter().any(|result| !result.inventory()),
        "called function with fail-open contract-critical command should be reported"
    );
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
