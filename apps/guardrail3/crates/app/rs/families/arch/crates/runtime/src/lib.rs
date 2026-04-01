// proc_macro2 is a direct dep only to enable span-locations feature for syn.
use proc_macro2 as _;

mod complexity;
mod dependency;
mod facade;
mod facts;

use facts::ArchFacts;
use guardrail3_app_rs_family_mapper::{RsArchRoute, RsProjectSurface};
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_mapper::RsProjectSurface as ProjectTree;
#[cfg(test)]
use guardrail3_app_rs_family_mapper::FamilyMapper;
#[cfg(test)]
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
#[cfg(test)]
use std::collections::BTreeSet;

pub fn check(surface: &RsProjectSurface, _route: &RsArchRoute) -> Vec<CheckResult> {
    let facts = facts::collect(surface);
    run_with_facts(&facts)
}

pub(crate) fn run_with_facts(facts: &ArchFacts) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Facade rules: per crate node.
    for node in facts.crate_tree.nodes.values() {
        facade::rs_arch_01_crate_has_facade::check(node, &mut results);

        let lib_surface = node
            .lib_rs_rel
            .as_ref()
            .and_then(|rel| facts.facade_surfaces.get(rel));
        facade::rs_arch_02_lib_facade_only::check(node, lib_surface, &mut results);

        // Complexity rules: per crate node.
        complexity::rs_arch_07_force_crate_split::check(node, &mut results);
        complexity::rs_arch_08_feature_gated_exports::check(node, lib_surface, &mut results);
    }

    // Facade rules: per module directory.
    for module in facts.module_layouts.values() {
        facade::rs_arch_03_mod_rs_required::check(module, &mut results);
    }

    // Facade rules: per mod.rs surface.
    for surface in facts.facade_surfaces.values() {
        if surface.is_mod_rs {
            facade::rs_arch_04_mod_facade_only::check(surface, &mut results);
        }
    }

    // Dependency rules: per dependency edge.
    for edge in &facts.dependency_edges.edges {
        dependency::rs_arch_05_no_boundary_crossing::check(
            edge,
            &facts.crate_tree,
            &mut results,
        );
        dependency::rs_arch_06_shared_flag_required::check(
            edge,
            &facts.crate_tree,
            &mut results,
        );
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
