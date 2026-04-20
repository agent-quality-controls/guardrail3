use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::{
    baseline::{REQUIRED_THRESHOLD_PRESENCE_RULES, THRESHOLD_RULES},
    support::{
        error, format_rule_list, info, missing_error_rules, parsed_document, selected_rel_path,
        threshold_matches,
    },
};

const ID: &str = "TS-ESLINT-CONFIG-07";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let wrong_thresholds = THRESHOLD_RULES
        .iter()
        .filter(|(rule_name, expected, keys)| {
            !threshold_matches(input, EslintProbeKind::TsSource, rule_name, *expected, keys)
        })
        .map(|(rule_name, expected, _)| format!("{rule_name}={expected}"))
        .collect::<Vec<_>>();

    let missing_presence_rules = missing_error_rules(
        input,
        EslintProbeKind::TsSource,
        REQUIRED_THRESHOLD_PRESENCE_RULES,
    );

    if wrong_thresholds.is_empty() && missing_presence_rules.is_empty() {
        results.push(info(
            ID,
            "baseline thresholds and restricted imports enforced",
            "The TS source probe has the expected threshold rules and `no-restricted-imports`."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    let mut parts = Vec::new();
    if !wrong_thresholds.is_empty() {
        parts.push(format!(
            "Wrong or missing threshold settings: {}.",
            wrong_thresholds
                .iter()
                .map(|item| format!("`{item}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !missing_presence_rules.is_empty() {
        parts.push(format!(
            "Missing required error rules: {}.",
            format_rule_list(&missing_presence_rules)
        ));
    }

    results.push(error(
        ID,
        "baseline thresholds or restricted imports not enforced",
        parts.join(" "),
        rel_path,
    ));
}
