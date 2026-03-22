use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_when_confidence_threshold_is_weaker() {
    let deny = config_facts(
        &canonical_deny_toml_service()
            .replace("confidence-threshold = 0.8", "confidence-threshold = 0.7"),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-15");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "confidence-threshold weaker than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `confidence-threshold = 0.7`."
    );
}

#[test]
fn inventories_when_confidence_threshold_is_stricter() {
    let deny = config_facts(
        &canonical_deny_toml_service()
            .replace("confidence-threshold = 0.8", "confidence-threshold = 0.9"),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-15");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "confidence-threshold stricter than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `confidence-threshold = 0.9`."
    );
    assert!(result.inventory);
}

#[test]
fn warns_when_confidence_threshold_is_missing() {
    let deny =
        config_facts(&canonical_deny_toml_service().replace("confidence-threshold = 0.8\n", ""));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-15");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "confidence-threshold missing or invalid");
    assert_eq!(
        result.message,
        "`deny.toml` must set `confidence-threshold >= 0.8`."
    );
}
