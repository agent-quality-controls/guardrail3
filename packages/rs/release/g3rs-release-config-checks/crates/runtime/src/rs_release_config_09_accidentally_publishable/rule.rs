use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-09";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    if krate.description_present || krate.license_present || krate.repository_present {
        return;
    }

    results.push(error(
        ID,
        format!("{} may be accidentally publishable", krate.name),
        "Crate is publishable but has no description, license, or repository. \
         If this crate is not intended for publication, add `publish = false` to [package]."
            .to_owned(),
        &krate.cargo_rel_path,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

#[cfg(test)]
pub(crate) fn run_check(cargo_toml: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input =
        crate::lib_tests::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::rs_release_config_09_accidentally_publishable::check(
        &input.crate_checks[0],
        &mut results,
    );
    results
}

#[cfg(test)]
pub(crate) fn run_check_with_workspace(
    cargo_toml: &str,
    workspace_cargo_toml: &str,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::lib_tests::test_support::config_input_for_crate(
        cargo_toml,
        Some(workspace_cargo_toml),
    );
    crate::check(&input)
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_cargo.toml"
));
