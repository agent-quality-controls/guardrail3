use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

/// `ID` constant.
const ID: &str = "g3rs-release/keywords-present";

/// `check` function.
pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !crate::support::crate_publishable(krate) {
        return;
    }

    match crate::support::crate_keywords_count(krate) {
        Some(count) if (1..=5).contains(&count) => {
            results.push(info(
                ID,
                format!("{}: keywords present", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        Some(count) => {
            results.push(error(
                ID,
                format!("{}: keywords count invalid ({count})", krate.name),
                "Publishable crates must have between 1 and 5 keywords.".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: keywords missing", krate.name),
                "Publishable crates must have keywords in [package].".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
    }
}
