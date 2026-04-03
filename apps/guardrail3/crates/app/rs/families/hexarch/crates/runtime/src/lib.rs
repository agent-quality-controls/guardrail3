mod dependency_facts;
mod dependency_integrity;
mod dependency_policy;
mod facts;
mod inputs;
mod inventory;
mod source_facts;
mod structure;
mod workspace_policy;

extern crate glob as _;
extern crate guardrail3_domain_modules as _;
extern crate guardrail3_outbound_traits as _;
extern crate proc_macro2 as _;
extern crate quote as _;
extern crate semver as _;
extern crate serde_yaml as _;

mod run;
pub use run::check;

#[cfg(test)]
use std::collections::BTreeSet;

#[doc(hidden)]
#[cfg(feature = "api")]
pub use self::dependency_facts::DependencyFamilyFacts;

#[doc(hidden)]
#[cfg(test)]
pub fn collect_dependency_facts_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> DependencyFamilyFacts {
    dependency_facts::collect(tree, route)
}

#[cfg(test)]
pub fn family_route_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsHexarchRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selection = guardrail3_validation_model::RustFamilySelection::new(BTreeSet::from([
        guardrail3_validation_model::RustValidateFamily::Hexarch,
    ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(
        &legality,
        config.as_ref(),
        &selection,
        None,
    )
    .map_rs_hexarch()
}

#[cfg(test)]
fn routed_surface_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsHexarchRoute,
) -> guardrail3_app_rs_family_view::FamilyView {
    let mut extra_file_rels = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    if let Some(repo_root_cargo) = route.repo_root_cargo_rel_path() {
        extra_file_rels.push(repo_root_cargo.to_owned());
    }
    if let Some(guardrail_rel) = route.guardrail_config_rel_path() {
        extra_file_rels.push(guardrail_rel.to_owned());
    }
    let root_rels = route
        .roots()
        .iter()
        .map(|root| root.rel_dir().to_owned())
        .collect::<Vec<_>>();
    guardrail3_app_rs_family_view::FamilyView::build(
        tree.root_path().to_path_buf(),
        tree.structure(),
        tree.content(),
        &root_rels,
        &extra_file_rels,
        &[],
        None,
        &[],
    )
}

#[cfg(test)]
pub fn check_test_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let route = family_route_for_tests(tree);
    let surface = routed_surface_for_tests(tree, &route);
    check(&surface, &route)
}

#[cfg(test)]
pub fn collect_dependency_facts_from_tree_for_tests(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> DependencyFamilyFacts {
    let route = family_route_for_tests(tree);
    collect_dependency_facts_for_tests(tree, &route)
}

#[cfg(test)]

mod tests;
