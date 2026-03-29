use guardrail3_domain_report::Severity;

use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{
    build_fixture_deny_toml, config_facts, remove_section_key, set_license_confidence_threshold,
};

#[test]
fn warns_when_confidence_threshold_is_missing_or_invalid() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "licenses",
        "confidence-threshold",
    ));
    let invalid = config_facts(&set_license_confidence_threshold(
        &build_fixture_deny_toml("service"),
        toml::Value::String("high".to_owned()),
    ));

    for config in [&missing, &invalid] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.id, "RS-DENY-15");
        assert_eq!(result.severity, Severity::Warn);
        assert_eq!(result.title, "confidence-threshold missing or invalid");
        assert_eq!(
            result.message,
            "`deny.toml` must set `confidence-threshold >= 0.8`."
        );
        assert_eq!(result.file.as_deref(), Some("deny.toml"));
        assert!(!result.inventory);
    }
}
