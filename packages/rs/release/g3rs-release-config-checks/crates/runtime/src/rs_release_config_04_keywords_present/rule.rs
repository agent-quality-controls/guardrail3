use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-04";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    match krate.keywords_count {
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
    let input =
        crate::lib_tests::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::rs_release_config_04_keywords_present::check(&input.crate_checks[0], &mut results);
    results
}

#[cfg(test)]
pub(crate) const GOLDEN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/fixtures/golden_cargo.toml"
));
