use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/include-exclude-inventory";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    if crate::support::crate_include_exclude_present(krate) {
        results.push(info(
            ID,
            format!("{}: include/exclude configured", krate.name),
            String::new(),
            &krate.cargo_rel_path,
        ));
    } else {
        results.push(warn(
            ID,
            format!("{}: include/exclude missing", krate.name),
            "Publishable crates should set `include` or `exclude` patterns to control what gets published. Add `include = [\"src/**\", \"Cargo.toml\", \"README.md\", \"LICENSE\"]` to `[package]`.".to_owned(),
            &krate.cargo_rel_path,
        ));
    }
}
