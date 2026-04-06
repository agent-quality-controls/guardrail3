use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, info, is_binary, is_publishable, warn};

/// Check ID for binstall metadata.
const ID: &str = "RS-RELEASE-CONFIG-08";

/// Verify that a publishable binary crate has `[package.metadata.binstall]`.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    if !is_binary(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let has_binstall = input
        .cargo
        .package
        .as_ref()
        .and_then(|p| p.metadata.as_ref())
        .and_then(|m| m.get("binstall"))
        .is_some();

    if has_binstall {
        results.push(info(
            ID,
            format!("{name}: binstall metadata present"),
            String::new(),
            file,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{name}: missing binstall metadata"),
            "Binary crates should have [package.metadata.binstall] for cargo-binstall support."
                .to_owned(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
