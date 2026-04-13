use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-06";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    match krate.version_string.as_ref() {
        Some(version) if krate.version_valid => {
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
#[path = "rule_tests/mod.rs"]
mod tests;
