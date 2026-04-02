mod discover;
mod facts;
mod inputs;
mod lint_support;
mod member_policy;
mod workspace_policy;

use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_app_rs_family_view::FamilyView;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
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

pub fn check(surface: &FamilyView, route: &RsCargoRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in InputFailureCargoInput::from_facts(&facts) {
        member_policy::rs_cargo_14_input_failures::check(&input, &mut results);
    }
    for input in InputFailureInventoryCargoInput::from_facts(&facts) {
        member_policy::rs_cargo_14_input_failures::check_inventory(&input, &mut results);
    }

    for input in PolicyRootCargoInput::from_facts(&facts) {
        workspace_policy::rs_cargo_01_workspace_lints::check(&input, &mut results);
        workspace_policy::rs_cargo_02_lint_levels::check(&input, &mut results);
        workspace_policy::rs_cargo_03_allow_inventory::check(&input, &mut results);
        workspace_policy::rs_cargo_05_workspace_metadata::check(&input, &mut results);
        workspace_policy::rs_cargo_07_priority_order::check(&input, &mut results);
        workspace_policy::rs_cargo_08_resolver::check(&input, &mut results);
        workspace_policy::rs_cargo_11_disallowed_macros_deny::check(&input, &mut results);
        workspace_policy::rs_cargo_12_unapproved_allow_entries::check(&input, &mut results);
        workspace_policy::rs_cargo_15_rust_version_policy::check(&input, &mut results);
    }

    for input in WorkspaceMemberCargoInput::from_facts(&facts) {
        member_policy::rs_cargo_04_lint_inheritance::check(&input, &mut results);
        member_policy::rs_cargo_06_no_weakened_overrides::check(&input, &mut results);
        member_policy::rs_cargo_09_member_edition_drift::check(&input, &mut results);
        member_policy::rs_cargo_13_member_local_allows_forbidden::check(&input, &mut results);
    }

    for input in MissingMemberCargoInput::from_facts(&facts) {
        member_policy::rs_cargo_10_missing_member_cargo::check(&input, &mut results);
    }
    for input in MissingMemberInventoryCargoInput::from_facts(&facts) {
        member_policy::rs_cargo_10_missing_member_cargo::check_inventory(&input, &mut results);
    }

    results
}

#[cfg(test)]
pub fn check_test_tree(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    let surface = FamilyView::build(
        tree.root().clone(), tree.structure(), tree.content(),
        &["".to_owned()], &[], &[], None, &[],
    );
    check(
        &surface,
        &test_route_for_checks(&surface, None),
    )
}

#[cfg(test)]
pub fn check_test_tree_with_validation_scope(
    tree: &guardrail3_domain_project_tree::ProjectTree,
    validation_scope: &str,
) -> Vec<CheckResult> {
    let surface = FamilyView::build(
        tree.root().clone(), tree.structure(), tree.content(),
        &["".to_owned()], &[], &[], None, &[],
    );
    check(
        &surface,
        &test_route_for_checks(&surface, Some(validation_scope)),
    )
}

#[cfg(test)]
fn test_route_for_checks(tree: &ProjectTree, validation_scope: Option<&str>) -> RsCargoRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    FamilyMapper::from_legality(&legality, config.as_ref(), &selected, None)
        .with_validation_scope(validation_scope)
        .map_rs_cargo()
}
