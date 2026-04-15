use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;
use guardrail3_check_types::G3CheckResult;

use crate::support::error;

const ID: &str = "RS-RELEASE-CONFIG-20";

pub(crate) fn check(edge: &G3RsReleaseConfigEdge, results: &mut Vec<G3CheckResult>) {
    if !edge.source_publishable || !edge.has_path || !edge.dep_publishable {
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

#[cfg(test)]
mod tests {
    use super::check;
    use g3rs_release_config_checks_types::G3RsReleaseConfigEdge;

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
            path_target_kind: None,
            dep_publishable: true,
            version_req: Some("^0.2.0".to_owned()),
            actual_version: Some("0.1.0".to_owned()),
            version_satisfied: Some(false),
        }
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
