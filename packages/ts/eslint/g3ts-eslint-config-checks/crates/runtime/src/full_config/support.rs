use eslint_config_parser::{
    probe, rule_setting,
    types::{EslintEffectiveConfigProbe, EslintProbeKind, EslintRuleSetting, EslintRuleSeverity},
};
use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use serde_json::Value;

#[must_use]
pub(crate) fn selected_rel_path(input: &G3TsEslintConfigChecksInput) -> Option<&str> {
    match &input.config {
        G3TsEslintConfigState::Missing => None,
        G3TsEslintConfigState::Unreadable { rel_path, .. }
        | G3TsEslintConfigState::ParseError { rel_path, .. }
        | G3TsEslintConfigState::Parsed { rel_path, .. } => Some(rel_path),
    }
}

#[must_use]
pub(crate) fn parsed_document(
    input: &G3TsEslintConfigChecksInput,
) -> Option<&eslint_config_parser::types::EslintConfigDocument> {
    match &input.config {
        G3TsEslintConfigState::Parsed { document, .. } => Some(document),
        G3TsEslintConfigState::Missing
        | G3TsEslintConfigState::Unreadable { .. }
        | G3TsEslintConfigState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn has_ts_plugin(input: &G3TsEslintConfigChecksInput) -> bool {
    has_plugin_for(input, EslintProbeKind::TsSource, "@typescript-eslint")
}

#[must_use]
pub(crate) fn project_service_enabled(input: &G3TsEslintConfigChecksInput) -> bool {
    project_service_enabled_for(input, EslintProbeKind::TsSource)
}

#[must_use]
pub(crate) fn rule_is_error(input: &G3TsEslintConfigChecksInput, rule_name: &str) -> bool {
    rule_is_error_for(input, EslintProbeKind::TsSource, rule_name)
}

#[must_use]
pub(crate) fn probe_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
) -> Option<&EslintEffectiveConfigProbe> {
    probe(parsed_document(input)?, probe_kind)
}

#[must_use]
pub(crate) fn has_plugin_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    plugin_name: &str,
) -> bool {
    probe_for(input, probe_kind)
        .is_some_and(|probe| probe.plugins.iter().any(|plugin| plugin == plugin_name))
}

#[must_use]
pub(crate) fn project_service_enabled_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
) -> bool {
    probe_for(input, probe_kind).is_some_and(|probe| probe.project_service == Some(true))
}

#[must_use]
pub(crate) fn project_service_disabled_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
) -> bool {
    probe_for(input, probe_kind).is_some_and(|probe| probe.project_service != Some(true))
}

#[must_use]
pub(crate) fn rule_setting_for<'a>(
    input: &'a G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    rule_name: &str,
) -> Option<&'a EslintRuleSetting> {
    parsed_document(input).and_then(|document| rule_setting(document, probe_kind, rule_name))
}

#[must_use]
pub(crate) fn rule_is_error_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    rule_name: &str,
) -> bool {
    rule_setting_for(input, probe_kind, rule_name)
        .is_some_and(|rule| rule.severity == EslintRuleSeverity::Error)
}

#[must_use]
pub(crate) fn rule_is_off_for(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    rule_name: &str,
) -> bool {
    rule_setting_for(input, probe_kind, rule_name)
        .is_some_and(|rule| rule.severity == EslintRuleSeverity::Off)
}

#[must_use]
pub(crate) fn missing_error_rules(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    rule_names: &[&str],
) -> Vec<String> {
    rule_names
        .iter()
        .filter(|rule_name| !rule_is_error_for(input, probe_kind, rule_name))
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

#[must_use]
pub(crate) fn threshold_matches(
    input: &G3TsEslintConfigChecksInput,
    probe_kind: EslintProbeKind,
    rule_name: &str,
    expected: i64,
    keys: &[&str],
) -> bool {
    let Some(rule) = rule_setting_for(input, probe_kind, rule_name) else {
        return false;
    };

    if rule.severity != EslintRuleSeverity::Error {
        return false;
    }

    rule.options
        .iter()
        .any(|value| option_matches_expected(value, expected, keys))
}

#[must_use]
pub(crate) fn format_rule_list(rule_names: &[String]) -> String {
    rule_names
        .iter()
        .map(|rule_name| format!("`{rule_name}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

#[must_use]
pub(crate) fn error(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
}

fn option_matches_expected(value: &Value, expected: i64, keys: &[&str]) -> bool {
    if value.as_i64().is_some_and(|number| number == expected) {
        return true;
    }

    if let Some(object) = value.as_object() {
        return keys.iter().any(|key| {
            object
                .get(*key)
                .and_then(Value::as_i64)
                .is_some_and(|number| number == expected)
        });
    }

    false
}
