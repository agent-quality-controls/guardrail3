use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::GardeInputFailureInput;

const ID: &str = "RS-GARDE-10";

pub fn check(input: &GardeInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "garde-family input failure".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
pub(super) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}

#[cfg(test)]
pub(super) fn run_family(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    {
        let scope = guardrail3_app_rs_placement::collect(tree);
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
        super::check(tree, &route)
    }
}

#[cfg(test)]
#[path = "rs_garde_10_input_failures_tests/mod.rs"]
mod rs_garde_10_input_failures_tests;
