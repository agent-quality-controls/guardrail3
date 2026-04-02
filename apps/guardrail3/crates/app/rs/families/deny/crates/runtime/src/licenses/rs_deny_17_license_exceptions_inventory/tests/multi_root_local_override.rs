use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_17_license_exceptions_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_license_exceptions};

#[test]
fn local_license_exception_only_warns_for_the_owned_local_root() {
    let results = super::super::run_check(&set_license_exceptions(
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
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "license exception entry",
                "`deny.toml` has documented license exception for `windows-sys`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "license exception count",
                "`deny.toml` has 1 license exceptions (1 documented, 0 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
