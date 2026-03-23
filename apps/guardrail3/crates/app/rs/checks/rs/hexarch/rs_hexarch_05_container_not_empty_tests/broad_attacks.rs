use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, copy_fixture, empty_dir, errors_by_id, run_family,
};

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_safe_owned_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in ["devctl", "backend", "worker"] {
        for suffix in CONTAINER_SUFFIXES {
            paths.push(format!("apps/{app}/crates/{suffix}"));
        }
    }
    for suffix in CONTAINER_SUFFIXES {
        paths.push(format!("{INNER_HEX}/{suffix}"));
    }
    paths
}

#[test]
fn emptying_all_owned_safe_container_dirs_hits_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        empty_dir(tmp.path(), path);
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(
        errors.len(),
        20,
        "expected one empty-container hit per owned safe container: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("empty container"));
        assert!(error.message.contains("is empty"));
    }
}

#[test]
fn emptying_outer_adapters_inbound_destroys_the_nested_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        empty_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(
        errors.len(),
        3,
        "expected one outer adapters/inbound hit per owned app: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/adapters/inbound".to_owned(),
        "apps/backend/crates/adapters/inbound".to_owned(),
        "apps/worker/crates/adapters/inbound".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}
