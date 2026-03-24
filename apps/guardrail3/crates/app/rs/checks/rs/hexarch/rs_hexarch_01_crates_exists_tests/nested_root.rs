use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, errors_by_id, remove_dir, run_family};

#[test]
fn missing_inner_hex_crates_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested missing-crates cases: {errors:#?}"
    );
}

#[test]
fn empty_inner_hex_crates_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::fs::create_dir_all(
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("recreate nested crates dir");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested empty-crates cases: {errors:#?}"
    );
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

#[test]
fn broken_nested_crates_symlink_is_not_owned_by_app_level_rule_01() {
    let tmp = copy_fixture();
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::os::unix::fs::symlink(
        "/nonexistent/path",
        tmp.path()
            .join("apps/backend/crates/adapters/inbound/mcp/crates"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 is app-level only and should not own nested broken-symlink cases: {errors:#?}"
    );
}

#[test]
fn nested_crates_symlink_loop_does_not_become_a_rule_01_hit() {
    let tmp = copy_fixture();
    let inner = tmp
        .path()
        .join("apps/backend/crates/adapters/inbound/mcp/crates");
    let outer = tmp.path().join("apps/backend/crates");
    remove_dir(
        tmp.path(),
        "apps/backend/crates/adapters/inbound/mcp/crates",
    );
    std::os::unix::fs::symlink(&outer, &inner).expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-01");
    assert!(
        errors.is_empty(),
        "rule 01 must not start owning nested symlink-loop cases after link-following: {errors:#?}"
    );
}
