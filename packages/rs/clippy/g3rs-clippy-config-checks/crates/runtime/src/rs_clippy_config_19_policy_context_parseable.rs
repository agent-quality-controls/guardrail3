use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{policy_context_failure, policy_context_rel_path};

const ID: &str = "RS-CLIPPY-CONFIG-19";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match policy_context_failure(input) {
        Some((rel_path, reason)) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "clippy policy context is not parseable".to_owned(),
            format!(
                "Failed to parse active `guardrail3.toml` used for clippy profile and garde policy: {reason}"
            ),
            Some(rel_path.to_owned()),
            None,
        )),
        None => {
            if let Some(rel_path) = policy_context_rel_path(input) {
                results.push(
                    G3CheckResult::new(
                        ID.to_owned(),
                        G3Severity::Info,
                        "clippy policy context parseable".to_owned(),
                        "Active `guardrail3.toml` parsed successfully for clippy policy context."
                            .to_owned(),
                        Some(rel_path.to_owned()),
                        None,
                    )
                    .into_inventory(),
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "rs_clippy_config_19_policy_context_parseable_tests/mod.rs"]
mod tests;
