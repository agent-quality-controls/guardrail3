use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_license_exceptions,
};
use super::super::check;

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
        &canonical_deny_toml_service(),
        vec![
            exception_entry("name", "demo"),
            exception_entry("crate", "demo-legacy"),
        ],
    );
    let config = config_facts(&deny);
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
            "`deny.toml` has license exception for `demo`.",
            "`deny.toml` has license exception for `demo-legacy`.",
        ]
    );

    for result in &results {
        assert_eq!(result.id, "RS-DENY-17");
        assert_eq!(result.severity, Severity::Info);
        assert_eq!(result.title, "license exception entry");
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(result.inventory);
    }
}
