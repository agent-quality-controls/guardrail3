use g3rs_release_types::G3RsReleaseConfigCrate;
use guardrail3_check_types::G3CheckResult;

use crate::support::info;

const ID: &str = "RS-RELEASE-CONFIG-24";

pub(crate) fn check(krate: &G3RsReleaseConfigCrate, results: &mut Vec<G3CheckResult>) {
    if !krate.publishable || !krate.is_binary {
        return;
    }

    let (title, message) = if krate.linux_release_target_present {
        (
            format!("{}: linux release target present", krate.name),
            "A workflow includes a Linux target.".to_owned(),
        )
    } else {
        (
            format!("{}: no linux release target", krate.name),
            "No workflow includes a Linux target for binary release.".to_owned(),
        )
    };

    results.push(info(ID, title, message, &krate.cargo_rel_path));
}
