use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/description-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_description_present(krate) {
        results.push(info(
            ID,
            format!("{}: description present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            format!("{}: missing description", krate.name),
            "Publishable crates must have a description field in [package].".to_owned(),
            &krate.cargo_rel_path,
        ));
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
    let input =
        crate::test_support::config_input_for_publishable_crate(cargo_toml, workspace_cargo_toml);
    let mut results = Vec::new();
    crate::description_present::check(
        input
            .crates
            .first()
            .expect("test fixture must include a crate"),
        &mut results,
    );
    results
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_cargo.toml"
));
