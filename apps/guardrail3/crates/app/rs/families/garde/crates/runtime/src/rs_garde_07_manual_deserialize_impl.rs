use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ManualDeserializeImplInput;

const ID: &str = "RS-GARDE-07";

pub fn check(input: &ManualDeserializeImplInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.target.needs_validate || input.target.has_validate {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!(
            "manual Deserialize impl for `{}` without Validate",
            input.target.type_name
        ),
        message: format!(
            "Manual `Deserialize` impl for `{}` bypasses derive-based garde checks and the type does not also implement `Validate`.",
            input.target.type_name
        ),
        file: Some(input.target.rel_path.clone()),
        line: Some(input.target.line),
        inventory: false,
    });
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
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
#[path = "rs_garde_07_manual_deserialize_impl_tests/mod.rs"]
mod rs_garde_07_manual_deserialize_impl_tests;
