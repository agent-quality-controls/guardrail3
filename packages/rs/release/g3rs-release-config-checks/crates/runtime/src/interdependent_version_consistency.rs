use g3rs_release_types::G3RsReleaseConfigEdge;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

/// `ID` constant.
const ID: &str = "g3rs-release/interdependent-version-consistency";

/// `check` function.
pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !crate::support::edge_source_publishable(edge)
        || !edge.has_path
        || !crate::support::edge_target_publishable(edge)
    {
        return;
    }
    let Some(version_req) = &edge.version_req else {
        return;
    };
    if crate::support::edge_version_satisfied(edge) {
        return;
    }

    let package_suffix = if edge.dep_name == edge.dep_package_name {
        String::new()
    } else {
        format!(" (package `{}`)", edge.dep_package_name)
    };
    let target_suffix = edge
        .target_label
        .as_ref()
        .map(|target| format!(" under target `{target}`"))
        .unwrap_or_default();

    results.push(error(
        ID,
        format!("{}: version mismatch with {}", edge.source.name, edge.dep_name),
        format!(
            "Dependency `{}`{} in `[{}]`{} requires `{}` but actual local publishable version is `{}`. Update the version requirement to match the local crate's version.",
            edge.dep_name,
            package_suffix,
            edge.section_label,
            target_suffix,
            version_req,
            crate::support::edge_target_version(edge)
                .as_deref()
                .unwrap_or("unknown"),
        ),
        &edge.source.cargo_rel_path,
    ));
}
