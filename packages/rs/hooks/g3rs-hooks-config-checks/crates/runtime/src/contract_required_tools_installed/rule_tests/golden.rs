use g3rs_hooks_config_checks_assertions::contract_required_tools_installed::rule as assertions;
use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement,
};

use super::super::run_case;

#[test]
fn reports_missing_contract_derived_tool() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-deny missing for hook contract",
            "cargo-deny is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn accepts_installed_contract_derived_tool() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec![
            "cargo-deny".to_owned(),
            "g3rs".to_owned(),
            "gitleaks".to_owned(),
        ],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assertions::assert_all_inventory(&results);
}

#[test]
fn reports_universal_g3rs_and_gitleaks_tools() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        Vec::new(),
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "g3rs missing for hook contract",
            "g3rs is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::error(
            "gitleaks missing for hook contract",
            "gitleaks is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn reports_machete_and_dupes_contract_tools() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![
            requirement(G3HookCommandRequirement::CargoMachete),
            requirement(G3HookCommandRequirement::CargoDupes),
            requirement(G3HookCommandRequirement::CargoDupesExcludeTests),
        ],
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-machete missing for hook contract",
            "cargo-machete is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-dupes missing for hook contract",
            "cargo-dupes is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn path_qualified_tool_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/cargo-deny check\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assertions::assert_all_inventory(&results);
}

#[test]
fn path_qualified_universal_g3rs_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path .\ngitleaks detect\ncargo test\n",
        vec!["gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assertions::assert_all_inventory(&results);
}

#[test]
fn path_qualified_gitleaks_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path .\n/usr/local/bin/gitleaks detect\ncargo test\n",
        vec!["g3rs".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assertions::assert_all_inventory(&results);
}

#[test]
fn path_qualified_machete_and_dupes_satisfy_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/cargo-machete\n/usr/local/bin/cargo-dupes check --exclude-tests\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![
            requirement(G3HookCommandRequirement::CargoMachete),
            requirement(G3HookCommandRequirement::CargoDupes),
            requirement(G3HookCommandRequirement::CargoDupesExcludeTests),
        ],
    );

    assertions::assert_all_inventory(&results);
}

#[test]
fn g3rs_empty_path_does_not_satisfy_installed_tool_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path \"\"\ngitleaks detect\ncargo test\n",
        vec!["gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "g3rs missing for hook contract",
            "g3rs is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

#[test]
fn critical_commands_add_required_tools() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![critical_requirement(
            G3HookCriticalCommand::CargoSubcommand("machete".to_owned()),
        )],
    );

    assertions::assert_contains(
        &results,
        assertions::error(
            "cargo-machete missing for hook contract",
            "cargo-machete is required by a family hook contract but is not available on PATH or via a path-qualified command.",
            ".githooks/pre-commit",
        ),
    );
}

fn requirement(command: G3HookCommandRequirement) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: Vec::new(),
        required_commands: vec![command],
        critical_commands: Vec::new(),
    }
}

fn critical_requirement(command: G3HookCriticalCommand) -> G3HookRequirement {
    G3HookRequirement {
        id: "test/hook-contract".to_owned(),
        owner_family: "test".to_owned(),
        trigger_patterns: Vec::new(),
        required_commands: Vec::new(),
        critical_commands: vec![command],
    }
}
