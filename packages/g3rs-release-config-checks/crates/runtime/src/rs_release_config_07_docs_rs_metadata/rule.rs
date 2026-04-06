use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, info, is_library, is_publishable, warn};

/// Check ID for docs.rs metadata.
const ID: &str = "RS-RELEASE-CONFIG-07";

/// Verify that a publishable library has `[package.metadata.docs.rs]`.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    if !is_library(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let has_docs_rs = input
        .cargo
        .package
        .as_ref()
        .and_then(|p| p.metadata.as_ref())
        .and_then(|m| m.get("docs"))
        .and_then(|d| d.get("rs"))
        .is_some();

    if has_docs_rs {
        results.push(info(
            ID,
            format!("{name}: docs.rs metadata present"),
            String::new(),
            file,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{name}: docs.rs metadata missing"),
            "Library crates should have [package.metadata.docs.rs] for docs.rs configuration."
                .to_owned(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
