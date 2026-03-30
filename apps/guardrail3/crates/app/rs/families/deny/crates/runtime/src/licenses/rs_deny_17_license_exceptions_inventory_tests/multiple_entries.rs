use guardrail3_app_rs_family_deny_assertions::rs_deny_17_license_exceptions_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_license_exceptions};

fn exception_entry(key: &str, value: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        (key.to_owned(), toml::Value::String(value.to_owned())),
        (
            "allow".to_owned(),
            toml::Value::Array(vec![toml::Value::String("MIT".to_owned())]),
        ),
    ]))
}

#[test]
fn inventories_each_named_license_exception_entry() {
    let deny = set_license_exceptions(
        &build_fixture_deny_toml("service"),
        vec![
            exception_entry("name", "demo"),
            exception_entry("crate", "demo-legacy"),
        ],
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "license exception entry",
                "`deny.toml` has license exception for `demo`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "license exception entry",
                "`deny.toml` has license exception for `demo-legacy`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
