use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, remove_allowed_license};

#[test]
fn errors_when_a_baseline_allowed_license_is_missing() {
    let results = super::super::run_check(&remove_allowed_license(
        &build_fixture_deny_toml("service"),
        "MIT",
    ));

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
