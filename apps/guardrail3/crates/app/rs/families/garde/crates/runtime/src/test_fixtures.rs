use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::{FamilyMapper, RsGardeRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub(crate) fn canonical_clippy_toml() -> String {
    build_clippy_toml("service", false, true, "", "")
}

pub(crate) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
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

pub(crate) fn family_route(
    tree: &ProjectTree,
    scoped_files: Option<&BTreeSet<String>>,
) -> RsGardeRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = parse_guardrail_config(tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Garde]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selected, scoped_files).map_rs_garde()
}

pub(crate) fn run_family(tree: &ProjectTree) -> Vec<CheckResult> {
    super::check(tree, &family_route(tree, None))
}

fn parse_guardrail_config(tree: &ProjectTree) -> Option<GuardrailConfig> {
    tree.file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok())
}
