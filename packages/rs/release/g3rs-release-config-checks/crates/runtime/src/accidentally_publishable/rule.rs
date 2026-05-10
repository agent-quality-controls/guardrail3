use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

/// `ID` constant.
const ID: &str = "g3rs-release/accidentally-publishable";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_description_present(krate)
        || crate::support::crate_license_present(krate)
        || crate::support::crate_repository_present(krate)
    {
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
    let input = crate::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::accidentally_publishable::check(
        input
            .crates
            .first()
            .expect("test fixture must include a crate"),
        &mut results,
    );
    results
}

#[cfg(test)]
pub(crate) fn run_check_with_workspace(
    cargo_toml: &str,
    workspace_cargo_toml: &str,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::test_support::config_input_for_crate(cargo_toml, Some(workspace_cargo_toml));
    crate::check(&input)
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_cargo.toml"
));
