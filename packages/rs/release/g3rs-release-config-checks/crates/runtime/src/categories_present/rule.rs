use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/categories-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    match crate::support::crate_categories_count(krate) {
        Some(0) => {
            results.push(error(
                ID,
                format!("{}: categories missing", krate.name),
                "Publishable crates must have at least one category.".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
        Some(_) => {
            results.push(info(
                ID,
                format!("{}: categories present", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: categories missing", krate.name),
                "Publishable crates must have categories in [package].".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
    }
}
