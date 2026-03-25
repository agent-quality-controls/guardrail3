use std::collections::BTreeSet;

use super::super::super::test_support::{
    RUST_APPS, copy_fixture, create_dir, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn missing_outer_crates_dir_hits_every_owned_rust_app_and_only_them() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = RUST_APPS
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing crates/"));
    }
}

#[test]
fn empty_outer_crates_dirs_still_count_as_missing_for_the_owned_apps() {
    let tmp = copy_fixture();

    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        create_dir(tmp.path(), &format!("apps/{app}/crates"));
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/backend", "apps/devctl", "apps/worker"]
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing crates/"));
    }
}

#[test]
fn file_outer_crates_dirs_still_count_as_missing_for_the_owned_apps() {
    let tmp = copy_fixture();

    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates"), "not a dir");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = RUST_APPS
        .iter()
        .map(|app| format!("apps/{app}"))
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing crates/"));
    }
}

#[test]
fn single_app_missing_crates_hits_only_that_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
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
fn single_app_empty_crates_hits_only_that_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/worker/crates");
    create_dir(tmp.path(), "apps/worker/crates");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/worker".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected single-app empty-dir hit set: {errors:#?}"
    );
}
