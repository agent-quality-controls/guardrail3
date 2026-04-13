use g3rs_release_config_checks_types::{G3RsReleaseConfigCrate, G3RsReleaseDryRunOutcome};
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, info};

const ID: &str = "RS-RELEASE-CONFIG-18";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable {
        return;
    }

    match &krate.dry_run {
        Some(G3RsReleaseDryRunOutcome::Passed) => {
            results.push(info(
                ID,
                format!("{}: publish dry-run passed", krate.name),
                String::new(),
                &krate.cargo_rel_path,
            ));
        }
        Some(G3RsReleaseDryRunOutcome::Failed(stderr)) => {
            results.push(error(
                ID,
                format!("{}: publish dry-run failed", krate.name),
                format!("`cargo publish --dry-run` failed: {stderr}"),
                &krate.cargo_rel_path,
            ));
        }
        None => {
            results.push(error(
                ID,
                format!("{}: publish dry-run missing", krate.name),
                "Expected `cargo publish --dry-run` result, but no result was collected.".to_owned(),
                &krate.cargo_rel_path,
            ));
        }
    }
}
