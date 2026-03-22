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

use crate::domain::project_tree::ProjectTree;
use crate::domain::report::CheckResult;

use self::discover::collect;
use self::inputs::{WorkspaceCargoInput, WorkspaceMemberInput, WorkspaceMembersSetInput};

pub fn check(tree: &ProjectTree) -> Vec<CheckResult> {
    let Some(facts) = collect(tree) else {
        return Vec::new();
    };

    let mut results = Vec::new();
    let workspace = WorkspaceCargoInput::new(&facts.workspace);
    let declared_vs_discovered = WorkspaceMembersSetInput::from_facts(&facts);

    rs_cargo_01_workspace_lints::check(&workspace, &mut results);
    rs_cargo_02_lint_levels::check(&workspace, &mut results);
    rs_cargo_03_allow_inventory::check(&workspace, &mut results);
    rs_cargo_05_workspace_metadata::check(&workspace, &mut results);
    rs_cargo_07_priority_order::check(&workspace, &mut results);
    rs_cargo_08_resolver::check(&workspace, &mut results);
    rs_cargo_04_lint_inheritance::check_missing_member_cargos(&declared_vs_discovered, &mut results);

    for member in &facts.members {
        let input = WorkspaceMemberInput::new(&facts.workspace, member);
        rs_cargo_04_lint_inheritance::check(&input, &mut results);
        rs_cargo_06_no_weakened_overrides::check(&input, &mut results);
        rs_cargo_09_member_edition_drift::check(&input, &mut results);
    }

    results
}

#[cfg(test)]
#[path = "cargo_tests.rs"]
mod tests;
