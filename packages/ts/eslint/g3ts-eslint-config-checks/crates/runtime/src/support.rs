use eslint_config_parser::{probe, rule_setting, types::EslintProbeKind};
use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

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
pub(crate) fn ts_probe(
    input: &G3TsEslintConfigChecksInput,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe(parsed_document(input)?, EslintProbeKind::TsSource)
}

#[must_use]
pub(crate) fn has_ts_plugin(input: &G3TsEslintConfigChecksInput) -> bool {
    ts_probe(input).is_some_and(|probe| {
        probe
            .plugins
            .iter()
            .any(|plugin| plugin == "@typescript-eslint")
    })
}

#[must_use]
pub(crate) fn project_service_enabled(input: &G3TsEslintConfigChecksInput) -> bool {
    ts_probe(input).is_some_and(|probe| probe.project_service == Some(true))
}

#[must_use]
pub(crate) fn rule_is_error(input: &G3TsEslintConfigChecksInput, rule_name: &str) -> bool {
    parsed_document(input)
        .and_then(|document| rule_setting(document, EslintProbeKind::TsSource, rule_name))
        .is_some_and(|rule| rule.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
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
