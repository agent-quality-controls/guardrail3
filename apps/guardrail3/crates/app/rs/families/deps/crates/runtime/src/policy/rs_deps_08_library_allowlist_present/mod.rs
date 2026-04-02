mod rule;
pub use rule::{check};

#[cfg(test)]
fn family_route(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> guardrail3_app_rs_family_mapper::RsDepsRoute {
    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Deps,
        ]));
    guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
        .map_rs_deps()
}
#[cfg(test)]
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
#[cfg(test)]
pub(crate) fn coverage_facts(
    profile_name: Option<&str>,
    has_allowlist: bool,
) -> crate::facts::DepsFacts {
    crate::facts::DepsFacts {
        tools: Vec::new(),
        lockfiles: vec![crate::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: vec![crate::facts::AllowlistCoverageFacts {
            crate_name: "core".to_owned(),
            cargo_rel_path: "packages/core/Cargo.toml".to_owned(),
            profile_name: profile_name.map(str::to_owned),
            has_allowlist,
        }],
        direct_dependency_caps: Vec::new(),
        input_failures: Vec::new(),
    }
}
#[cfg(test)]
pub(crate) fn coverage_input<'a>(
    facts: &'a crate::facts::DepsFacts,
    cargo_rel_path: &str,
) -> crate::inputs::AllowlistCoverageDepsInput<'a> {
    let coverage = facts
        .allowlist_coverage
        .iter()
        .find(|coverage| coverage.cargo_rel_path == cargo_rel_path)
        .expect("expected allowlist coverage facts");
    crate::inputs::AllowlistCoverageDepsInput::new(coverage)
}
#[cfg(test)]
pub(crate) fn run_with_facts(
    facts: &crate::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}
#[cfg(test)]

mod tests;
