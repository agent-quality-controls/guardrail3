/// `active_error_rules` helper.
pub(crate) fn active_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| {
        probe
            .rules
            .iter()
            .filter_map(|(rule_name, setting)| {
                (setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
                    .then_some(rule_name.clone())
            })
            .collect()
    })
}

/// `active_warn_or_error_rules` helper.
pub(crate) fn active_warn_or_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe.map_or_else(Vec::new, |probe| {
        probe
            .rules
            .iter()
            .filter_map(|(rule_name, setting)| {
                (setting.severity >= eslint_config_parser::types::EslintRuleSeverity::Warn)
                    .then_some(rule_name.clone())
            })
            .collect()
    })
}

/// `restricted_disable_patterns` helper.
pub(crate) fn restricted_disable_patterns(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(setting) = probe.and_then(|probe| {
        probe
            .rules
            .get("@eslint-community/eslint-comments/no-restricted-disable")
    }) else {
        return Vec::new();
    };

    if setting.severity < eslint_config_parser::types::EslintRuleSeverity::Warn {
        return Vec::new();
    }

    setting
        .options
        .iter()
        .filter_map(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

/// `unused_disable_fail_closed` helper.
pub(crate) fn unused_disable_fail_closed(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> bool {
    let Some(probe) = probe else {
        return false;
    };

    probe
        .rules
        .get("@eslint-community/eslint-comments/no-unused-disable")
        .is_some_and(|setting| {
            setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
        })
        || probe.linter_options_report_unused_disable_directives
            == Some(eslint_config_parser::types::EslintReportUnusedSetting::Error)
}
