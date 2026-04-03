pub(crate) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}
pub(super) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
    {
        let mut parsed = toml::from_str::<toml::Value>(clippy_toml).expect("valid clippy TOML");
        let entries = parsed
            .get_mut(key)
            .and_then(toml::Value::as_array_mut)
            .expect("expected ban array");
        entries.retain(|entry| {
            entry
                .get("path")
                .and_then(toml::Value::as_str)
                .or_else(|| entry.as_str())
                != Some(path)
        });
        toml::to_string(&parsed).expect("serialize clippy TOML")
    }
}
pub(super) fn run_family(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    {
        let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
        let config = tree.file_content("guardrail3.toml").and_then(|content| {
            toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
        });
        let selected = guardrail3_validation_model::RustFamilySelection::new(
            std::collections::BTreeSet::from([
                guardrail3_validation_model::RustValidateFamily::Garde,
            ]),
        );
        let route = guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(
        &legality,
            config.as_ref(),
            &selected,
            None,
        )
        .map_rs_garde();
        crate::check_test_tree(tree, &route)
    }
}
