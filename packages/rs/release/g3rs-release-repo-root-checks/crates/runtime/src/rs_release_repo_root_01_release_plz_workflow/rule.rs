use g3rs_release_repo_root_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-REPO-ROOT-01";

pub(crate) fn check(input: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if input.has_release_plz_workflow {
        results.push(info(
            ID,
            "Release-plz workflow present",
            String::new(),
            input.release_plz_workflow_rel_path
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
