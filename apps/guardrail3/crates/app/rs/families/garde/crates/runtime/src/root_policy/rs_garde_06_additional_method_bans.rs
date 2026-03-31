use guardrail3_domain_report::{CheckResult, Severity};

use super::garde_support::{ADDITIONAL_METHOD_BANS, extract_ban_paths, missing_bans};
use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-06";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(parsed) = input.root.clippy_parsed.as_ref() else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cannot verify additional garde method bans".to_owned(),
            input.root.clippy_parse_error.clone().unwrap_or_else(|| {
                "No covering clippy configuration found for additional garde method-ban validation.".to_owned()
            }),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
        return;
    };

    let found = extract_ban_paths(parsed, "disallowed-methods");
    let missing = missing_bans(&found, ADDITIONAL_METHOD_BANS);
    if missing.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "additional garde method bans present".to_owned(),
                "All additional garde deserialization entry-point bans are present in the covering clippy configuration.".to_owned(),
                input.root.clippy_rel_path.clone(),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "missing additional garde method bans".to_owned(),
            format!(
                "Missing additional garde deserialization bans from `disallowed-methods`: {}.",
                missing.join(", ")
            ),
            input.root.clippy_rel_path.clone(),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(super) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}

#[cfg(test)]
pub(super) fn remove_clippy_ban_path(clippy_toml: &str, key: &str, path: &str) -> String {
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

#[cfg(test)]
pub(super) fn run_family(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
) -> Vec<guardrail3_domain_report::CheckResult> {
    {
        let scope = guardrail3_app_rs_structure::collect(tree);
        let config = tree.file_content("guardrail3.toml").and_then(|content| {
            toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
        });
        let selected = guardrail3_validation_model::RustFamilySelection::new(
            std::collections::BTreeSet::from([
                guardrail3_validation_model::RustValidateFamily::Garde,
            ]),
        );
        let route = guardrail3_app_rs_family_mapper::FamilyMapper::new(
            tree,
            &scope,
            config.as_ref(),
            &selected,
            None,
        )
        .map_rs_garde();
        super::check_test_tree(tree, &route)
    }
}

#[cfg(test)]
#[path = "rs_garde_06_additional_method_bans_tests/mod.rs"]
mod rs_garde_06_additional_method_bans_tests;
