use g3rs_hooks_contract_types::{G3HookCommandRequirement, G3HookRequirement};

use crate::contract_required_tools_installed::rule::run_case;

#[test]
fn reports_missing_contract_derived_tool() {
    let results = run_case(
        "#!/bin/sh\ncargo deny check\n",
        vec!["g3rs".to_owned(), "gitleaks".to_owned()],
        vec![requirement(G3HookCommandRequirement::CargoDenyCheck)],
    );

    assert!(
        results.iter().any(|result| {
            !result.inventory() && result.title() == "cargo-deny missing for hook contract"
        }),
        "missing cargo-deny should be reported from hook contract"
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

    assert!(
        results
            .iter()
            .all(guardrail3_check_types::G3CheckResult::inventory),
        "installed contract-derived tools should only emit inventory"
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
