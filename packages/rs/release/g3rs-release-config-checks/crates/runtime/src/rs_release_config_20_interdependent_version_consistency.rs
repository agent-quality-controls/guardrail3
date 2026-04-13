use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-20";

pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !edge.has_path || !edge.dep_publishable {
        return;
    }
    let Some(version_req) = &edge.version_req else {
        return;
    };
    if edge.version_satisfied.unwrap_or(true) {
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
        format!("{}: version mismatch with {}", edge.crate_name, edge.dep_name),
        format!(
            "Dependency `{}`{} in `[{}]`{} requires `{}` but actual local publishable version is `{}`. Update the version requirement to match the local crate's version.",
            edge.dep_name,
            package_suffix,
            edge.section_label,
            target_suffix,
            version_req,
            edge.actual_version.as_deref().unwrap_or("unknown"),
        ),
        &edge.cargo_rel_path,
    ));
}
