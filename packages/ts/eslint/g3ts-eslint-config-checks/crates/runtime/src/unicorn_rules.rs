use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::{
    baseline::UNICORN_RULES,
    support::{
        error, format_rule_list, info, missing_error_rules, parsed_document, selected_rel_path,
    },
};

/// Internal constant `ID`.
const ID: &str = "g3ts-eslint/unicorn-rules";

/// Internal function `check`.
pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let missing = missing_error_rules(input, EslintProbeKind::TsSource, UNICORN_RULES);
    if missing.is_empty() {
        results.push(info(
            ID,
            "unicorn rules enforced",
            "The TS source probe has the required unicorn baseline rules at error severity."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "unicorn rules missing or relaxed",
        format!(
            "The TS source probe must enforce these unicorn rules at error severity: {}.",
            format_rule_list(&missing)
        ),
        rel_path,
    ));
}
