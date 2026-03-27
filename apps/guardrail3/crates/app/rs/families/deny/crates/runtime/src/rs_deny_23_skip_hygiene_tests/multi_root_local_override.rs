use guardrail3_app_rs_family_deny_assertions::rs_deny_23_skip_hygiene as assertions;

use super::super::{add_skip_entry, copy_fixture, write_file, build_fixture_deny_toml};

#[test]
fn local_skip_inventory_only_hits_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &add_skip_entry(
            &build_fixture_deny_toml("service"),
            toml::Value::String("plain-crate".to_owned()),
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "skip entry",
            "`apps/devctl/deny.toml` has skip entry `plain-crate`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
