use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_04_loose_files as assertions;

#[test]
fn ignored_untracked_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    write_file(tmp.path(), "apps/devctl/crates/app/.env", "SECRET=1\n");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &[],
        &[],
        &[".env"],
        &[],
    );
}

#[test]
#[cfg(unix)]
fn ignored_symlink_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &[],
        &[],
        &[".env"],
        &[],
    );
}

#[test]
#[cfg(unix)]
fn ignored_broken_symlink_loose_files_still_hit_rule_04() {
    let tmp = copy_fixture();
    write_file(tmp.path(), ".gitignore", "*.env\n");
    std::os::unix::fs::symlink(
        tmp.path().join("missing-target"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/app",
        1,
        &[],
        &[],
        &[".env"],
        &[],
    );
}
