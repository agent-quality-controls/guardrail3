use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, remove_section_key};

#[test]
fn warns_when_multiple_versions_is_missing() {
    let results = super::super::run_check(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "bans",
        "multiple-versions",
    ));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-10");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "multiple-versions missing");
    assert_eq!(
        result.message,
        "`deny.toml` does not set `[bans].multiple-versions`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
