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
fn empty_and_file_crates_dirs_still_count_as_missing_for_the_owned_apps() {
    let tmp = copy_fixture();

    remove_dir(tmp.path(), "apps/devctl/crates");
    write_file(tmp.path(), "apps/devctl/crates", "not a dir");
    remove_dir(tmp.path(), "apps/backend/crates");
    create_dir(tmp.path(), "apps/backend/crates");
    remove_dir(tmp.path(), "apps/worker/crates");

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
