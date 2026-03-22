use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;
use super::check;

fn bans_toml(entry: &str) -> String {
    format!("[bans]\ndeny = [{entry}]\n")
}

#[test]
fn inventories_named_ban_entries_without_reason() {
    let config = config_facts(&bans_toml(r#"{ name = "lazy_static", wrappers = [] }"#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-26");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "ban entry missing reason");
    assert_eq!(
        result.message,
        "`deny.toml` ban entry `lazy_static` has no `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}

#[test]
fn inventories_plain_string_ban_entries_without_reason() {
    let config = config_facts(&bans_toml(r#""lazy_static""#));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-26");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "ban entry missing reason");
    assert_eq!(
        result.message,
        "`deny.toml` ban entry `lazy_static` has no `reason`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
