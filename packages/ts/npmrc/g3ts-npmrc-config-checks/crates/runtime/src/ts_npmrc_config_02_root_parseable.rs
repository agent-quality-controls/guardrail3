use g3ts_npmrc_types::{G3TsNpmrcChecksInput, G3TsNpmrcRootState};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "TS-NPMRC-CONFIG-02";

pub(crate) fn check(input: &G3TsNpmrcChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.root {
        G3TsNpmrcRootState::NotPackageManagerRoot => {}
        G3TsNpmrcRootState::Missing => {}
        G3TsNpmrcRootState::Unreadable { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root .npmrc unreadable",
                format!("Failed to read root `.npmrc`: {reason}"),
                rel_path,
            ));
        }
        G3TsNpmrcRootState::ParseError { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root .npmrc parse error",
                format!("Failed to parse root `.npmrc`: {reason}"),
                rel_path,
            ));
        }
        G3TsNpmrcRootState::Parsed { snapshot } => {
            results.push(info(
                ID,
                "root .npmrc parses",
                "The root .npmrc parses as valid config.".to_owned(),
                &snapshot.rel_path,
            ));
        }
    }
}
