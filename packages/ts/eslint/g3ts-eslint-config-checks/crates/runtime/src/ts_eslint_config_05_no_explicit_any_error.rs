use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info, parsed_document, rule_is_error, selected_rel_path};

const ID: &str = "TS-ESLINT-CONFIG-05";
const RULE_NAME: &str = "@typescript-eslint/no-explicit-any";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    if rule_is_error(input, RULE_NAME) {
        results.push(info(
            ID,
            "no-explicit-any enforced on TS source",
            format!("`{RULE_NAME}` is set to error on the TS source probe."),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "no-explicit-any not enforced on TS source",
        format!("`{RULE_NAME}` must be set to error on the TS source probe."),
        rel_path,
    ));
}
