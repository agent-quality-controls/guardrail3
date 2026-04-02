const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::structure::rs_hexarch_03_inbound_outbound as assertions;

#[test]
fn unexpected_directional_dir_hits_only_the_mutated_owned_container() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "apps/devctl/crates/ports/sideways/.gitkeep", "");

    let results = super::run_family(tmp.path());
    assertions::assert_error_count_matching_file(
        &results,
        "",
        "apps/devctl/crates/ports/sideways",
        1,
        &["unexpected directory crates/ports/sideways/"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn unexpected_dir_in_adapters_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    let inner_adapters = format!("{}/adapters", inner_hex());
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        inner_adapters.as_str(),
    ] {
        write_file(tmp.path(), &format!("{dir}/shared/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/adapters/shared",
            "apps/backend/crates/adapters/shared",
            "apps/worker/crates/adapters/shared",
            &format!("{inner_adapters}/shared"),
        ],
        None,
        Some(&["unexpected", "shared"]),
        None,
        None,
    );
}

#[test]
fn unexpected_dir_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_ports.as_str(),
    ] {
        write_file(tmp.path(), &format!("{dir}/common/.gitkeep"), "");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/ports/common",
            "apps/backend/crates/ports/common",
            "apps/worker/crates/ports/common",
            &format!("{inner_ports}/common"),
        ],
        None,
        Some(&["unexpected", "common"]),
        None,
        None,
    );
}

#[test]
fn deep_unexpected_dir_tree_blames_only_the_top_level_unexpected_dir() {
    let tmp = copy_fixture();
    let inner_adapters = format!("{}/adapters", inner_hex());
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_adapters.as_str(),
        inner_ports.as_str(),
    ] {
        write_file(
            tmp.path(),
            &format!("{dir}/utils/helpers/deep/lib.rs"),
            "// buried",
        );
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates/adapters/utils",
            "apps/backend/crates/adapters/utils",
            "apps/worker/crates/adapters/utils",
            "apps/devctl/crates/ports/utils",
            "apps/backend/crates/ports/utils",
            "apps/worker/crates/ports/utils",
            &format!("{inner_adapters}/utils"),
            &format!("{inner_ports}/utils"),
        ],
        None,
        Some(&["utils"]),
        Some(&["helpers", "deep"]),
        None,
    );
}
