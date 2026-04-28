use g3rs_hooks_contract_types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement,
};
use guardrail3_check_types::G3Severity;

use crate::contract_required_tools_installed::rule::run_case;

#[test]
fn reports_missing_contract_derived_tool() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assert_contains_missing(&results, "cargo-deny");
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

    assert_all_inventory(&results);
}

#[test]
fn reports_universal_g3rs_and_gitleaks_tools() {
    let results = run_case(
        "#!/bin/sh\ntrue\n",
        Vec::new(),
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assert_contains_missing(&results, "g3rs");
    assert_contains_missing(&results, "gitleaks");
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

    assert_contains_missing(&results, "cargo-machete");
    assert_contains_missing(&results, "cargo-dupes");
}

#[test]
fn path_qualified_tool_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/cargo-deny check\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assert_all_inventory(&results);
}

#[test]
fn path_qualified_universal_g3rs_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path .\ngitleaks detect\ncargo test\n",
        vec!["gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assert_all_inventory(&results);
}

#[test]
fn path_qualified_gitleaks_satisfies_contract() {
    let results = run_case(
        "#!/bin/sh\ng3rs validate --path .\n/usr/local/bin/gitleaks detect\ncargo test\n",
        vec!["g3rs".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assert_all_inventory(&results);
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

    assert_all_inventory(&results);
}

#[test]
fn g3rs_empty_path_does_not_satisfy_installed_tool_contract() {
    let results = run_case(
        "#!/bin/sh\n/usr/local/bin/g3rs validate --path \"\"\ngitleaks detect\ncargo test\n",
        vec!["gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoTest)],
    );

    assert_contains_missing(&results, "g3rs");
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

    assert_contains_missing(&results, "cargo-machete");
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

fn assert_contains_missing(results: &[guardrail3_check_types::G3CheckResult], tool: &str) {
    assert!(
        results.iter().any(|result| {
            !result.inventory()
                && result.id() == "g3rs-hooks/contract-required-tools-installed"
                && result.severity() == G3Severity::Error
                && result.title() == format!("{tool} missing for hook contract")
        }),
        "missing {tool} should be reported; got {results:?}"
    );
}

fn assert_all_inventory(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(!results.is_empty(), "expected contract tool findings");
    assert!(
        results.iter().all(|result| {
            result.inventory()
                && result.id() == "g3rs-hooks/contract-required-tools-installed"
                && result.severity() == G3Severity::Error
        }),
        "all installed-tool findings should be inventory; got {results:?}"
    );
}
