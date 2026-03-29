use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::QueryAsMacroInput;

const ID: &str = "RS-GARDE-09";

pub fn check(input: &QueryAsMacroInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "sqlx query_as requires validation review".to_owned(),
            message: format!(
                "`{}` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
                input.macro_use.macro_name
            ),
            file: Some(input.macro_use.rel_path.clone()),
            line: Some(input.macro_use.line),
            inventory: false,
        }
        .as_inventory(),
    );
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
#[path = "rs_garde_09_query_as_inventory_tests/mod.rs"]
mod rs_garde_09_query_as_inventory_tests;
