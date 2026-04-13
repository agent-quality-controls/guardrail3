use g3rs_release_config_checks_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-23";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable || !krate.is_binary {
        return;
    }

    let (title, message) = if krate.binary_release_workflow_present {
        (
            format!("{}: binary release workflow present", krate.name),
            "A workflow builds release binaries and uses a GitHub release action.".to_owned(),
        )
    } else {
        (
            format!("{}: no binary release workflow", krate.name),
            "No workflow builds a release binary and publishes it via GitHub Releases.".to_owned(),
        )
    };

    results.push(info(ID, title, message, &krate.cargo_rel_path));
}
