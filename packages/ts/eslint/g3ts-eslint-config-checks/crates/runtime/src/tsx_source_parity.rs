use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::support::{
    error, format_plugin_list, format_rule_list, has_tsx_source_probe, info, missing_error_rules,
    missing_error_rules_for_tsx, missing_plugins_for_tsx, parsed_document,
    project_service_enabled_for, selected_rel_path, threshold_matches,
};

/// Internal constant `ID`.
const ID: &str = "g3ts-eslint/tsx-source-parity";

/// Collected drift findings for the TSX source probe baseline parity check.
struct TsxBaselineDrift {
    /// Threshold rules whose value or option keys disagree with the baseline.
    wrong_thresholds: Vec<String>,
    /// Required presence rules that are missing at error severity.
    missing_presence_rules: Vec<String>,
    /// Plugins required for TSX-source parity that are not active on the probe.
    missing_plugins: Vec<String>,
    /// Grouped baseline rules that are missing at error severity.
    missing_rules: Vec<String>,
}

/// Internal function `check`.
pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() || !has_tsx_source_probe(input) {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let drift = collect_drift(input);
    let project_service = project_service_enabled_for(input, EslintProbeKind::TsxSource);

    if project_service && drift.is_clean() {
        results.push(info(
            ID,
            "TSX source baseline matches TS source baseline",
            "The TSX source probe keeps the same typed-lint, threshold, plugin, and grouped-rule baseline as the TS source probe."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    let message = render_drift_message(&drift, project_service);
    results.push(error(
        ID,
        "TSX source baseline drift detected",
        message,
        rel_path,
    ));
}

/// Collects all baseline drift findings for the TSX source probe.
fn collect_drift(input: &G3TsEslintConfigChecksInput) -> TsxBaselineDrift {
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

    TsxBaselineDrift {
        wrong_thresholds,
        missing_presence_rules,
        missing_plugins,
        missing_rules,
    }
}

impl TsxBaselineDrift {
    /// Returns `true` when no drift findings are recorded.
    fn is_clean(&self) -> bool {
        self.wrong_thresholds.is_empty()
            && self.missing_presence_rules.is_empty()
            && self.missing_plugins.is_empty()
            && self.missing_rules.is_empty()
    }
}

/// Renders the human-readable error message from a `TsxBaselineDrift`.
fn render_drift_message(drift: &TsxBaselineDrift, project_service: bool) -> String {
    let mut parts = Vec::new();
    if !project_service {
        parts.push("The TSX source probe must enable `projectService: true`.".to_owned());
    }
    if !drift.wrong_thresholds.is_empty() {
        parts.push(format!(
            "Wrong or missing threshold settings on the TSX source probe: {}.",
            drift
                .wrong_thresholds
                .iter()
                .map(|item| format!("`{item}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !drift.missing_presence_rules.is_empty() {
        parts.push(format!(
            "Missing required error rules on the TSX source probe: {}.",
            format_rule_list(&drift.missing_presence_rules)
        ));
    }
    if !drift.missing_plugins.is_empty() {
        parts.push(format!(
            "The TSX source probe must activate these plugins: {}.",
            format_plugin_list(&drift.missing_plugins)
        ));
    }
    if !drift.missing_rules.is_empty() {
        parts.push(format!(
            "The TSX source probe must enforce these grouped baseline rules at error severity: {}.",
            format_rule_list(&drift.missing_rules)
        ));
    }
    parts.join(" ")
}
