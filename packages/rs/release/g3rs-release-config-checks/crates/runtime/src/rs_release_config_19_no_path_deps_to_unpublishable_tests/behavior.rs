use g3rs_release_config_checks_assertions::rs_release_config_19_no_path_deps_to_unpublishable as assertions;
use g3rs_release_types::{G3RsReleaseConfigEdge, G3RsReleasePathTargetKind};

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

    super::super::check(&edge(), &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "crate-a: path dep to non-publishable crate",
            "Dependency `dep-a` in `[dependencies]` points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn stands_down_for_external_path_dep_with_version_requirement() {
    let mut edge = edge();
    edge.path_target_kind = None;
    edge.version_req = Some("^0.1.0".to_owned());
    edge.actual_version = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_for_local_unpublishable_path_dep_even_with_version_requirement() {
    let mut edge = edge();
    edge.version_req = Some("^0.1.0".to_owned());
    edge.actual_version = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "crate-a: path dep to non-publishable crate",
            "Dependency `dep-a` in `[dependencies]` points at a crate inside this workspace that is not publishable. Make the target crate publishable or stop depending on it from a publishable crate.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn warns_for_outside_workspace_path_dep_with_version_requirement() {
    let mut edge = edge();
    edge.path_target_kind = Some(G3RsReleasePathTargetKind::OutsideWorkspace);
    edge.version_req = Some("^0.1.0".to_owned());
    edge.actual_version = None;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "crate-a: path dep escapes workspace",
            "Dependency `dep-a` in `[dependencies]` points outside this workspace by path. Replace it with a normal versioned dependency if this crate is meant to publish.",
            "Cargo.toml",
            false,
        )],
    );
}

#[test]
fn skips_non_publishable_source_crate() {
    let mut edge = edge();
    edge.source_publishable = false;
    let mut results = Vec::new();

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}
