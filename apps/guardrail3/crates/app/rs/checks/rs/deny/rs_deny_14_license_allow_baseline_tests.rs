use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_when_license_baseline_is_missing() {
    let config = config_facts(&canonical_deny_toml_service().replace("\"MIT\", ", ""));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-14"
            && result.severity == Severity::Error
            && result.title == "baseline license missing"
            && result.message == "`deny.toml` is missing allowed license `MIT`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
