use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_feature_entries,
};
use super::super::check;

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
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
    let existing = parsed
        .get("bans")
        .and_then(|b| b.get("features"))
        .and_then(toml::Value::as_array)
        .cloned()
        .expect("feature entries");
    let mut entries = existing;
    entries.push(feature_entry("serde"));
    entries.push(feature_entry("axum"));
    let config = config_facts(&set_feature_entries(
        &canonical_deny_toml_service(),
        entries,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    let messages = results
        .iter()
        .map(|r| r.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        vec![
            "`deny.toml` has extra feature-ban entry for `serde`.",
            "`deny.toml` has extra feature-ban entry for `axum`.",
        ]
    );
    for result in &results {
        assert_eq!(result.id, "RS-DENY-22");
        assert_eq!(result.severity, Severity::Info);
        assert_eq!(result.title, "extra feature ban");
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(result.inventory);
    }
}
