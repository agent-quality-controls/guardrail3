use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, RUST_APPS, copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn replacing_outer_crates_with_a_file_only_hits_the_owned_app_roots() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates");
    write_file(tmp.path(), "apps/backend/crates", "not a directory");

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
fn replacing_nested_crates_with_a_file_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), INNER_HEX);
    write_file(tmp.path(), INNER_HEX, "not a directory");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested file-vs-dir cases: {errors:#?}"
    );
}

#[test]
fn replacing_all_outer_crates_with_broken_symlinks_hits_all_owned_apps() {
    let tmp = copy_fixture();
    for app in RUST_APPS {
        remove_dir(tmp.path(), &format!("apps/{app}/crates"));
        std::os::unix::fs::symlink(
            "/nonexistent/path",
            tmp.path().join(format!("apps/{app}/crates")),
        )
        .expect("symlink");
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
        "unexpected hit set for broken symlink crates/: {errors:#?}"
    );
}

#[test]
fn replacing_outer_crates_with_dev_null_symlink_hits_that_app() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/devctl/crates"))
        .expect("symlink");

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
        "unexpected hit set for /dev/null crates/ symlink: {errors:#?}"
    );
}

#[test]
fn symlinking_outer_crates_to_another_valid_app_stays_out_of_rule_01() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/worker/crates"),
        tmp.path().join("apps/devctl/crates"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "symlink to a valid crates/ root should stay out of rule 01: {errors:#?}"
    );
}
