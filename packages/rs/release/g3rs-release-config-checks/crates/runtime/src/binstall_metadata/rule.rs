use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/binstall-metadata";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) || !krate.is_binary {
        return;
    }

    if crate::support::crate_has_binstall_metadata(krate) {
        results.push(info(
            ID,
            format!("{}: binstall metadata present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{}: missing binstall metadata", krate.name),
            "Binary crates should have [package.metadata.binstall] for cargo-binstall support."
                .to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

#[cfg(test)]
pub(crate) fn run_check(cargo_toml: &str) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = crate::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::binstall_metadata::check(
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
