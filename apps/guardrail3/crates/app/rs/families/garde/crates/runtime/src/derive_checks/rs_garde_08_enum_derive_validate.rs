use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DerivedBoundaryTypeInput;
use super::parse::BoundaryKind;

const ID: &str = "RS-GARDE-08";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Enum || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("enum `{}` missing Validate derive", input.target.name),
    format!(
            "Enum `{}` derives {} and has non-primitive payload fields, but does not derive `Validate`.",
            input.target.name,
            input.target.boundary_macros.join(", ")
        ),
    Some(input.target.rel_path.clone()),
    Some(input.target.line),
    false,
    ));
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
#[path = "rs_garde_08_enum_derive_validate_tests/mod.rs"]
mod rs_garde_08_enum_derive_validate_tests;
