const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_03_inbound_outbound as assertions;
use super::{copy_fixture, remove_dir, write_file};

#[test]
fn replacing_outbound_dirs_with_files_hits_every_owned_directional_container() {
    let tmp = copy_fixture();
    let inner_adapters = format!("{}/adapters", inner_hex());
    let inner_ports = format!("{}/ports", inner_hex());
    for dir in [
        "apps/devctl/crates/adapters",
        "apps/backend/crates/adapters",
        "apps/worker/crates/adapters",
        inner_adapters.as_str(),
        "apps/devctl/crates/ports",
        "apps/backend/crates/ports",
        "apps/worker/crates/ports",
        inner_ports.as_str(),
    ] {
        remove_dir(tmp.path(), &format!("{dir}/outbound"));
        write_file(tmp.path(), &format!("{dir}/outbound"), "not a directory");
    }

    let results = super::run_family(tmp.path());
    assertions::assert_error_summary(
        &results,
        "",
        8,
        [
            "apps/devctl/crates/adapters",
            "apps/backend/crates/adapters",
            "apps/worker/crates/adapters",
            inner_adapters.as_str(),
            "apps/devctl/crates/ports",
            "apps/backend/crates/ports",
            "apps/worker/crates/ports",
            inner_ports.as_str(),
        ],
        None,
        Some(&["outbound"]),
        None,
        None,
    );
}

#[test]
fn replacing_inbound_dirs_with_files_on_outer_roots_does_not_double_fire_nested_hex() {
    let tmp = copy_fixture();
    for app in ["devctl", "backend", "worker"] {
        remove_dir(tmp.path(), &format!("apps/{app}/crates/adapters/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/adapters/inbound"),
            "not a directory",
        );
        remove_dir(tmp.path(), &format!("apps/{app}/crates/ports/inbound"));
        write_file(
            tmp.path(),
            &format!("apps/{app}/crates/ports/inbound"),
            "not a directory",
        );
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
            "apps/devctl/crates/ports",
            "apps/backend/crates/ports",
            "apps/worker/crates/ports",
        ],
        None,
        Some(&["inbound"]),
        None,
        None,
    );
}
