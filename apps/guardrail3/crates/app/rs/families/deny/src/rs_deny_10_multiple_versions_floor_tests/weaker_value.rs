use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_section_string,
};
use super::super::check;

#[test]
fn warns_when_multiple_versions_is_weaker_than_baseline() {
    let config = config_facts(&set_section_string(
        &canonical_deny_toml_service(),
        "bans",
        "multiple-versions",
        "warn",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-10");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "multiple-versions weaker than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `[bans].multiple-versions = \"warn\"`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
