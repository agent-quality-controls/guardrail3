use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, RUST_APPS, copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn replacing_outer_crates_with_a_file_only_hits_the_owned_app_roots() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        if *app == "backend" {
            remove_dir(tmp.path(), "apps/backend/crates");
            write_file(tmp.path(), "apps/backend/crates", "not a directory");
        }
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
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
fn replacing_all_outer_crates_with_files_hits_all_owned_apps() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        write_file(tmp.path(), &format!("apps/{app}/crates"), "not a directory");
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
        "file replacement should hit every owned app boundary: {errors:#?}"
    );
}

#[test]
fn replacing_nested_crates_with_a_file_only_hits_the_nested_owned_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{INNER_HEX}/crates"));
    write_file(
        tmp.path(),
        &format!("{INNER_HEX}/crates"),
        "not a directory",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [INNER_HEX.to_owned()].into_iter().collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "nested file-vs-dir replacement should not leak to outer roots: {errors:#?}"
    );
}
