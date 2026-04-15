use g3rs_release_config_checks_types::{G3RsReleaseConfigEdge, G3RsReleasePathTargetKind};
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
mod tests {
    use g3rs_release_config_checks_types::{G3RsReleaseConfigEdge, G3RsReleasePathTargetKind};

    use super::check;

    fn edge() -> G3RsReleaseConfigEdge {
        G3RsReleaseConfigEdge {
            crate_name: "crate-a".to_owned(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            source_publishable: true,
            dep_name: "dep-a".to_owned(),
            dep_package_name: "dep-a".to_owned(),
            section_label: "dependencies".to_owned(),
            target_label: None,
            has_path: true,
            path_target_kind: Some(G3RsReleasePathTargetKind::InWorkspace),
            dep_publishable: false,
            version_req: None,
            actual_version: Some("0.1.0".to_owned()),
            version_satisfied: None,
        }
    }

    #[test]
    fn errors_for_local_path_dep_to_non_publishable_crate() {
        let mut results = Vec::new();

        check(&edge(), &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id(), "RS-RELEASE-CONFIG-19");
        assert_eq!(results[0].title(), "crate-a: path dep to non-publishable crate");
    }

    #[test]
    fn stands_down_for_external_path_dep_with_version_requirement() {
        let mut edge = edge();
        edge.path_target_kind = None;
        edge.version_req = Some("^0.1.0".to_owned());
        edge.actual_version = None;
        let mut results = Vec::new();

        check(&edge, &mut results);

        assert!(results.is_empty());
    }

    #[test]
    fn errors_for_local_unpublishable_path_dep_even_with_version_requirement() {
        let mut edge = edge();
        edge.version_req = Some("^0.1.0".to_owned());
        edge.actual_version = None;
        let mut results = Vec::new();

        check(&edge, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].severity(), guardrail3_check_types::G3Severity::Error);
        assert_eq!(results[0].title(), "crate-a: path dep to non-publishable crate");
    }

    #[test]
    fn warns_for_outside_workspace_path_dep_with_version_requirement() {
        let mut edge = edge();
        edge.path_target_kind = Some(G3RsReleasePathTargetKind::OutsideWorkspace);
        edge.version_req = Some("^0.1.0".to_owned());
        edge.actual_version = None;
        let mut results = Vec::new();

        check(&edge, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].severity(), guardrail3_check_types::G3Severity::Warn);
        assert_eq!(results[0].title(), "crate-a: path dep escapes workspace");
    }

    #[test]
    fn skips_non_publishable_source_crate() {
        let mut edge = edge();
        edge.source_publishable = false;
        let mut results = Vec::new();

        check(&edge, &mut results);

        assert!(results.is_empty());
    }
}
