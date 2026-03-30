use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::DerivedBoundaryTypeInput;
use super::parse::BoundaryKind;

const ID: &str = "RS-GARDE-05";

pub fn check(input: &DerivedBoundaryTypeInput<'_>, results: &mut Vec<CheckResult>) {
    if input.target.boundary_kind != BoundaryKind::Struct || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("struct `{}` missing Validate derive", input.target.name),
        format!(
            "Struct `{}` derives {} but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
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
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree.file_content("guardrail3.toml").and_then(|content| {
        toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(content).ok()
    });
    let selected =
        guardrail3_validation_model::RustFamilySelection::new(std::collections::BTreeSet::from([
            guardrail3_validation_model::RustValidateFamily::Garde,
        ]));
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

#[cfg(test)]
#[path = "rs_garde_05_struct_derive_validate_tests/mod.rs"]
mod rs_garde_05_struct_derive_validate_tests;
