use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::config_facts;
use super::super::check;

#[test]
fn errors_when_bans_section_is_missing() {
    let config = config_facts("[graph]\nall-features = true\n");
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[bans] section missing");
    assert_eq!(result.message, "`deny.toml` has no `[bans]` section.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

#[test]
fn errors_when_bans_deny_is_missing() {
    let config = config_facts("[bans]\nmultiple-versions = \"deny\"\n");
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[bans].deny missing");
    assert_eq!(result.message, "`deny.toml` must contain `[bans].deny`.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
