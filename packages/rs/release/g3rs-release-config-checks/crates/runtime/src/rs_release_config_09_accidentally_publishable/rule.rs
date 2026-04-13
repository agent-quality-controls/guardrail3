use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
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
#[path = "rule_tests/mod.rs"]
mod tests;
