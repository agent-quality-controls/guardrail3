use g3rs_clippy_config_checks_types::G3RsClippyConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::{rust_policy_failure, rust_policy_rel_path};

const ID: &str = "RS-CLIPPY-CONFIG-19";

pub(crate) fn check(input: &G3RsClippyConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    match rust_policy_failure(input) {
        Some((rel_path, reason)) => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "clippy rust policy is not parseable".to_owned(),
            format!(
                "Failed to parse active `guardrail3-rs.toml` used for clippy profile and garde policy: {reason}"
            ),
            Some(rel_path.to_owned()),
            None,
        )),
        None => {
            if let Some(rel_path) = rust_policy_rel_path(input) {
                results.push(
                    G3CheckResult::new(
                        ID.to_owned(),
                        G3Severity::Info,
                        "clippy rust policy parseable".to_owned(),
                        "Active `guardrail3-rs.toml` parsed successfully for clippy policy context."
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
