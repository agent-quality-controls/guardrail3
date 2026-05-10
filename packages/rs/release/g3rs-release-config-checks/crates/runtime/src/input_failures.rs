use g3rs_release_types::G3RsReleaseInputFailure;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

/// `ID` constant.
const ID: &str = "g3rs-release/config-input-failures";

/// `check` function.
pub(crate) fn check(failure: &G3RsReleaseInputFailure, results: &mut Vec<G3CheckResult>) {
    results.push(error(
        ID,
        "failed to read release config input",
        failure.message.clone(),
        &failure.rel_path,
    ));
}
