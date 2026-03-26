use std::collections::BTreeSet;
const FIXTURE: test_support::HexarchFixture = test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use test_support::{copy_fixture, empty_dir, write_file};

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
        paths.push(format!("{}/{}", inner_hex(), suffix));
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-05");
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

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-05");
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

#[test]
fn emptying_only_inner_hex_containers_hits_inner_hex_and_leaves_outer_apps_clean() {
    let tmp = copy_fixture();
    let expected_files = [
        format!("{}/app", inner_hex()),
        format!("{}/domain", inner_hex()),
        format!("{}/adapters/inbound", inner_hex()),
        format!("{}/adapters/outbound", inner_hex()),
        format!("{}/ports/inbound", inner_hex()),
        format!("{}/ports/outbound", inner_hex()),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    for path in &expected_files {
        empty_dir(tmp.path(), path);
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
}

#[test]
fn files_only_all_owned_safe_containers_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_safe_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        empty_dir(tmp.path(), path);
        write_file(tmp.path(), &format!("{path}/README.md"), "# stray");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-05");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    for error in &errors {
        assert!(error.message.contains("contains files"), "{error:#?}");
        assert!(error.message.contains("README.md"), "{error:#?}");
    }
}
