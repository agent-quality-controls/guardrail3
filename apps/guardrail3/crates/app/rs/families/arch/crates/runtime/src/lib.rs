mod facts;
mod inputs;
mod rs_arch_01_escalation_required;
mod rs_arch_02_split_root_workspace_facade;
mod rs_arch_03_split_root_has_internal_members;
mod rs_arch_04_external_roots_no_internal_deps;

use facts::ArchFacts;
use guardrail3_app_rs_family_mapper::{RsArchRoute, RsProjectSurface};
#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
use guardrail3_domain_report::CheckResult;
use inputs::PackageArchInput;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

pub fn check(surface: &RsProjectSurface, route: &RsArchRoute) -> Vec<CheckResult> {
    let tree = surface;
    run_with_facts(&facts::collect(tree, route))
}

pub(crate) fn run_with_facts(facts: &ArchFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for package in &facts.packages {
        let input = PackageArchInput::new(package);
        rs_arch_01_escalation_required::check(&input, &mut results);
        rs_arch_02_split_root_workspace_facade::check(&input, &mut results);
        rs_arch_03_split_root_has_internal_members::check(&input, &mut results);
        rs_arch_04_external_roots_no_internal_deps::check(&input, &mut results);
    }
    results
}

#[cfg(test)]
pub fn family_route_for_tests(tree: &ProjectTree) -> RsArchRoute {
    let scope = guardrail3_app_rs_structure::collect(tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Arch]));
    FamilyMapper::new(tree, &scope, None, &selected, None).map_rs_arch()
}

#[cfg(test)]
pub(crate) fn check_test_tree(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = family_route_for_tests(tree);
    check(&RsProjectSurface::from_tree(tree), &route)
}
