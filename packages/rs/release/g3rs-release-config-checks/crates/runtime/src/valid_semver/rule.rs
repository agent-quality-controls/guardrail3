use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "g3rs-release/valid-semver";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    match crate::support::crate_version_string(krate) {
        Some(version) if crate::support::crate_version_valid(krate) => {
            results.push(info(
                ID,
                format!("{}: valid semver", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        Some(version) => {
            results.push(error(
                ID,
                format!("{}: invalid version", krate.name),
                format!("Version \"{version}\" is not valid semver (expected major.minor.patch)."),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: invalid version", krate.name),
                "Publishable crates must have a version field in [package].".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

#[cfg(test)]
pub(crate) fn run_check(cargo_toml: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    run_check_with_workspace(cargo_toml, None)
}

#[cfg(test)]
pub(crate) fn run_check_with_workspace(
    cargo_toml: &str,
    workspace_cargo_toml: Option<&str>,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::lib_tests::test_support::config_input_for_publishable_crate(
        cargo_toml,
        workspace_cargo_toml,
    );
    let mut results = Vec::new();
    crate::valid_semver::check(&input.crate_checks[0], &mut results);
    results
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_cargo.toml"
));
