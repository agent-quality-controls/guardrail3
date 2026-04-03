mod discover;
mod facts;
mod inputs;
mod lint_support;
mod member_policy;
mod workspace_policy;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsCargoRoute;
#[cfg(test)]
use guardrail3_domain_config::types::GuardrailConfig;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

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
