use std::collections::BTreeSet;
const FIXTURE: crate::test_support::HexarchFixture = crate::test_support::HexarchFixture;

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_01_crates_exists as assertions;
use crate::test_support::{copy_fixture, remove_dir, write_file};

#[test]
fn golden_has_no_rule_01_errors() {
    let tmp = copy_fixture();
    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    assert!(errors.is_empty(), "golden should pass rule 01: {errors:#?}");
}

#[test]
fn missing_outer_crates_dir_hits_every_owned_rust_app_and_only_them() {
    let tmp = copy_fixture();
    for app in FIXTURE.apps() {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = FIXTURE
        .apps()
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}

#[test]
fn file_outer_crates_dirs_still_count_as_missing_for_the_owned_apps() {
    let tmp = copy_fixture();

    for app in FIXTURE.apps() {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates"), "not a dir");
    }

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = FIXTURE
        .apps()
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}

#[test]
fn single_app_missing_crates_hits_only_that_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/devctl".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected single-app hit set: {errors:#?}"
    );
}

#[test]
fn replacing_outer_crates_with_a_file_only_hits_the_owned_app_roots() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a directory");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/backend"].into_iter().map(str::to_owned).collect();

    assert_eq!(
        actual_files, expected_files,
        "file-vs-dir replacement should only hit the mutated owned app: {errors:#?}"
    );
}

#[test]
fn replacing_nested_crates_with_a_file_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), FIXTURE.inner_hex_root());
    write_file(tmp.path(), FIXTURE.inner_hex_root(), "not a directory");

    let results = assertions::run_family(tmp.path());
    let errors = assertions::errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested file-vs-dir cases: {errors:#?}"
    );
}
