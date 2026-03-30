const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{copy_fixture, remove_dir};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;

#[test]
fn missing_outbound_in_adapters_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    let inner_adapters = format!("{}/adapters", inner_hex());
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        inner_adapters.as_str(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/adapters",
            "apps/backend/crates/adapters",
            "apps/worker/crates/adapters",
            inner_adapters.as_str(),
        ],
        None,
        Some(&["outbound", "adapters"]),
        None,
        None,
    );
}

#[test]
fn missing_inbound_in_adapters_hits_only_outer_containers_because_nested_hex_becomes_unreachable() {
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        3,
        [
            "apps/devctl/crates/adapters",
            "apps/backend/crates/adapters",
            "apps/worker/crates/adapters",
        ],
        None,
        Some(&["inbound", "adapters"]),
        None,
        None,
    );
}

#[test]
fn missing_inbound_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_ports.as_str(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/ports",
            "apps/backend/crates/ports",
            "apps/worker/crates/ports",
            inner_ports.as_str(),
        ],
        None,
        Some(&["inbound", "ports"]),
        None,
        None,
    );
}

#[test]
fn missing_outbound_in_ports_hits_all_owned_outer_and_nested_containers() {
    let tmp = copy_fixture();
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_ports.as_str(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        4,
        [
            "apps/devctl/crates/ports",
            "apps/backend/crates/ports",
            "apps/worker/crates/ports",
            inner_ports.as_str(),
        ],
        None,
        Some(&["outbound", "ports"]),
        None,
        None,
    );
}

#[test]
fn both_direction_dirs_missing_in_ports_emit_two_missing_results_per_owned_container() {
    let tmp = copy_fixture();
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_ports.as_str(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates/ports",
            "apps/backend/crates/ports",
            "apps/worker/crates/ports",
            inner_ports.as_str(),
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
        &["/inbound/ directory"],
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
        &["/outbound/ directory"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn both_direction_dirs_missing_in_adapters_emit_two_missing_results_per_surviving_outer_container()
{
    let tmp = copy_fixture();
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
    ] {
        remove_dir(tmp.path(), &format!("{dir}/inbound"));
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        6,
        [
            "apps/devctl/crates/adapters",
            "apps/backend/crates/adapters",
            "apps/worker/crates/adapters",
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
        &["inbound"],
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
        &["outbound"],
        &[],
        &[],
        &[],
    );
}

#[test]
fn removing_outer_backend_inbound_destroys_the_nested_hex_path_and_does_not_double_fire() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        3,
        [
            "apps/devctl/crates/adapters",
            "apps/backend/crates/adapters",
            "apps/worker/crates/adapters",
        ],
        None,
        None,
        None,
        None,
    );
}
