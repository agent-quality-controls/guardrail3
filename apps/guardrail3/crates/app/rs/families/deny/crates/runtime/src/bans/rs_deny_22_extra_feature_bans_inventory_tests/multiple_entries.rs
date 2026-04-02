use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_22_extra_feature_bans_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_feature_entries};

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
fn inventories_each_non_tokio_feature_ban_entry() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let existing = parsed
        .get("bans")
        .and_then(|b| b.get("features"))
        .and_then(toml::Value::as_array)
        .cloned()
        .expect("expected feature inventory entries in generated deny TOML");
    let mut entries = existing;
    entries.push(feature_entry("serde"));
    entries.push(feature_entry("axum"));
    let results = super::super::run_check(&set_feature_entries(
        &build_fixture_deny_toml("service"),
        entries,
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::info(
                "extra feature ban",
                "`deny.toml` has extra feature-ban entry for `axum`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "extra feature ban",
                "`deny.toml` has extra feature-ban entry for `serde`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
