use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    add_deny_ban_entry, add_skip_entry, canonical_deny_toml_service, config_facts,
    set_advisory_ignores, set_feature_entries,
};
use super::super::check;

fn deny_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn skip_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "version".to_owned(),
            toml::Value::String("1.0.0".to_owned()),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

fn feature_entry(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(vec![toml::Value::String("full".to_owned())]),
        ),
        (
            "allow".to_owned(),
            toml::Value::Array(vec![toml::Value::String("fs".to_owned())]),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

#[test]
fn warns_once_per_duplicated_entry_family() {
    let with_bans = add_deny_ban_entry(
        &add_deny_ban_entry(&canonical_deny_toml_service(), deny_entry("json5")),
        deny_entry("json5"),
    );
    let with_skip = add_skip_entry(
        &add_skip_entry(&with_bans, skip_entry("demo")),
        skip_entry("demo"),
    );
    let with_ignores = set_advisory_ignores(
        &with_skip,
        vec![
            toml::Value::String("RUSTSEC-2020-0001".to_owned()),
            toml::Value::String("RUSTSEC-2020-0001".to_owned()),
        ],
    );
    let deny = set_feature_entries(
        &with_ignores,
        vec![feature_entry("tokio"), feature_entry("tokio")],
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 4);
    let titles = results.iter().map(|r| r.title.as_str()).collect::<Vec<_>>();
    assert_eq!(
        titles,
        vec![
            "duplicate deny entry",
            "duplicate skip entry",
            "duplicate advisory ignore entry",
            "duplicate feature-ban entry",
        ]
    );
    for result in &results {
        assert_eq!(result.id, "RS-DENY-27");
        assert_eq!(result.severity, Severity::Warn);
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
