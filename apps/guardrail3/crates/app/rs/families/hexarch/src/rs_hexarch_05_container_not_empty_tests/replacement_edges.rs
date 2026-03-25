use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

const SAFE_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_safe_owned_container_paths() -> Vec<String> {
    let mut paths = Vec::new();
    for app in ["devctl", "backend", "worker"] {
        for suffix in SAFE_SUFFIXES {
            paths.push(format!("apps/{app}/crates/{suffix}"));
        }
    }
    for suffix in SAFE_SUFFIXES {
        paths.push(format!("{INNER_HEX}/{suffix}"));
    }
    paths
}

#[test]
fn replacing_container_dirs_with_files_hits_all_owned_app_roots() {
    let tmp = copy_fixture();
    let paths = vec![
        "apps/devctl/crates/app".to_owned(),
        "apps/backend/crates/app".to_owned(),
        "apps/worker/crates/app".to_owned(),
        format!("{INNER_HEX}/app"),
    ];
    for path in &paths {
        remove_dir(tmp.path(), path);
        write_file(tmp.path(), path, "not a directory");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(
        errors.len(),
        4,
        "expected one empty-container hit per replaced app container: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates/app".to_owned(),
        "apps/backend/crates/app".to_owned(),
        "apps/worker/crates/app".to_owned(),
        format!("{INNER_HEX}/app"),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected replacement hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.message.contains("is empty"));
    }
}

#[test]
fn replacing_nested_adapters_inbound_with_a_file_hits_nested_root_only() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/adapters/inbound"));
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/adapters/inbound"),
        "not a directory",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(
        errors.len(),
        1,
        "expected one nested-root hit for replaced adapters/inbound: {errors:#?}"
    );
    assert_eq!(
        errors[0].file.as_deref(),
        Some("apps/backend/crates/adapters/inbound/mcp/crates/adapters/inbound")
    );
}

#[test]
fn replacing_all_owned_safe_containers_with_files_hits_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        remove_dir(tmp.path(), path);
        write_file(tmp.path(), path, "not a directory");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    for error in &errors {
        assert!(error.message.contains("is empty"), "{error:#?}");
    }
}
