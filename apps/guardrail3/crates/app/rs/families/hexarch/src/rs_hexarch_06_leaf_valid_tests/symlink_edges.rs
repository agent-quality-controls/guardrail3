use super::super::super::test_support::{copy_fixture, errors_by_id, run_family};

#[test]
#[cfg(unix)]
fn symlink_leaf_is_not_owned_by_rule_06() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/domain/types"),
        tmp.path().join("apps/devctl/crates/app/link_leaf"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors
            .iter()
            .all(|error| error.file.as_deref() != Some("apps/devctl/crates/app/link_leaf")),
        "symlink leaves should not materialize rule-06 errors: {errors:#?}"
    );
}

#[test]
#[cfg(unix)]
fn dangling_symlink_leaf_is_not_owned_by_rule_06() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        "/nonexistent/leaf-target",
        tmp.path().join("apps/devctl/crates/app/dangling_leaf"),
    )
    .expect("symlink");

    let results = run_family(tmp.path());
    let errors = errors_by_id(&results, "RS-HEXARCH-06");
    assert!(
        errors
            .iter()
            .all(|error| error.file.as_deref() != Some("apps/devctl/crates/app/dangling_leaf")),
        "dangling symlink leaves should not materialize rule-06 errors: {errors:#?}"
    );
}
