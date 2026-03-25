use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, remove_allowed_license,
};
use super::super::check;

#[test]
fn errors_when_a_baseline_allowed_license_is_missing() {
    let config = config_facts(&remove_allowed_license(
        &canonical_deny_toml_service(),
        "MIT",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-14");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "baseline license missing");
    assert_eq!(
        result.message,
        "`deny.toml` is missing allowed license `MIT`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
