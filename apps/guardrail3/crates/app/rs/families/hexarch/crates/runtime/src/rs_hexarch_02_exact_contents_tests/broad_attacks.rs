const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, remove_dir, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_02_exact_contents as assertions;

#[test]
fn missing_domain_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["missing", "domain/"]),
        None,
        None,
    );
}

#[test]
fn missing_app_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/app"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["missing", "app/"]),
        None,
        None,
    );
}

#[test]
fn missing_ports_hits_all_owned_outer_and_nested_hex_roots() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/ports"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
            inner_hex(),
        ],
        None,
        Some(&["missing", "ports/"]),
        None,
        None,
    );
}

#[test]
fn replacing_domain_with_file_hits_missing_and_loose_per_owned_root() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        write_file(tmp.path(), &format!("{dir}/domain"), "not a directory");
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
fn missing_outer_adapters_hits_only_outer_roots_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        3,
        [
            "apps/devctl/crates",
            "apps/backend/crates",
            "apps/worker/crates",
        ],
        None,
        Some(&["missing", "adapters/"]),
        None,
        None,
    );
}

#[test]
fn replacing_outer_adapters_with_files_hits_only_outer_roots_because_nested_hex_becomes_unreachable()
 {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/adapters"));
        write_file(tmp.path(), &format!("{dir}/adapters"), "not a directory");
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
}

#[test]
fn missing_inner_adapters_hits_only_the_nested_hex_root() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), &format!("{}/adapters", inner_hex()));

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        inner_hex(),
        1,
        &["missing", "adapters/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn missing_two_required_dirs_hits_each_owned_root_once_per_dir() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/domain"));
        remove_dir(tmp.path(), &format!("{dir}/ports"));
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
        &["domain/"],
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
        &["ports/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn nested_optional_macros_dir_is_allowed_alongside_outer_macros() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates",
        "apps/backend/crates",
        "apps/worker/crates",
        inner_hex(),
    ] {
        write_file(tmp.path(), &format!("{dir}/macros/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}

#[test]
fn unexpected_top_level_dir_hits_only_the_mutated_owned_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/misc/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        1,
        ["apps/devctl/crates/misc"],
        None,
        Some(&["unexpected directory crates/misc/"]),
        None,
        None,
    );
}
