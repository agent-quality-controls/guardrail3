use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

const ID: &str = "RS-RELEASE-CONFIG-22";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    if krate.include_exclude_present {
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
