use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for description presence.
const ID: &str = "RS-RELEASE-CONFIG-01";

/// Verify that a publishable crate has a `description` field in `[package]`.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let has_description = input
        .cargo
        .package
        .as_ref()
        .and_then(|p| p.description.as_ref())
        .is_some();

    if has_description {
        results.push(info(ID, format!("{name}: description present"), String::new(), file));
    } else {
        results.push(error(
            ID,
            format!("{name}: missing description"),
            "Publishable crates must have a description field in [package].".to_owned(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
