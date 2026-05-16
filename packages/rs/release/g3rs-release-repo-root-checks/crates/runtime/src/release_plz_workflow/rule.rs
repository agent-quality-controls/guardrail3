use g3rs_release_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/release-plz-workflow";

/// `check` function.
pub(crate) fn check(input: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if input.workflow_flags.has_release_plz_workflow {
        results.push(info(
            ID,
            "Release-plz workflow present",
            String::new(),
            input
                .release_plz_workflow_rel_path
                .as_deref()
                .unwrap_or(input.release_plz_rel_path.as_str()),
        ));
    } else {
        results.push(warn(
            ID,
            "Release-plz workflow missing",
            "No workflow contains an actual release-plz execution step. Add a release-plz step to a GitHub Actions workflow.".to_owned(),
            &input.cargo_rel_path,
        ));
    }
}
