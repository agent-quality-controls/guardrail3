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

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-07"
            && result.severity == Severity::Error
            && result.title == "graph all-features must be true"
            && result.message == "`deny.toml` must set `[graph].all-features = true`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
