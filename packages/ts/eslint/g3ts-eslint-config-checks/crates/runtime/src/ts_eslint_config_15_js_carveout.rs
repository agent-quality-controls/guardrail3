use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::support::{
    error, info, parsed_document, project_service_disabled_for, selected_rel_path,
};

const ID: &str = "TS-ESLINT-CONFIG-15";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    if project_service_disabled_for(input, EslintProbeKind::JsSource) {
        results.push(info(
            ID,
            "JS carve-out disables projectService",
            "The JS source probe does not enable `projectService: true`.".to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "JS carve-out for projectService missing",
        "The JS source probe must not enable `projectService: true`.".to_owned(),
        rel_path,
    ));
}
