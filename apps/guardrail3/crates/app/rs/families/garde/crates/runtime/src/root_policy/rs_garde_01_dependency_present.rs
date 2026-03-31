use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::GardeRootInput;

const ID: &str = "RS-GARDE-01";

pub fn check(input: &GardeRootInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.garde_dependency_present {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "garde dependency found".to_owned(),
                format!(
                    "garde is present in `{}` for this {}. Garde-specific boundary checks are active.",
                    input.root.cargo_rel_path,
                    input.root.kind.label()
                ),
                Some(input.root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "garde dependency missing".to_owned(),
            format!(
                "Missing `garde` dependency in `{}` for this {}. Runtime input validation at Rust adapter boundaries requires garde.",
                input.root.cargo_rel_path,
                input.root.kind.label()
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(super) fn run_family(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    let scope = guardrail3_app_rs_structure::collect(tree);
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
    super::check_test_tree(tree, &route)
}

#[cfg(test)]
#[path = "rs_garde_01_dependency_present_tests/mod.rs"]
mod rs_garde_01_dependency_present_tests;
