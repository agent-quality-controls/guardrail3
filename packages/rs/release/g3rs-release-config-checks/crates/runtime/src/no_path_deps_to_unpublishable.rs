use g3rs_release_types::{G3RsReleaseConfigEdge, G3RsReleasePathTargetKind};
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, warn};

/// `ID` constant.
const ID: &str = "g3rs-release/no-path-deps-to-unpublishable";

/// `check` function.
pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !crate::support::edge_source_publishable(edge)
        || !edge.has_path
        || crate::support::edge_target_publishable(edge)
    {
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

    match edge.path_target_kind {
        Some(G3RsReleasePathTargetKind::InWorkspace) => {
            results.push(error(
                ID,
                format!("{}: path dep to non-publishable crate", edge.source.name),
                format!(
                    "Dependency `{}`{} in `[{}]`{} points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
                    edge.dep_name, package_suffix, edge.section_label, target_suffix
                ),
                &edge.source.cargo_rel_path,
            ));
        }
        Some(G3RsReleasePathTargetKind::OutsideWorkspace) => {
            if edge.version_req.is_some() {
                results.push(warn(
                    ID,
                    format!("{}: path dep escapes workspace", edge.source.name),
                    format!(
                        "Dependency `{}`{} in `[{}]`{} points outside this workspace by path. Replace it with a normal versioned dependency if this crate is meant to publish.",
                        edge.dep_name, package_suffix, edge.section_label, target_suffix
                    ),
                    &edge.source.cargo_rel_path,
                ));
            } else {
                results.push(error(
                    ID,
                    format!("{}: path dep escapes workspace", edge.source.name),
                    format!(
                        "Dependency `{}`{} in `[{}]`{} points outside this workspace by path and has no version requirement. Replace it with a normal versioned dependency.",
                        edge.dep_name, package_suffix, edge.section_label, target_suffix
                    ),
                    &edge.source.cargo_rel_path,
                ));
            }
        }
        None => {
            if crate::support::edge_target_version(edge).is_none() && edge.version_req.is_some() {
                return;
            }
            results.push(error(
                ID,
                format!("{}: path dep to non-publishable crate", edge.source.name),
                format!(
                    "Dependency `{}`{} in `[{}]`{} points at a non-publishable local crate. Either make the target crate publishable or replace the path dependency with a version requirement.",
                    edge.dep_name, package_suffix, edge.section_label, target_suffix
                ),
                &edge.source.cargo_rel_path,
            ));
        }
    }
}

#[cfg(test)]
#[path = "no_path_deps_to_unpublishable_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod no_path_deps_to_unpublishable_tests;
