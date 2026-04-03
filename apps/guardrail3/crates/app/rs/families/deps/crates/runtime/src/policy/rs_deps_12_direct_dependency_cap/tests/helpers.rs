fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    family_route_with_validation_scope(tree, None)
}
fn family_route_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    validation_scope: Option<&str>,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
        .with_validation_scope(validation_scope)
        .map_rs_deps()
}
pub(crate) fn collected_facts(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    installed: &[&str],
) -> crate::facts::DepsFacts {
    crate::facts::collect(
        tree,
        &family_route(tree),
        &test_support::StubToolChecker::new(installed),
    )
}
pub(crate) fn collected_facts_with_validation_scope(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    installed: &[&str],
    validation_scope: Option<&str>,
) -> crate::facts::DepsFacts {
    crate::facts::collect(
        tree,
        &family_route_with_validation_scope(tree, validation_scope),
        &test_support::StubToolChecker::new(installed),
    )
}
pub(crate) fn direct_dependency_cap_facts(
    crate_name: &str,
    cargo_rel_path: &str,
    unique_direct_dependency_count: usize,
) -> crate::facts::DepsFacts {
    crate::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: Vec::new(),
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: vec![crate::facts::DirectDependencyCapFacts {
            crate_name: crate_name.to_owned(),
            cargo_rel_path: cargo_rel_path.to_owned(),
            unique_direct_dependency_count,
        }],
        input_failures: Vec::new(),
    }
}
pub(crate) fn run_with_facts(
    facts: &crate::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}
