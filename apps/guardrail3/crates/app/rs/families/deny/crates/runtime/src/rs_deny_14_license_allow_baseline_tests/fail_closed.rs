use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_licenses_section_is_missing() {
    let results = super::super::run_check(&remove_section(&build_fixture_deny_toml("service"), "licenses"));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-14");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[licenses] section missing");
    assert_eq!(result.message, "`deny.toml` has no `[licenses]` section.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
