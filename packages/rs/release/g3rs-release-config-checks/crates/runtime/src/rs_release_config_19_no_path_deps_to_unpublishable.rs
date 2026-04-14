use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-19";

pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !edge.has_path || edge.dep_publishable {
        return;
    }

    if edge.actual_version.is_none() && edge.version_req.is_some() {
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

#[cfg(test)]
mod tests {
    use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;

    use super::check;

    fn edge() -> G3RsReleaseConfigEdge {
        G3RsReleaseConfigEdge {
            crate_name: "crate-a".to_owned(),
            cargo_rel_path: "Cargo.toml".to_owned(),
            dep_name: "dep-a".to_owned(),
            dep_package_name: "dep-a".to_owned(),
            section_label: "dependencies".to_owned(),
            target_label: None,
            has_path: true,
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
        edge.version_req = Some("^0.1.0".to_owned());
        edge.actual_version = None;
        let mut results = Vec::new();

        check(&edge, &mut results);

        assert!(results.is_empty());
    }
}
