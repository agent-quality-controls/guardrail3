use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_private_ignore,
};
use super::super::check;

#[test]
fn errors_when_licenses_private_ignore_is_not_true() {
    let config = config_facts(&set_private_ignore(&canonical_deny_toml_service(), false));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

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
