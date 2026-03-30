use guardrail3_app_rs_family_deny_assertions::rs_deny_17_license_exceptions_inventory as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_license_exceptions, write_file};

#[test]
fn local_license_exception_only_inventories_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_license_exceptions(
            &build_fixture_deny_toml("service"),
            vec![toml::Value::Table(toml::map::Map::from_iter([
                (
                    "crate".to_owned(),
                    toml::Value::String("windows-sys".to_owned()),
                ),
                (
                    "allow".to_owned(),
                    toml::Value::Array(vec![toml::Value::String("Zlib".to_owned())]),
                ),
                (
                    "reason".to_owned(),
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ]))],
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "license exception entry",
            "`apps/devctl/deny.toml` has license exception for `windows-sys`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
