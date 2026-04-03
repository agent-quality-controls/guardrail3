// proc_macro2 is a direct dep only to enable span-locations feature for syn.
use proc_macro2 as _;

mod complexity;
mod dependency;
mod facade;
mod facts;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsArchRoute;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

#[cfg(test)]
pub fn family_route_for_tests(tree: &ProjectTree) -> RsArchRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Arch]));
    FamilyMapper::from_legality(&legality, None, &selected, None).map_rs_arch()
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = family_route_for_tests(tree);
    check(tree, &route)
}
