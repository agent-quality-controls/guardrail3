use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn warns_on_copyleft_license() {
    let config = config_facts(&canonical_deny_toml_service().replace(
        "\"MIT\", ",
        "\"MIT\", \"GPL-3.0\", ",
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-16"
            && result.severity == Severity::Warn
            && result.title == "copyleft license allowed"
            && result.message == "`deny.toml` allows copyleft license `GPL-3.0`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
