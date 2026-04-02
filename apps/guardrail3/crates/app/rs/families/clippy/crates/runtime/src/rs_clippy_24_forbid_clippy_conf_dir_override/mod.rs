mod rule;
pub use rule::{check, check_clean};
#[cfg(test)]
use crate::inputs::CargoConfigOverrideInput;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
pub(crate) fn run_for_tests(tree: &ProjectTree) -> Vec<CheckResult> {
    let facts = super::facts::collect_for_tests(tree);
    let mut results = Vec::new();
    if facts.cargo_config_overrides.is_empty() {
        check_clean(&mut results);
    } else {
        for override_facts in &facts.cargo_config_overrides {
            check(&CargoConfigOverrideInput::new(override_facts), &mut results);
        }
    }
    results
}
#[cfg(test)]
pub(crate) fn run_with_validation_scope_for_tests(
    tree: &ProjectTree,
    validation_scope: &str,
) -> Vec<CheckResult> {
    let facts = super::facts::collect_with_validation_scope_for_tests(tree, validation_scope);
    let mut results = Vec::new();
    if facts.cargo_config_overrides.is_empty() {
        check_clean(&mut results);
    } else {
        for override_facts in &facts.cargo_config_overrides {
            check(&CargoConfigOverrideInput::new(override_facts), &mut results);
        }
    }
    results
}
#[cfg(test)]
pub(crate) fn run_family_with_validation_scope_for_tests(
    tree: &ProjectTree,
    validation_scope: &str,
) -> Vec<CheckResult> {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Clippy,
        ]));
    let route =
        guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
            .with_validation_scope(Some(validation_scope))
            .map_rs_clippy();
    crate::check(tree, &route)
}

#[cfg(test)]
mod tests;
