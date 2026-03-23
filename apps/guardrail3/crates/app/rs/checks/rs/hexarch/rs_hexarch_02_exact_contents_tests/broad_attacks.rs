use std::collections::BTreeSet;

use super::super::super::test_support::{
    INNER_HEX, copy_fixture, errors_by_id, remove_dir, run_family, write_file,
};

#[test]
fn missing_domain_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/domain"), "not a directory");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        4,
        "expected one missing-dir hit per owned root: {errors:#?}"
    );
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [
        "apps/devctl/crates".to_owned(),
        "apps/backend/crates".to_owned(),
        "apps/worker/crates".to_owned(),
        format!("{INNER_HEX}"),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    for error in &errors {
        assert!(error.title.contains("missing"));
        assert!(error.title.contains("domain/"));
        assert!(!error.title.contains("unexpected directory"));
    }
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
        write_file(tmp.path(), &format!("{dir}/adapters"), "not a directory");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
    assert_eq!(
        errors.len(),
        3,
        "expected one outer-root hit per owned app: {errors:#?}"
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
    for error in &errors {
        assert!(error.title.contains("adapters/"));
        assert!(!error.title.contains("unexpected directory"));
    }
}

#[test]
fn nested_optional_macros_dir_is_allowed_alongside_outer_macros() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        INNER_HEX,
    ] {
        write_file(tmp.path(), &format!("{dir}/macros/.gitkeep"), "");
    }

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
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

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-02");
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
