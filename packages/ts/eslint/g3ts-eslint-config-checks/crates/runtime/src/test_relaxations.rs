use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::support::{error, info, parsed_document, rule_is_off_for, selected_rel_path};

const ID: &str = "g3ts-eslint/test-relaxations";
const RULE_NAME: &str = "@typescript-eslint/no-explicit-any";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    if rule_is_off_for(input, EslintProbeKind::TsTest, RULE_NAME) {
        results.push(info(
            ID,
            "test carve-out for no-explicit-any present",
            format!("`{RULE_NAME}` is off for the TS test probe."),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "test carve-out for no-explicit-any missing",
        format!("`{RULE_NAME}` must be off for the TS test probe."),
        rel_path,
    ));
}
