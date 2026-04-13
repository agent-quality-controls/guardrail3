use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-19";

pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !edge.has_path || edge.dep_publishable {
        return;
    }

    let package_suffix = (edge.dep_name != edge.dep_package_name)
        .then(|| format!(" (package `{}`)", edge.dep_package_name))
        .unwrap_or_default();
    let target_suffix = edge
        .target_label
        .as_ref()
        .map(|target| format!(" under target `{target}`"))
        .unwrap_or_default();

    results.push(error(
        ID,
        format!("{}: path dep to non-publishable crate", edge.crate_name),
        format!(
            "Dependency `{}`{} in `[{}]`{} points at a non-publishable local crate. Either make the target crate publishable or replace the path dependency with a version requirement.",
            edge.dep_name, package_suffix, edge.section_label, target_suffix
        ),
        &edge.cargo_rel_path,
    ));
}
