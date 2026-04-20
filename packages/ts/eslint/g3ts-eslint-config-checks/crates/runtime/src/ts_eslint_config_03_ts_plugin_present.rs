use g3ts_eslint_types::G3TsEslintConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::full_config::support::{error, has_ts_plugin, info, parsed_document, selected_rel_path};

const ID: &str = "TS-ESLINT-CONFIG-03";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if parsed_document(input).is_none() {
        return;
    }

    let Some(rel_path) = selected_rel_path(input) else {
        return;
    };

    if has_ts_plugin(input) {
        results.push(info(
            ID,
            "@typescript-eslint plugin active on TS source",
            "`@typescript-eslint` is active for the TS source probe.".to_owned(),
            rel_path,
        ));
        return;
    }

    results.push(error(
        ID,
        "@typescript-eslint plugin missing on TS source",
        "The TS source probe does not include the `@typescript-eslint` plugin. Route TS source files through the typed-lint ESLint stack."
            .to_owned(),
        rel_path,
    ));
}
