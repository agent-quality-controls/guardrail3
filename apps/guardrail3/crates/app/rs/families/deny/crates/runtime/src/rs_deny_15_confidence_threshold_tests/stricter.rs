use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_license_confidence_threshold};

#[test]
fn inventories_when_confidence_threshold_is_stricter() {
    let results = super::super::run_check(&set_license_confidence_threshold(
        &build_fixture_deny_toml("service"),
        toml::Value::Float(0.9),
    ));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-15");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "confidence-threshold stricter than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `confidence-threshold = 0.9`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
