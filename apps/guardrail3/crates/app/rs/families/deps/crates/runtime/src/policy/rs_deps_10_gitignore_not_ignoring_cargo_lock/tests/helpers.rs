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
pub(crate) fn lockfile_facts(
    cargo_lock_exists: bool,
    cargo_lock_ignored: bool,
    root_profile_name: Option<&str>,
) -> crate::facts::DepsFacts {
    crate::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![crate::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists,
            cargo_lock_ignored,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: root_profile_name.map(str::to_owned),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: Vec::new(),
        policy_content_checks: Vec::new(),
        direct_dependency_cap_content_checks: Vec::new(),
        input_failures: Vec::new(),
    }
}
pub(crate) fn lockfile_input<'a>(
    facts: &'a crate::facts::DepsFacts,
) -> crate::inputs::LockfileDepsInput<'a> {
    crate::inputs::LockfileDepsInput::new(facts.lockfiles.first().expect("expected lockfile facts"))
}
pub(crate) fn run_with_facts(
    facts: &crate::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}
