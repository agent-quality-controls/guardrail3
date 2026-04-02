use std::collections::BTreeSet;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_01_crates_exists as assertions;

#[test]
fn golden_has_no_rule_01_errors() {
    let tmp = copy_fixture();
    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn missing_outer_crates_dir_hits_every_owned_rust_app_and_only_them() {
    let tmp = copy_fixture();
    for app in FIXTURE.apps() {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }

    let results = super::run_family(tmp.path());
    let expected_files = FIXTURE
        .apps()
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        &expected_files
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        None,
        None,
        None,
        None,
    );
}

#[test]
fn file_outer_crates_dirs_still_count_as_missing_for_the_owned_apps() {
    let tmp = copy_fixture();

    for app in FIXTURE.apps() {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates"), "not a dir");
    }

    let results = super::run_family(tmp.path());
    let expected_files = FIXTURE
        .apps()
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        &expected_files
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        None,
        None,
        None,
        None,
    );
}

#[test]
fn single_app_missing_crates_hits_only_that_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");

    let results = super::run_family(tmp.path());
    let expected_files = ["apps/devctl".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        &expected_files
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        None,
        None,
        None,
        None,
    );
}

#[test]
fn replacing_outer_crates_with_a_file_only_hits_the_owned_app_roots() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a directory");

    let results = super::run_family(tmp.path());
    let expected_files = ["apps/backend"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    assertions::assert_error_summary(
        &results,
        "",
        expected_files.len(),
        &expected_files
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        None,
        None,
        None,
        None,
    );
}

#[test]
fn replacing_nested_crates_with_a_file_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), FIXTURE.inner_hex_root());
    write_file(tmp.path(), FIXTURE.inner_hex_root(), "not a directory");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
