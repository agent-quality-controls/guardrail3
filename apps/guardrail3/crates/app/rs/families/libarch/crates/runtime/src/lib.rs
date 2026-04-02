mod facts;
mod inputs;
mod rs_libarch_04_exact_layered_crate_set;
mod rs_libarch_07_core_no_api_dep;
mod rs_libarch_08_core_no_infra_dep;
mod rs_libarch_09_api_no_infra_dep;
mod rs_libarch_10_infra_not_public_surface;
mod rs_libarch_11_root_facade_exports_api;

use facts::LibarchFacts;
use guardrail3_app_rs_family_mapper::RsLibarchRoute;
use guardrail3_app_rs_family_view::FamilyView;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
use guardrail3_domain_report::CheckResult;
use inputs::PackageLibarchInput;

#[cfg(test)]
use guardrail3_app_rs_family_libarch_assertions as _;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
use test_support as _;

pub fn check(surface: &FamilyView, route: &RsLibarchRoute) -> Vec<CheckResult> {
    let tree = surface;
    run_with_facts(&facts::collect(tree, route))
}

pub(crate) fn run_with_facts(facts: &LibarchFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for package in &facts.packages {
        let input = PackageLibarchInput::new(package);
        rs_libarch_04_exact_layered_crate_set::check(&input, &mut results);
        rs_libarch_07_core_no_api_dep::check(&input, &mut results);
        rs_libarch_08_core_no_infra_dep::check(&input, &mut results);
        rs_libarch_09_api_no_infra_dep::check(&input, &mut results);
        rs_libarch_10_infra_not_public_surface::check(&input, &mut results);
        rs_libarch_11_root_facade_exports_api::check(&input, &mut results);
    }
    results
}

#[cfg(test)]
pub fn family_route_for_tests(tree: &ProjectTree) -> RsLibarchRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Libarch]));
    FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_libarch()
}

#[cfg(test)]
pub(crate) fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = family_route_for_tests(tree);
    check(&FamilyView::from_tree(tree), &route)
}

#[cfg(test)]
pub(crate) fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    check_test_tree(&tree)
}
