use super::{copy_fixture, remove_dir};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

#[test]
fn required_child_symlink_to_valid_directory_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/crates/app"),
        tmp.path().join("apps/devctl/crates/domain"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        2,
        &[],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["missing", "domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        1,
        &["loose files"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn required_child_broken_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink(
        "/nonexistent/path",
        tmp.path().join("apps/devctl/crates/domain"),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        2,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn required_child_dev_null_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");
    std::os::unix::fs::symlink("/dev/null", tmp.path().join("apps/devctl/crates/domain"))
        .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates",
        2,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn nested_required_child_valid_symlink_hits_missing_and_loose_for_that_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{}/domain", inner_hex()));
    std::os::unix::fs::symlink(
        tmp.path().join(format!("{}/app", inner_hex())),
        tmp.path().join(format!("{}/domain", inner_hex())),
    )
    .expect("symlink");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(&results, "", inner_hex(), 2, &[], &[], &[], &[]);
    assertions::assert_error_count_matching_file(
        &results,
        "",
        inner_hex(),
        1,
        &["missing", "domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching_file(
        &results,
        "",
        inner_hex(),
        1,
        &["loose files"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn required_child_symlink_hits_every_owned_root_for_non_special_required_name() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        std::os::unix::fs::symlink(
            tmp.path().join(format!("{dir}/app")),
            tmp.path().join(format!("{dir}/domain")),
        )
        .expect("symlink");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["missing", "domain/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        4,
        None,
        None,
        &["loose files"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn outer_adapters_symlink_hits_only_outer_roots_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
        std::os::unix::fs::symlink(
            tmp.path().join(format!("{dir}/app")),
            tmp.path().join(format!("{dir}/adapters")),
        )
        .expect("symlink");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        6,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
        ],
        None,
        None,
        None,
        None,
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        3,
        None,
        None,
        &["missing", "adapters/"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        3,
        None,
        None,
        &["loose files"],
        &[],
        &[],
        &[],
    );
    assertions::assert_error_count_matching(
        &results,
        "",
        0,
        Some(inner_hex()),
        None,
        &[],
        &[],
        &[],
        &[],
    );
}
