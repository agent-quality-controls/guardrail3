use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/keywords-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    match crate::support::crate_keywords_count(krate) {
        Some(count) if (1..=5).contains(&count) => {
            results.push(info(
                ID,
                format!("{}: keywords present", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        Some(count) => {
            results.push(error(
                ID,
                format!("{}: keywords count invalid ({count})", krate.name),
                "Publishable crates must have between 1 and 5 keywords.".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: keywords missing", krate.name),
                "Publishable crates must have keywords in [package].".to_owned(),
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
    let input = crate::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::keywords_present::check(
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
