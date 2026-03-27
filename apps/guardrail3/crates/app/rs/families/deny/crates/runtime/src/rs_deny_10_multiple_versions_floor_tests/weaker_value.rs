use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_section_string};

#[test]
fn warns_when_multiple_versions_is_weaker_than_baseline() {
    let results = super::super::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "multiple-versions",
        "warn",
    ));

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
