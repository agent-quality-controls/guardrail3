use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "g3rs-release/publish-must-be-explicit";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if crate::support::crate_publish_declared(krate) {
        return;
    }

    results.push(error(
        ID,
        format!("{}: publish must be explicit", krate.name),
        format!(
            "Crate `{}` does not set `[package].publish`. Add `publish = true` if this crate publishes or `publish = false` if it does not.",
            krate.name
        ),
        &krate.cargo_rel_path,
    ));
}

#[cfg(test)]
#[path = "publish_must_be_explicit_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod publish_must_be_explicit_tests;

#[cfg(test)]
pub(crate) fn run_check(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::test_support::config_input_for_crate(cargo_toml, workspace_cargo_toml);
    let mut results = Vec::new();
    check(&input.crate_checks[0], &mut results);
    results
}
