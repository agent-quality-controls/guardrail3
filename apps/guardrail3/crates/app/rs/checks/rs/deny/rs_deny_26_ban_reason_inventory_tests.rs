use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn inventories_named_ban_entries_without_reason() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "{ name = \"lazy_static\", wrappers = [] },",
        "{ name = \"lazy_static\", wrappers = [] },",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-26"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "ban entry missing reason"
            && result.message == "`deny.toml` ban entry `lazy_static` has no `reason`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}

#[test]
fn inventories_plain_string_ban_entries_without_reason() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "{ name = \"lazy_static\", wrappers = [] },",
        "\"lazy_static\",",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-26"
            && result.inventory
            && result.severity == Severity::Info
            && result.message == "`deny.toml` ban entry `lazy_static` has no `reason`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
