use g3ts_package_types::{G3TsPackageChecksInput, G3TsPackageRootState};
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

/// `ID` constant.
const ID: &str = "g3ts-package/root-parseable";

/// `check`: check.
pub(crate) fn check(input: &G3TsPackageChecksInput, results: &mut Vec<G3CheckResult>) {
    match &input.root {
        G3TsPackageRootState::NotPackageManagerRoot | G3TsPackageRootState::Missing => {}
        G3TsPackageRootState::Unreadable { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root package.json unreadable",
                format!("Failed to read root `package.json`: {reason}"),
                rel_path,
            ));
        }
        G3TsPackageRootState::ParseError { rel_path, reason } => {
            results.push(crate::support::error(
                ID,
                "root package.json parse error",
                format!("Failed to parse root `package.json`: {reason}"),
                rel_path,
            ));
        }
        G3TsPackageRootState::Parsed { snapshot } => {
            results.push(info(
                ID,
                "root package.json parses",
                "The root package manifest parses as valid package.json.".to_owned(),
                &snapshot.rel_path,
            ));
        }
    }
}
