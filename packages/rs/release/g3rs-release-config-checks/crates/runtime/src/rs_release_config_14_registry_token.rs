use g3rs_release_config_checks_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-CONFIG-14";

pub(crate) fn check(repo: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if repo.has_registry_token_workflow {
        results.push(info(
            ID,
            "CARGO_REGISTRY_TOKEN wired in workflow",
            String::new(),
            repo.registry_token_workflow_rel_path
                .as_deref()
                .unwrap_or(repo.cargo_rel_path.as_str()),
        ));
    } else {
        results.push(warn(
            ID,
            "CARGO_REGISTRY_TOKEN missing from workflows",
            "No workflow structurally wires `CARGO_REGISTRY_TOKEN` into release steps. Add `CARGO_REGISTRY_TOKEN` as a secret in the release workflow.".to_owned(),
            &repo.cargo_rel_path,
        ));
    }
}
