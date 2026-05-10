use g3rs_release_types::G3RsReleaseConfigRepo;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/registry-token";

/// `check` function.
pub(crate) fn check(input: &G3RsReleaseConfigRepo, results: &mut Vec<G3CheckResult>) {
    if input.workflow_flags.has_registry_token_workflow {
        results.push(info(
            ID,
            "CARGO_REGISTRY_TOKEN wired in workflow",
            String::new(),
            input
                .registry_token_workflow_rel_path
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
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
