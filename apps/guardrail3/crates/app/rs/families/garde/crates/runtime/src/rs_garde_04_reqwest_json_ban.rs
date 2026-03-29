use guardrail3_domain_report::{CheckResult, Severity};

use super::garde_support::{REQWEST_JSON_BAN, extract_ban_paths};
use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-04";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "cannot verify reqwest garde ban".to_owned(),
            message: input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for reqwest garde-ban validation."
                    .to_owned()
            }),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    if found.contains(REQWEST_JSON_BAN) {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "reqwest garde ban present".to_owned(),
                message:
                    "`reqwest::Response::json` is banned in the covering clippy configuration."
                        .to_owned(),
                file: input.root.clippy_rel_path.clone(),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "missing reqwest garde ban".to_owned(),
            message: "Missing `reqwest::Response::json` from `disallowed-methods`.".to_owned(),
            file: input.root.clippy_rel_path.clone(),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}

#[cfg(test)]
#[allow(dead_code)]
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

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn run_family(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok());
    let selected = guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([guardrail3_validation_model::RustValidateFamily::Garde]));
    let route = guardrail3_app_rs_family_mapper::FamilyMapper::new(tree, &scope, config.as_ref(), &selected, None).map_rs_garde();
    super::check(tree, &route)
}
}

#[cfg(test)]
#[path = "rs_garde_04_reqwest_json_ban_tests/mod.rs"]
mod rs_garde_04_reqwest_json_ban_tests;
