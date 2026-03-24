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

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::discover::collect;
use self::inputs::{
    InputFailureCargoInput, MissingMemberCargoInput, PolicyRootCargoInput,
    WorkspaceMemberCargoInput,
};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = collect(tree);
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

#[cfg(test)]
mod test_support;
