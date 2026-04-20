use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::full_config::support::{
    error, info, parsed_document, project_service_enabled, selected_rel_path,
};

const ID: &str = "TS-ESLINT-CONFIG-04";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    if project_service_enabled(input) {
        results.push(info(
            ID,
            "projectService enabled for TS source",
            "`projectService: true` is active for the TS source probe.".to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "projectService missing for TS source",
        "The TS source probe does not enable `projectService: true`. Enable typed linting through the modern typescript-eslint project-service flow."
            .to_owned(),
        rel_path,
    ));
}
