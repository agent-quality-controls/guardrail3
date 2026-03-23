use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, remove_dir, run_family};

const INNER_HEX_APP_DIR: &str = "apps/backend/crates/adapters/inbound/mcp";

#[test]
fn missing_inner_hex_crates_hits_only_the_nested_owned_root() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = [INNER_HEX_APP_DIR.to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
    assert!(errors[0].title.contains("missing crates/"));
}

#[test]
fn missing_outer_crates_does_not_cascade_into_unreachable_nested_hex() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/backend/crates");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    let actual_files = errors
        .iter()
        .filter_map(|error| error.file.clone())
        .collect::<BTreeSet<_>>();
    let expected_files = ["apps/backend".to_owned()]
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert_eq!(
        actual_files, expected_files,
        "unexpected hit set: {errors:#?}"
    );
}
