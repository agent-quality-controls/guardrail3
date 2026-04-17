use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-05";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    match krate.categories_count {
        Some(0) => {
            results.push(error(
                ID,
                format!("{}: categories missing", krate.name),
                "Publishable crates must have at least one category.".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
        Some(_) => {
            results.push(info(
                ID,
                format!("{}: categories present", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: categories missing", krate.name),
                "Publishable crates must have categories in [package].".to_owned(),
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
    let input = crate::lib_tests::test_support::config_input_for_publishable_crate(cargo_toml, None);
    let mut results = Vec::new();
    crate::rs_release_config_05_categories_present::check(&input.crates[0], &mut results);
    results
}

#[cfg(test)]
pub(crate) const GOLDEN: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/fixtures/golden_cargo.toml"));
