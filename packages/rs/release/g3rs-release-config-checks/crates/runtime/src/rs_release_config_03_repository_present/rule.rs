use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-03";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    if krate.repository_present {
        results.push(info(
            ID,
            format!("{}: repository present", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(error(
            ID,
            format!("{}: missing repository", krate.name),
            "Publishable crates must have a repository field in [package].".to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
