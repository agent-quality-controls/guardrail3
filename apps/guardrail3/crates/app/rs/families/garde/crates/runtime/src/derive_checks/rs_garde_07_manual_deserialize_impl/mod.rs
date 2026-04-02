use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ManualDeserializeImplInput;

const ID: &str = "RS-GARDE-07";

pub fn check(input: &ManualDeserializeImplInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.target.needs_validate || input.target.has_validate {
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
            "manual Deserialize impl for `{}` without Validate",
            input.target.type_name
        ),
    format!(
            "Manual `Deserialize` impl for `{}` bypasses derive-based garde checks and the type does not also implement `Validate`.",
            input.target.type_name
        ),
    Some(input.target.rel_path.clone()),
    Some(input.target.line),
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
