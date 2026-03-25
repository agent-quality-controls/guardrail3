use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_license_confidence_threshold,
};
use super::super::check;

#[test]
fn inventories_when_confidence_threshold_is_stricter() {
    let config = config_facts(&set_license_confidence_threshold(
        &canonical_deny_toml_service(),
        toml::Value::Float(0.9),
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

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
