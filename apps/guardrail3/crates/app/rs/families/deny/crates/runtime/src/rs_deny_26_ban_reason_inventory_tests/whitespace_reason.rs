use guardrail3_domain_report::Severity;

use super::super::add_deny_ban_entry;

#[test]
fn inventories_whitespace_only_reason_as_missing() {
    let deny = add_deny_ban_entry(
        "[bans]\ndeny = []\n",
        toml::Value::Table(toml::map::Map::from_iter([
            ("name".to_owned(), toml::Value::String("json5".to_owned())),
            ("wrappers".to_owned(), toml::Value::Array(Vec::new())),
            ("reason".to_owned(), toml::Value::String("   ".to_owned())),
        ])),
    );
    let results = super::super::run_check(&deny);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-26");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "ban entry missing reason");
    assert_eq!(
        result.message,
        "`deny.toml` ban entry `json5` has no `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
