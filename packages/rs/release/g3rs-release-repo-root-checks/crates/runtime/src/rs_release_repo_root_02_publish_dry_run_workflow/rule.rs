use g3rs_release_repo_root_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-REPO-ROOT-02";

pub(crate) fn check(input: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if input.has_publish_dry_run_workflow {
        results.push(info(
            ID,
            "Publish dry-run workflow present",
            String::new(),
            input.publish_dry_run_workflow_rel_path
                .as_deref()
                .unwrap_or(input.cargo_rel_path.as_str()),
        ));
    } else {
        results.push(warn(
            ID,
            "Publish dry-run workflow missing",
            "No workflow contains an actual `cargo publish --dry-run` step. Add a `cargo publish --dry-run` step to a CI workflow.".to_owned(),
            &input.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
