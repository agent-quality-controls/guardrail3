use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "TS-ESLINT-CONFIG-02";

pub(crate) fn check(input: &G3TsEslintConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.config {
        G3TsEslintConfigState::Missing => {}
        G3TsEslintConfigState::Unreadable { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                guardrail3_check_types::G3Severity::Error,
                "eslint config unreadable".to_owned(),
                format!("Failed to read `{rel_path}` while evaluating ESLint config: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsEslintConfigState::ParseError { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                guardrail3_check_types::G3Severity::Error,
                "eslint config parse error".to_owned(),
                format!("Failed to parse `{rel_path}` through ESLint: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsEslintConfigState::Parsed { snapshot } => {
            results.push(info(
                ID,
                "eslint config parseable",
                format!(
                    "`{}` parsed successfully through ESLint.",
                    snapshot.selected_config.rel_path
                ),
                &snapshot.selected_config.rel_path,
            ));
        }
    }
}
