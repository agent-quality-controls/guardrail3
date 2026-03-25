use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_bans_allow_entries,
};
use super::super::check;

#[test]
fn errors_on_non_empty_allow_list_and_deny_overrides() {
    let allow = vec![
        toml::Value::String("lazy_static".to_owned()),
        toml::Value::String("json5".to_owned()),
    ];
    let config = config_facts(&set_bans_allow_entries(
        &canonical_deny_toml_service(),
        allow,
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "bans allow-list present"
            && result.message == "`deny.toml` has non-empty `[bans].allow`: json5, lazy_static."
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "allow-list overrides deny-list"
            && result.message == "`deny.toml` allows `json5` even though it is banned."
            && result.file.as_deref() == Some("deny.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-25"
            && result.severity == Severity::Error
            && result.title == "allow-list overrides deny-list"
            && result.message == "`deny.toml` allows `lazy_static` even though it is banned."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
