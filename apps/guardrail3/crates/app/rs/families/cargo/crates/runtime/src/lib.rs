mod discover;
mod facts;
mod inputs;
mod lint_support;
mod rs_cargo_01_workspace_lints;
mod rs_cargo_02_lint_levels;
mod rs_cargo_03_allow_inventory;
mod rs_cargo_04_lint_inheritance;
mod rs_cargo_05_workspace_metadata;
mod rs_cargo_06_no_weakened_overrides;
mod rs_cargo_07_priority_order;
mod rs_cargo_08_resolver;
mod rs_cargo_09_member_edition_drift;
mod rs_cargo_10_missing_member_cargo;
mod rs_cargo_11_disallowed_macros_deny;
mod rs_cargo_12_unapproved_allow_entries;
mod rs_cargo_13_member_local_allows_forbidden;
mod rs_cargo_14_input_failures;
mod rs_cargo_15_rust_version_policy;

use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::{FamilyMapper, RsCargoRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use self::discover::collect;
use self::inputs::{
    InputFailureCargoInput, MissingMemberCargoInput, PolicyRootCargoInput,
    WorkspaceMemberCargoInput,
};

pub fn check(tree: &ProjectTree, route: &RsCargoRoute) -> Vec<CheckResult> {
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in InputFailureCargoInput::from_facts(&facts) {
        rs_cargo_14_input_failures::check(&input, &mut results);
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

    results
}

pub fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_cargo();
    check(tree, &route)
}
