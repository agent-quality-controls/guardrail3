use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-01";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    if krate.description_present {
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
#[path = "rule_tests/mod.rs"]
mod tests;
