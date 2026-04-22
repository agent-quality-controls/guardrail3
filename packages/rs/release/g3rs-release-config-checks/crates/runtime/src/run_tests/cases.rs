use std::collections::BTreeSet;

use g3rs_release_config_checks_assertions::run as run_assertions;
use g3rs_release_types::{
    G3RsReleaseConfigChecksInput, G3RsReleaseConfigEdge, G3RsReleaseConfigRepo,
    G3RsReleaseInputFailure, G3RsReleasePathTargetKind,
};

#[test]
fn dispatches_prebound_config_lanes() {
    let results = crate::run::check(&G3RsReleaseConfigChecksInput {
        repo_checks: vec![G3RsReleaseConfigRepo {
            cargo_rel_path: "Cargo.toml".to_owned(),
            release_plz_rel_path: "release-plz.toml".to_owned(),
            release_plz_exists: false,
            release_plz: None,
            release_plz_package_names: BTreeSet::new(),
            cliff_rel_path: "cliff.toml".to_owned(),
            cliff_exists: true,
            cliff: None,
            has_release_plz_workflow: false,
            release_plz_workflow_rel_path: None,
            has_publish_dry_run_workflow: false,
            publish_dry_run_workflow_rel_path: None,
            has_registry_token_workflow: false,
            registry_token_workflow_rel_path: None,
            publishable_crate_names: BTreeSet::new(),
            publishable_binary_crate_names: BTreeSet::new(),
            publishable_count: 1,
            non_publishable_count: 0,
            semver_checks_installed: true,
            publish_setting: Some("true".to_owned()),
            release_profile_settings: Vec::new(),
        }],
        crate_checks: vec![
            crate::lib_tests::test_support::config_input_for_crate(
                r#"
[package]
name = "demo"
version = "0.1.0"
"#,
                None,
            )
            .crate_checks
            .into_iter()
            .next()
            .expect("crate fixture should exist"),
        ],
        edge_checks: vec![G3RsReleaseConfigEdge {
            crate_name: "demo".to_owned(),
            cargo_rel_path: "crates/demo/Cargo.toml".to_owned(),
            source_publishable: true,
            dep_name: "private".to_owned(),
            dep_package_name: "private".to_owned(),
            section_label: "dependencies".to_owned(),
            target_label: None,
            has_path: true,
            path_target_kind: Some(G3RsReleasePathTargetKind::InWorkspace),
            dep_publishable: false,
            version_req: None,
            actual_version: None,
            version_satisfied: None,
        }],
        input_failure_checks: vec![G3RsReleaseInputFailure {
            rel_path: "Cargo.toml".to_owned(),
            message: "config failure".to_owned(),
        }],
    });

    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-00", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-15", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-19", 1);
    run_assertions::assert_result_id_count(&results, "RS-RELEASE-CONFIG-25", 1);
}
