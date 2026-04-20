use g3ts_tsconfig_types::{G3TsTsconfigChecksInput, G3TsTsconfigState};
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::info;

const ID: &str = "TS-TSCONFIG-CONFIG-02";

pub(crate) fn check(input: &G3TsTsconfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.config {
        G3TsTsconfigState::Missing => {}
        G3TsTsconfigState::Unreadable { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "tsconfig unreadable".to_owned(),
                format!("Failed to read `{rel_path}` while evaluating tsconfig policy: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsTsconfigState::ParseError { rel_path, reason } => {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "tsconfig parse error".to_owned(),
                format!("Failed to parse `{rel_path}` as tsconfig JSONC: {reason}"),
                Some(rel_path.clone()),
                None,
            ));
        }
        G3TsTsconfigState::Parsed { rel_path, .. } => {
            results.push(info(
                ID,
                "tsconfig parseable",
                format!("`{rel_path}` parsed successfully as tsconfig JSONC."),
                rel_path,
            ));
        }
    }
}
