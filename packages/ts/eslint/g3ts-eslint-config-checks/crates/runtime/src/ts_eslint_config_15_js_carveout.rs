use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::support::{
    error, info, parsed_document, present_error_rules, project_service_disabled_for,
    selected_rel_path,
};
use crate::baseline::JS_CARVEOUT_TYPED_RULES;

const ID: &str = "TS-ESLINT-CONFIG-15";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let leaked_typed_rules =
        present_error_rules(input, EslintProbeKind::JsSource, JS_CARVEOUT_TYPED_RULES);

    if project_service_disabled_for(input, EslintProbeKind::JsSource)
        && leaked_typed_rules.is_empty()
    {
        results.push(info(
            ID,
            "JS carve-out disables typed linting",
            "The JS source probe does not enable `projectService: true` and does not enforce the representative typed-lint rules."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    let mut parts = Vec::new();
    if !project_service_disabled_for(input, EslintProbeKind::JsSource) {
        parts.push("The JS source probe must not enable `projectService: true`.".to_owned());
    }
    if !leaked_typed_rules.is_empty() {
        parts.push(format!(
            "The JS source probe must not enforce these representative typed-lint rules at error severity: {}.",
            super::support::format_rule_list(&leaked_typed_rules)
        ));
    }

    results.push(error(
        ID,
        "JS carve-out for typed linting missing",
        parts.join(" "),
        rel_path,
    ));
}
