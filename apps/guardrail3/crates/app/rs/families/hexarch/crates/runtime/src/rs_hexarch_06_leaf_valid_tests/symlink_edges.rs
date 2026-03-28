use super::copy_fixture;
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_06_leaf_valid as assertions;

#[test]
#[cfg(unix)]
fn symlink_leaf_is_not_owned_by_rule_06() {
    let tmp = copy_fixture();
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/domain/types"),
        tmp.path().join("apps/devctl/crates/app/link_leaf"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_no_error_file_contains(&results, "apps/devctl/crates/app/link_leaf");
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error_file_contains(&results, "apps/devctl/crates/app/dangling_leaf");
}
