use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{crate_name, error, info, is_publishable};

/// Check ID for repository presence.
const ID: &str = "RS-RELEASE-CONFIG-03";

/// Verify that a publishable crate has a `repository` field in `[package]`.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !is_publishable(&input.cargo) {
        return;
    }

    let name = crate_name(&input.cargo, &input.cargo_rel_path);
    let file = &input.cargo_rel_path;

    let has_repository = input
        .cargo
        .package
        .as_ref()
        .and_then(|p| p.repository.as_ref())
        .is_some();

    if has_repository {
        results.push(info(ID, format!("{name}: repository present"), String::new(), file));
    } else {
        results.push(error(
            ID,
            format!("{name}: missing repository"),
            "Publishable crates must have a repository field in [package].".to_owned(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
