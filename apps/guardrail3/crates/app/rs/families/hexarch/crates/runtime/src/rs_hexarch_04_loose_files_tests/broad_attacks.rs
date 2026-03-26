use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;
use crate::test_support::{copy_fixture, write_file};

const CONTAINER_SUFFIXES: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

fn all_owned_container_paths() -> Vec<String> {
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
fn loose_files_in_all_owned_container_dirs_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    assert_eq!(
        errors.len(),
        expected_files.len(),
        "expected exactly one loose-file hit per owned container: {errors:#?}"
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
        assert!(error.title.contains("loose files"));
        assert!(error.message.contains("mod.rs"));
    }
}

#[test]
fn multiple_loose_files_in_all_owned_container_dirs_emit_one_error_per_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/mod.rs"), "// stray");
        write_file(tmp.path(), &format!("{path}/README.md"), "# stray");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    assert_eq!(errors.len(), expected_files.len(), "{errors:#?}");
    for error in &errors {
        assert!(error.message.contains("mod.rs"), "{error:#?}");
        assert!(error.message.contains("README.md"), "{error:#?}");
    }
}

#[test]
fn near_miss_placeholder_files_hit_every_owned_container() {
    let tmp = copy_fixture();
    let expected_files = all_owned_container_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for path in &expected_files {
        write_file(tmp.path(), &format!("{path}/.gitignore"), "target/");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-04");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_files, expected_files, "{errors:#?}");
    for error in &errors {
        assert!(error.message.contains(".gitignore"), "{error:#?}");
    }
}
