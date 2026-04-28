use std::collections::BTreeSet;

use g3rs_hooks_contract_types::G3HookRequirement;
use guardrail3_check_types::G3CheckResult;

/// Checks that all Rust owner families publish hook contracts into the process runner.
///
/// # Panics
///
/// Panics if the owner set does not match the required Rust family inventory.
pub fn assert_rust_hook_contract_owners(requirements: Vec<G3HookRequirement>) {
    let owners = requirements
        .into_iter()
        .map(|requirement| requirement.owner_family)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        owners,
        BTreeSet::from([
            "apparch".to_owned(),
            "arch".to_owned(),
            "cargo".to_owned(),
            "clippy".to_owned(),
            "code".to_owned(),
            "deny".to_owned(),
            "deps".to_owned(),
            "fmt".to_owned(),
            "garde".to_owned(),
            "release".to_owned(),
            "test".to_owned(),
            "toolchain".to_owned(),
            "topology".to_owned(),
        ]),
        "Rust hook requirements should include every family-owned hook contract"
    );
}

/// Checks that hooks source checks receive family-owned hook contracts.
///
/// # Panics
///
/// Panics if no contract-derived missing-command result is emitted.
pub fn assert_hooks_runner_injects_contracts(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-hooks/required-contract-command-present"
                && !result.inventory()
                && result.message().contains("Owner families:")
        }),
        "hooks runner should inject family hook contracts into source checks"
    );
}
