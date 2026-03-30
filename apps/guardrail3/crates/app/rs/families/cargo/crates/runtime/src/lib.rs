mod discover;
mod facts;
mod inputs;
mod lint_support;
#[path = "workspace_policy/rs_cargo_01_workspace_lints.rs"]
mod rs_cargo_01_workspace_lints;
#[path = "workspace_policy/rs_cargo_02_lint_levels.rs"]
mod rs_cargo_02_lint_levels;
#[path = "workspace_policy/rs_cargo_03_allow_inventory.rs"]
mod rs_cargo_03_allow_inventory;
#[path = "member_policy/rs_cargo_04_lint_inheritance.rs"]
mod rs_cargo_04_lint_inheritance;
#[path = "workspace_policy/rs_cargo_05_workspace_metadata.rs"]
mod rs_cargo_05_workspace_metadata;
#[path = "member_policy/rs_cargo_06_no_weakened_overrides.rs"]
mod rs_cargo_06_no_weakened_overrides;
#[path = "workspace_policy/rs_cargo_07_priority_order.rs"]
mod rs_cargo_07_priority_order;
#[path = "workspace_policy/rs_cargo_08_resolver.rs"]
mod rs_cargo_08_resolver;
#[path = "member_policy/rs_cargo_09_member_edition_drift.rs"]
mod rs_cargo_09_member_edition_drift;
#[path = "member_policy/rs_cargo_10_missing_member_cargo.rs"]
mod rs_cargo_10_missing_member_cargo;
#[path = "workspace_policy/rs_cargo_11_disallowed_macros_deny.rs"]
mod rs_cargo_11_disallowed_macros_deny;
#[path = "workspace_policy/rs_cargo_12_unapproved_allow_entries.rs"]
mod rs_cargo_12_unapproved_allow_entries;
#[path = "member_policy/rs_cargo_13_member_local_allows_forbidden.rs"]
mod rs_cargo_13_member_local_allows_forbidden;
#[path = "member_policy/rs_cargo_14_input_failures.rs"]
mod rs_cargo_14_input_failures;
#[path = "workspace_policy/rs_cargo_15_rust_version_policy.rs"]
mod rs_cargo_15_rust_version_policy;

use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

use self::discover::collect;
use self::inputs::{
    InputFailureCargoInput, InputFailureInventoryCargoInput, MissingMemberCargoInput,
    MissingMemberInventoryCargoInput, PolicyRootCargoInput, WorkspaceMemberCargoInput,
};

pub fn check(tree: &ProjectTree, route: &RsCargoRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in InputFailureCargoInput::from_facts(&facts) {
        rs_cargo_14_input_failures::check(&input, &mut results);
    }
    for input in InputFailureInventoryCargoInput::from_facts(&facts) {
        rs_cargo_14_input_failures::check_inventory(&input, &mut results);
    }

    for input in PolicyRootCargoInput::from_facts(&facts) {
        rs_cargo_01_workspace_lints::check(&input, &mut results);
        rs_cargo_02_lint_levels::check(&input, &mut results);
        rs_cargo_03_allow_inventory::check(&input, &mut results);
        rs_cargo_05_workspace_metadata::check(&input, &mut results);
        rs_cargo_07_priority_order::check(&input, &mut results);
        rs_cargo_08_resolver::check(&input, &mut results);
        rs_cargo_11_disallowed_macros_deny::check(&input, &mut results);
        rs_cargo_12_unapproved_allow_entries::check(&input, &mut results);
        rs_cargo_15_rust_version_policy::check(&input, &mut results);
    }

    for input in WorkspaceMemberCargoInput::from_facts(&facts) {
        rs_cargo_04_lint_inheritance::check(&input, &mut results);
        rs_cargo_06_no_weakened_overrides::check(&input, &mut results);
        rs_cargo_09_member_edition_drift::check(&input, &mut results);
        rs_cargo_13_member_local_allows_forbidden::check(&input, &mut results);
    }

    for input in MissingMemberCargoInput::from_facts(&facts) {
        rs_cargo_10_missing_member_cargo::check(&input, &mut results);
    }
    for input in MissingMemberInventoryCargoInput::from_facts(&facts) {
        rs_cargo_10_missing_member_cargo::check_inventory(&input, &mut results);
    }

    results
}

#[cfg(test)]
pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_cargo();
    check(tree, &route)
}

#[cfg(test)]
pub fn check_test_tree_with_validation_scope(
    tree: &ProjectTree,
    validation_scope: &str,
) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None)
        .with_validation_scope(Some(validation_scope))
        .map_rs_cargo();
    check(tree, &route)
}
