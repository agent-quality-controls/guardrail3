use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

#[test]
fn missing_domain_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        4,
        "expected one missing-domain hit per owned root: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
        inner_hex().to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("missing") && error.title.contains("domain/"))
    );
}

#[test]
fn missing_app_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/app"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        4,
        "expected one missing-app hit per owned root: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("missing") && error.title.contains("app/"))
    );
}

#[test]
fn missing_ports_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        4,
        "expected one missing-ports hit per owned root: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("missing") && error.title.contains("ports/"))
    );
}

#[test]
fn replacing_domain_with_file_hits_missing_and_loose_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/domain"), "not a directory");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        8,
        "expected one missing-dir and one loose-file hit per owned root: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
        inner_hex().to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("missing") && error.title.contains("domain/"))
            .count(),
        4,
        "expected one missing-domain hit per owned root: {errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("loose files"))
            .count(),
        4,
        "expected one loose-file hit per owned root after dir-to-file replacement: {errors:#?}"
    );
}

#[test]
fn missing_outer_adapters_hits_only_outer_roots_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        3,
        "expected one missing-adapters hit per owned outer app: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|error| error.title.contains("missing") && error.title.contains("adapters/"))
    );
}

#[test]
fn replacing_outer_adapters_with_files_hits_only_outer_roots_because_nested_hex_becomes_unreachable()
 {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
        write_file(tmp.path(), &format!("{dir}/adapters"), "not a directory");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        6,
        "expected one missing-dir and one loose-file hit per owned outer app: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("missing") && error.title.contains("adapters/"))
            .count(),
        3,
        "expected one missing-adapters hit per owned outer app: {errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("loose files"))
            .count(),
        3,
        "expected one loose-file hit per owned outer app after dir-to-file replacement: {errors:#?}"
    );
}

#[test]
fn missing_inner_adapters_hits_only_the_nested_hex_root() {
    let tmp = copy_fixture();
        remove_dir(tmp.path(), &format!("{}/adapters", inner_hex()));

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        1,
        "expected one nested missing-adapters hit: {errors:#?}"
    );
    assert_eq!(errors[0].file.as_deref(), Some(inner_hex()), "{errors:#?}");
    assert!(errors[0].title.contains("missing") && errors[0].title.contains("adapters/"));
}

#[test]
fn missing_two_required_dirs_hits_each_owned_root_once_per_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        8,
        "expected one missing-domain and one missing-ports hit per owned root: {errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("domain/"))
            .count(),
        4,
        "{errors:#?}"
    );
    assert_eq!(
        errors
            .iter()
            .filter(|error| error.title.contains("ports/"))
            .count(),
        4,
        "{errors:#?}"
    );
}

#[test]
fn nested_optional_macros_dir_is_allowed_alongside_outer_macros() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/macros/.gitkeep"), "");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        0,
        "optional macros should not trigger rule 02 anywhere: {errors:#?}"
    );
    assert!(
        errors.is_empty(),
        "optional macros should not trigger rule 02 anywhere: {errors:#?}"
    );
}

#[test]
fn unexpected_top_level_dir_hits_only_the_mutated_owned_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/misc/.gitkeep", "");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        1,
        "unexpected top-level sibling should only hit one owned root: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl/crates/misc".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(
        errors[0]
            .title
            .contains("unexpected directory crates/misc/")
    );
}
