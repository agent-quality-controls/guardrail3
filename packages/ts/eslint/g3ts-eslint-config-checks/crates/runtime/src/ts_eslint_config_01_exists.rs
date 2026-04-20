use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::full_config::support::{info, selected_rel_path};

const ID: &str = "TS-ESLINT-CONFIG-01";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(rel_path) = selected_rel_path(input) else {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            guardrail3_check_types::G3Severity::Error,
            "eslint config missing".to_owned(),
            "No root `eslint.config.*` file was found. Add a root flat ESLint config.".to_owned(),
            None,
            None,
        ));
        return;
    };

    results.push(info(
        ID,
        "eslint config exists",
        format!("Found root ESLint config `{rel_path}`."),
        rel_path,
    ));
}
