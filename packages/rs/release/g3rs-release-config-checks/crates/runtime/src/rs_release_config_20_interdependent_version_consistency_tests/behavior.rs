use g3rs_release_config_checks_assertions::rs_release_config_20_interdependent_version_consistency as assertions;
use g3rs_release_types::G3RsReleaseConfigEdge;

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

    super::super::check(&edge, &mut results);

    assertions::assert_no_findings(&results);
}
