use g3rs_release_repo_root_checks_types::G3RsReleaseRepoRootChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-REPO-ROOT-03";

pub(crate) fn check(input: &G3RsReleaseRepoRootChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.has_registry_token_workflow {
        results.push(info(
            ID,
            "CARGO_REGISTRY_TOKEN wired in workflow",
            String::new(),
            input.registry_token_workflow_rel_path
                .as_deref()
                .unwrap_or(input.cargo_rel_path.as_str()),
        ));
    } else {
        results.push(warn(
            ID,
            "CARGO_REGISTRY_TOKEN missing from workflows",
            "No workflow structurally wires `CARGO_REGISTRY_TOKEN` into release steps. Add `CARGO_REGISTRY_TOKEN` as a secret in the release workflow.".to_owned(),
            &input.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
