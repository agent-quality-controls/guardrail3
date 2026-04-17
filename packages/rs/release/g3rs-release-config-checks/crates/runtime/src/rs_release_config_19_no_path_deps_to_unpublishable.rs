use g3rs_release_types::{G3RsReleaseConfigEdge, G3RsReleasePathTargetKind};
use guardrail3_check_types::G3CheckResult;

use crate::support::{error, warn};

const ID: &str = "RS-RELEASE-CONFIG-19";

pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !edge.source_publishable || !edge.has_path || edge.dep_publishable {
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

    match edge.path_target_kind {
        Some(G3RsReleasePathTargetKind::InWorkspace) => {
            results.push(error(
                ID,
                format!("{}: path dep to non-publishable crate", edge.crate_name),
                format!(
                    "Dependency `{}`{} in `[{}]`{} points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
                    edge.dep_name, package_suffix, edge.section_label, target_suffix
                ),
                &edge.cargo_rel_path,
            ));
        }
        Some(G3RsReleasePathTargetKind::OutsideWorkspace) => {
            if edge.version_req.is_some() {
                results.push(warn(
                    ID,
                    format!("{}: path dep escapes workspace", edge.crate_name),
                    format!(
                        "Dependency `{}`{} in `[{}]`{} points outside this workspace by path. Replace it with a normal versioned dependency if this crate is meant to publish.",
                        edge.dep_name, package_suffix, edge.section_label, target_suffix
                    ),
                    &edge.cargo_rel_path,
                ));
            } else {
                results.push(error(
                    ID,
                    format!("{}: path dep escapes workspace", edge.crate_name),
                    format!(
                        "Dependency `{}`{} in `[{}]`{} points outside this workspace by path and has no version requirement. Replace it with a normal versioned dependency.",
                        edge.dep_name, package_suffix, edge.section_label, target_suffix
                    ),
                    &edge.cargo_rel_path,
                ));
            }
        }
        None => {
            if edge.actual_version.is_none() && edge.version_req.is_some() {
                return;
            }
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
    }
}

#[cfg(test)]
#[path = "rs_release_config_19_no_path_deps_to_unpublishable_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_release_config_19_no_path_deps_to_unpublishable_tests;
