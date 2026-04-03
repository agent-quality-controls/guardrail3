use guardrail3_app_rs_family_mapper::RsCargoRoute;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::CheckResult;

use crate::discover::collect;
use crate::inputs::{
    InputFailureCargoInput, InputFailureInventoryCargoInput, MissingMemberCargoInput,
    MissingMemberInventoryCargoInput, PolicyRootCargoInput, WorkspaceMemberCargoInput,
};

pub fn check(surface: &FamilyView, route: &RsCargoRoute) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    for input in InputFailureCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_14_input_failures::check(&input, &mut results);
    }
    for input in InputFailureInventoryCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_14_input_failures::check_inventory(&input, &mut results);
    }

    for input in PolicyRootCargoInput::from_facts(&facts) {
        crate::workspace_policy::rs_cargo_01_workspace_lints::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_02_lint_levels::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_03_allow_inventory::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_05_workspace_metadata::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_07_priority_order::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_08_resolver::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_11_disallowed_macros_deny::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_12_unapproved_allow_entries::check(&input, &mut results);
        crate::workspace_policy::rs_cargo_15_rust_version_policy::check(&input, &mut results);
    }

    for input in WorkspaceMemberCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_04_lint_inheritance::check(&input, &mut results);
        crate::member_policy::rs_cargo_06_no_weakened_overrides::check(&input, &mut results);
        crate::member_policy::rs_cargo_09_member_edition_drift::check(&input, &mut results);
        crate::member_policy::rs_cargo_13_member_local_allows_forbidden::check(&input, &mut results);
    }

    for input in MissingMemberCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_10_missing_member_cargo::check(&input, &mut results);
    }
    for input in MissingMemberInventoryCargoInput::from_facts(&facts) {
        crate::member_policy::rs_cargo_10_missing_member_cargo::check_inventory(&input, &mut results);
    }

    results
}
