use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use std::collections::BTreeMap;

use crate::facts::QueryAsMacroFacts;
use crate::inputs::QueryAsMacroInput;

const ID: &str = "RS-GARDE-09";

pub fn check(input: &QueryAsMacroInput<'_>, results: &mut Vec<CheckResult>) {
    match input.macro_use.escape_hatch_reason.as_deref() {
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "sqlx query_as missing reason".to_owned(),
            format!(
                "`{}` bypasses derive-based garde boundary checks without a matching escape-hatch reason.",
                input.macro_use.macro_name
            ),
            Some(input.macro_use.rel_path.clone()),
            Some(input.macro_use.line),
            false,
        )),
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "sqlx query_as requires validation review".to_owned(),
                format!(
                    "`{}` bypasses derive-based garde boundary checks with documented reason `{reason}`. Review the target type and ensure validated input handling is explicit.",
                    input.macro_use.macro_name
                ),
                Some(input.macro_use.rel_path.clone()),
                Some(input.macro_use.line),
                false,
            )),
            Err(issue) => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "sqlx query_as reason too weak".to_owned(),
                format!(
                    "`{}` bypasses derive-based garde boundary checks with a weak reason: {}.",
                    input.macro_use.macro_name,
                    issue.message()
                ),
                Some(input.macro_use.rel_path.clone()),
                Some(input.macro_use.line),
                false,
            )),
        },
    }
}

pub fn check_count<'a>(
    macro_uses: impl IntoIterator<Item = &'a QueryAsMacroFacts>,
    results: &mut Vec<CheckResult>,
) {
    let mut counts = BTreeMap::<String, usize>::new();
    for macro_use in macro_uses {
        *counts.entry(macro_use.rel_path.clone()).or_default() += 1;
    }

    for (rel_path, count) in counts {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "sqlx query_as count".to_owned(),
            format!("`{rel_path}` has {count} sqlx query_as escape hatches."),
            None,
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
#[path = "rs_garde_09_query_as_inventory_tests/mod.rs"]
mod rs_garde_09_query_as_inventory_tests;
