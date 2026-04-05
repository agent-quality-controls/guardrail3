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
pub(crate) fn tool_input<'a>(
    facts: &'a crate::facts::DepsFacts,
    tool_name: &str,
) -> crate::inputs::ToolDepsInput<'a> {
    let tool = facts
        .tools
        .iter()
        .find(|tool| tool.tool_name == tool_name)
        .expect("expected tool facts");
    crate::inputs::ToolDepsInput::new(tool)
}
pub(crate) fn tool_facts(tool_name: &str, installed: bool) -> crate::facts::DepsFacts {
    crate::facts::DepsFacts {
        tools: vec![crate::facts::ToolFacts {
            tool_name: tool_name.to_owned(),
            installed,
        }],
        lockfiles: vec![crate::facts::LockfileFacts {
            root_rel_dir: String::new(),
            cargo_lock_rel_path: "Cargo.lock".to_owned(),
            cargo_lock_exists: true,
            cargo_lock_ignored: false,
            gitignore_rel_path: Some(".gitignore".to_owned()),
            profile_name: Some("service".to_owned()),
        }],
        dependency_entries: Vec::new(),
        allowlist_coverage: Vec::new(),
        direct_dependency_caps: Vec::new(),
        policy_content_checks: Vec::new(),
        direct_dependency_cap_content_checks: Vec::new(),
        input_failures: Vec::new(),
    }
}
pub(crate) fn run_with_facts(
    facts: &crate::facts::DepsFacts,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::run_with_facts(facts)
}
