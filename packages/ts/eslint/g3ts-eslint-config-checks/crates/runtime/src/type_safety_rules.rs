use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::{
    baseline::TYPE_SAFETY_RULES,
    support::{
        error, format_rule_list, info, missing_error_rules, parsed_document, selected_rel_path,
    },
};

const ID: &str = "g3ts-eslint/type-safety-rules";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let missing = missing_error_rules(input, EslintProbeKind::TsSource, TYPE_SAFETY_RULES);
    if missing.is_empty() {
        results.push(info(
            ID,
            "type safety rules enforced",
            "The TS source probe has the extended type-safety rule baseline at error severity."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "type safety rules missing or relaxed",
        format!(
            "The TS source probe must enforce these type-safety rules at error severity: {}.",
            format_rule_list(&missing)
        ),
        rel_path,
    ));
}
