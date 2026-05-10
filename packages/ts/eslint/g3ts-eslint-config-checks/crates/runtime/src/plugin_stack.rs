use eslint_config_parser::types::EslintProbeKind;
use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use super::{
    baseline::REQUIRED_TS_SOURCE_PLUGINS,
    support::{
        error, format_plugin_list, info, missing_plugins_for, parsed_document, selected_rel_path,
    },
};

/// Internal constant `ID`.
const ID: &str = "g3ts-eslint/plugin-stack";

/// Internal function `check`.
pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    let missing = missing_plugins_for(input, EslintProbeKind::TsSource, REQUIRED_TS_SOURCE_PLUGINS);
    if missing.is_empty() {
        results.push(info(
            ID,
            "TS plugin stack active on TS source",
            "The TS source probe has the required unicorn, regexp, and sonarjs plugin stack."
                .to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "TS plugin stack missing on TS source",
        format!(
            "The TS source probe must activate these plugins: {}.",
            format_plugin_list(&missing)
        ),
        rel_path,
    ));
}
