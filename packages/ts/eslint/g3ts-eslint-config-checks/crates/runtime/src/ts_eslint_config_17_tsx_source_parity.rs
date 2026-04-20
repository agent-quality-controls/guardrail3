use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::support::{
    error, format_plugin_list, format_rule_list, has_tsx_source_probe, info, missing_error_rules,
    missing_error_rules_for_tsx, missing_plugins_for_tsx, parsed_document,
    project_service_enabled_for, selected_rel_path, threshold_matches,
};

const ID: &str = "TS-ESLINT-CONFIG-17";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() || !has_tsx_source_probe(input) {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let wrong_thresholds = super::baseline::THRESHOLD_RULES
        .iter()
        .filter(|(rule_name, expected, keys)| {
            !threshold_matches(
                input,
                EslintProbeKind::TsxSource,
                rule_name,
                *expected,
                keys,
            )
        })
        .map(|(rule_name, expected, _)| format!("{rule_name}={expected}"))
        .collect::<Vec<_>>();
    let missing_presence_rules = missing_error_rules(
        input,
        EslintProbeKind::TsxSource,
        super::baseline::REQUIRED_THRESHOLD_PRESENCE_RULES,
    );
    let missing_plugins =
        missing_plugins_for_tsx(input, super::baseline::REQUIRED_TS_SOURCE_PLUGINS);
    let missing_rules = [
        missing_error_rules_for_tsx(input, super::baseline::CORE_BASELINE_RULES),
        missing_error_rules_for_tsx(input, super::baseline::TYPE_SAFETY_RULES),
        missing_error_rules_for_tsx(input, super::baseline::HYGIENE_RULES),
        missing_error_rules_for_tsx(input, super::baseline::UNICORN_RULES),
        missing_error_rules_for_tsx(input, super::baseline::REGEXP_RULES),
        missing_error_rules_for_tsx(input, super::baseline::SONARJS_RULES),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if project_service_enabled_for(input, EslintProbeKind::TsxSource)
        && wrong_thresholds.is_empty()
        && missing_presence_rules.is_empty()
        && missing_plugins.is_empty()
        && missing_rules.is_empty()
    {
        results.push(info(
            ID,
            "TSX source baseline matches TS source baseline",
            "The TSX source probe keeps the same typed-lint, threshold, plugin, and grouped-rule baseline as the TS source probe."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    let mut parts = Vec::new();
    if !project_service_enabled_for(input, EslintProbeKind::TsxSource) {
        parts.push("The TSX source probe must enable `projectService: true`.".to_owned());
    }
    if !wrong_thresholds.is_empty() {
        parts.push(format!(
            "Wrong or missing threshold settings on the TSX source probe: {}.",
            wrong_thresholds
                .iter()
                .map(|item| format!("`{item}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !missing_presence_rules.is_empty() {
        parts.push(format!(
            "Missing required error rules on the TSX source probe: {}.",
            format_rule_list(&missing_presence_rules)
        ));
    }
    if !missing_plugins.is_empty() {
        parts.push(format!(
            "The TSX source probe must activate these plugins: {}.",
            format_plugin_list(&missing_plugins)
        ));
    }
    if !missing_rules.is_empty() {
        parts.push(format!(
            "The TSX source probe must enforce these grouped baseline rules at error severity: {}.",
            format_rule_list(&missing_rules)
        ));
    }

    results.push(error(
        ID,
        "TSX source baseline drift detected",
        parts.join(" "),
        rel_path,
    ));
}
