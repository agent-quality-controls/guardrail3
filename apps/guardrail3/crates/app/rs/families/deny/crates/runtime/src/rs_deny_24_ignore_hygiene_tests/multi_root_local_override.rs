use guardrail3_app_rs_family_deny_assertions::rs_deny_24_ignore_hygiene as assertions;

use super::super::{copy_fixture, set_advisory_ignores, write_file, build_fixture_deny_toml};

#[test]
fn local_advisory_ignore_inventory_only_hits_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_advisory_ignores(
            &build_fixture_deny_toml("service"),
            vec![toml::Value::String("RUSTSEC-2026-0000".to_owned())],
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "advisory ignore entry",
            "`apps/devctl/deny.toml` ignores advisory `RUSTSEC-2026-0000`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
