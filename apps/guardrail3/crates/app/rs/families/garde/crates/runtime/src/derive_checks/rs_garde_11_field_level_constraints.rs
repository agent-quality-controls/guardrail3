use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::BoundaryFieldInput;

const ID: &str = "RS-GARDE-11";

pub fn check(input: &BoundaryFieldInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.field.requires_field_validation
        || input.field.has_garde_skip
        || input.field.nested_validated
        || input.field.has_garde_dive
        || input.field.has_meaningful_garde_rule
    {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "boundary field `{}` missing garde validator",
            input.field.field_name
        ),
    format!(
            "Field `{}` in validated boundary `{}` has type `{}` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
            input.field.field_name, input.field.boundary_name, input.field.field_type
        ),
    Some(input.field.rel_path.clone()),
    Some(input.field.line),
    false,
    ));
}

#[cfg(test)]
pub(super) fn canonical_clippy_toml() -> String {
    guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", "")
}

#[cfg(test)]
pub(super) fn run_family(
    tree: &guardrail3_app_rs_family_view::FamilyView,
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
        crate::check_test_tree(tree, &route)
    }
}

#[cfg(test)]
#[path = "rs_garde_11_field_level_constraints_tests/mod.rs"]
mod rs_garde_11_field_level_constraints_tests;
