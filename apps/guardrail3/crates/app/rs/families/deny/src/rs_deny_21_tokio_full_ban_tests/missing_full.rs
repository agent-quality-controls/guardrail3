use guardrail3_domain_report::Severity;

use super::super::super::deny_support::expected_tokio_allowed_features;
use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_feature_entries,
};
use super::super::check;

fn tokio_entry(deny: &[&str], allow: &[&str]) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String("tokio".to_owned())),
        (
            "deny".to_owned(),
            toml::Value::Array(
                deny.iter()
                    .map(|v| toml::Value::String((*v).to_owned()))
                    .collect(),
            ),
        ),
        (
            "allow".to_owned(),
            toml::Value::Array(
                allow
                    .iter()
                    .map(|v| toml::Value::String((*v).to_owned()))
                    .collect(),
            ),
        ),
        (
            "reason".to_owned(),
            toml::Value::String("good enough reason text".to_owned()),
        ),
    ]))
}

#[test]
fn warns_when_tokio_full_is_not_banned() {
    let expected_allow = expected_tokio_allowed_features()
        .into_iter()
        .collect::<Vec<_>>();
    let allow_refs = expected_allow
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let deny = set_feature_entries(
        &canonical_deny_toml_service(),
        vec![tokio_entry(&[], &allow_refs)],
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-21");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "tokio full feature not banned");
    assert_eq!(
        result.message,
        "`deny.toml` must ban `tokio` feature `full` under `[[bans.features]]`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
