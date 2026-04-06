use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_17_extra_feature_bans_inventory as assertions;

use super::helpers::{build_fixture_deny_toml, set_feature_entries};

fn feature_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(vec![toml::Value::String("derive".to_owned())]),
        ),
    ]))
}

#[test]
fn local_extra_feature_bans_inventory_stays_owned_by_the_local_root() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let existing = parsed
        .get("bans")
        .and_then(|b| b.get("features"))
        .and_then(toml::Value::as_array)
        .cloned()
        .expect("expected feature inventory entries in generated deny TOML");
    let mut local_entries = existing;
    local_entries.push(feature_entry("serde"));

    let results = super::helpers::run_check(&set_feature_entries(
        &build_fixture_deny_toml("service"),
        local_entries,
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "extra feature ban",
            "`deny.toml` has extra feature-ban entry for `serde`.",
            "deny.toml",
            true,
        )],
    );
}
