use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{add_deny_ban_entry, config_facts};
use super::super::check;

fn deny_entry_without_reason(name: &str) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter([
        ("name".to_owned(), toml::Value::String(name.to_owned())),
        ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
    ]))
}

#[test]
fn inventories_each_ban_entry_without_reason() {
    let deny = add_deny_ban_entry(
        &add_deny_ban_entry(
            "[bans]\ndeny = []\n",
            deny_entry_without_reason("lazy_static"),
        ),
        deny_entry_without_reason("json5"),
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
            "`deny.toml` ban entry `lazy_static` has no `reason`.",
            "`deny.toml` ban entry `json5` has no `reason`.",
        ]
    );
    for result in &results {
        assert_eq!(result.id, "RS-DENY-26");
        assert_eq!(result.severity, Severity::Info);
        assert_eq!(result.title, "ban entry missing reason");
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(result.inventory);
    }
}
