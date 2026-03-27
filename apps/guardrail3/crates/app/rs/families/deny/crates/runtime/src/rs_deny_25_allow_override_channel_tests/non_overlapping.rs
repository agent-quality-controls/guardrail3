use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_bans_allow_entries};

#[test]
fn errors_on_non_empty_allow_list_even_when_it_does_not_override_a_ban() {
    let allow = vec![toml::Value::String("totally-custom-crate".to_owned())];
    let results = super::super::run_check(&set_bans_allow_entries(
        &build_fixture_deny_toml("service"),
        allow,
    ));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-25");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "bans allow-list present");
    assert_eq!(
        result.message,
        "`deny.toml` has non-empty `[bans].allow`: totally-custom-crate."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
