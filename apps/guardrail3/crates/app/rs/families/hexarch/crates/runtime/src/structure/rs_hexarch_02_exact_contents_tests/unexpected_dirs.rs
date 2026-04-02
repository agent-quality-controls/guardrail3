const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_02_exact_contents as assertions;

#[test]
fn unexpected_utils_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/utils",
            "apps/backend/crates/utils",
            "apps/worker/crates/utils",
            &format!("{}/utils", inner_hex()),
        ],
        None,
        None,
        None,
        None,
    );
}

#[test]
fn unexpected_dir_inner_hex_only_hits_only_the_nested_hex_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), &format!("{}/utils/.gitkeep", inner_hex()), "");

    let expected_file = format!("{}/utils", inner_hex());
    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        &expected_file,
        1,
        &[],
        &[],
        &[],
        &[],
    );
}

#[test]
fn multiple_unexpected_dirs_hit_each_owned_root_once_per_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/utils/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/helpers/.gitkeep"), "");
        write_file(tmp.path(), &format!("{dir}/config/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        12,
        [
            "apps/devctl/crates/utils",
            "apps/devctl/crates/helpers",
            "apps/devctl/crates/config",
            "apps/backend/crates/utils",
            "apps/backend/crates/helpers",
            "apps/backend/crates/config",
            "apps/worker/crates/utils",
            "apps/worker/crates/helpers",
            "apps/worker/crates/config",
            &format!("{}/utils", inner_hex()),
            &format!("{}/helpers", inner_hex()),
            &format!("{}/config", inner_hex()),
        ],
        None,
        None,
        None,
        None,
    );
}

#[test]
fn near_miss_required_dir_names_are_unexpected_everywhere() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        for name in ["domains", "adapter", "port", "application"] {
            write_file(tmp.path(), &format!("{dir}/{name}/.gitkeep"), "");
        }
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        16,
        [
            "apps/devctl/crates/domains",
            "apps/devctl/crates/adapter",
            "apps/devctl/crates/port",
            "apps/devctl/crates/application",
            "apps/backend/crates/domains",
            "apps/backend/crates/adapter",
            "apps/backend/crates/port",
            "apps/backend/crates/application",
            "apps/worker/crates/domains",
            "apps/worker/crates/adapter",
            "apps/worker/crates/port",
            "apps/worker/crates/application",
            &format!("{}/domains", inner_hex()),
            &format!("{}/adapter", inner_hex()),
            &format!("{}/port", inner_hex()),
            &format!("{}/application", inner_hex()),
        ],
        None,
        Some(&["unexpected"]),
        None,
        None,
    );
}

#[test]
fn gitkeep_directory_is_unexpected_not_an_allowed_gitkeep_file() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/crates/.gitkeep/nested.txt",
        "not allowed",
    );

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/.gitkeep",
        1,
        &["unexpected"],
        &[],
        &[],
        &[],
    );
}
