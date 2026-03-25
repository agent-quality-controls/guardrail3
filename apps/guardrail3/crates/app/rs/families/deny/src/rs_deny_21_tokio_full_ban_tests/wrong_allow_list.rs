use guardrail3_domain_report::Severity;

use super::super::super::deny_support::{expected_tokio_allowed_features, join_set};
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
fn warns_when_tokio_allow_list_drifts() {
    let deny = set_feature_entries(
        &canonical_deny_toml_service(),
        vec![tokio_entry(&["full"], &["rt-multi-thread"])],
    );
    let config = config_facts(&deny);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-21");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "tokio allowed features changed");
    assert_eq!(
        result.message,
        format!(
            "`deny.toml` must keep `tokio` allowed features `{}`.",
            join_set(&expected_tokio_allowed_features())
        )
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
