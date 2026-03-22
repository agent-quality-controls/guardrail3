use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_when_all_features_missing() {
    let config = config_facts(&canonical_deny_toml_service().replace("all-features = true\n", ""));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-07");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "graph all-features must be true");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[graph].all-features = true`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
