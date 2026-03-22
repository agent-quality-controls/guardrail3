use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn warns_on_copyleft_license() {
    let config =
        config_facts(&canonical_deny_toml_service().replace("\"MIT\", ", "\"MIT\", \"GPL-3.0\", "));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-16");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "copyleft license allowed");
    assert_eq!(
        result.message,
        "`deny.toml` allows copyleft license `GPL-3.0`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
