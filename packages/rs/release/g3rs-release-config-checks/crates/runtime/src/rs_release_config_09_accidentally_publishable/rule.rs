use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, is_publishable};

/// Check ID for accidentally publishable crates.
const ID: &str = "RS-RELEASE-CONFIG-09";

/// Flag crates that are publishable but appear to lack intentional release metadata.
///
/// A crate missing ALL THREE of description, license, and repository is likely
/// accidentally publishable. Having even one of the three signals intent.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let pkg = input.cargo.package.as_ref();
    let has_description = pkg.and_then(|p| p.description.as_ref()).is_some();
    let has_license = pkg.and_then(|p| p.license.as_ref()).is_some()
        || pkg.and_then(|p| p.license_file.as_ref()).is_some();
    let has_repository = pkg.and_then(|p| p.repository.as_ref()).is_some();

    if !has_description && !has_license && !has_repository {
        let name = crate_name(&input.cargo, &input.cargo_rel_path);
        let file = &input.cargo_rel_path;
        results.push(error(
            ID,
            format!("{name} may be accidentally publishable"),
            "Crate is publishable but has no description, license, or repository. \
             If this crate is not intended for publication, add `publish = false` to [package]."
                .to_owned(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
