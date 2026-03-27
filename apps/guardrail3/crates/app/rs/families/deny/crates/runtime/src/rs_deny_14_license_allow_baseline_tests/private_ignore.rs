use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_private_ignore};

#[test]
fn errors_when_licenses_private_ignore_is_not_true() {
    let results = super::super::run_check(&set_private_ignore(&build_fixture_deny_toml("service"), false));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-14");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "licenses.private.ignore must be true");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[licenses.private].ignore = true`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
