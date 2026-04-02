use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-12";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.nested_validated || input.field.has_garde_skip || input.field.has_garde_dive {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "nested validated field `{}` missing garde(dive)",
            input.field.field_name
        ),
    format!(
            "Field `{}` in validated boundary `{}` points at validated nested type `{}` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
    Some(input.field.rel_path.clone()),
    Some(input.field.line),
    false,
    ));
}

#[cfg(test)]
pub(crate) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}

#[cfg(test)]
pub(crate) fn run_family(
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

#[cfg(test)]

mod tests;
