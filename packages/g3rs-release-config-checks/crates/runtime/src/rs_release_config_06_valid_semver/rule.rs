use cargo_toml_parser::InheritableValue;
use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for valid semver.
const ID: &str = "RS-RELEASE-CONFIG-06";

/// Verify that a publishable crate has a valid semver version.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let version = input.cargo.package.as_ref().and_then(|p| p.version.as_ref());

    match version {
        Some(InheritableValue::Inherit(_)) => {
            // Workspace manages version — treat as valid.
            results.push(info(ID, format!("{name}: valid semver"), String::new(), file));
        }
        Some(InheritableValue::Value(v)) => {
            if semver::Version::parse(v).is_ok() {
                results.push(info(ID, format!("{name}: valid semver"), String::new(), file));
            } else {
                results.push(error(
                    ID,
                    format!("{name}: invalid version"),
                    format!("Version \"{v}\" is not valid semver (expected major.minor.patch)."),
                    file,
                ));
            }
        }
        None => {
            results.push(error(
                ID,
                format!("{name}: invalid version"),
                "Publishable crates must have a version field in [package].".to_owned(),
                file,
            ));
        }
    }
}

/// Check that a version string has at least `major.minor` components.

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
