use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_when_advisories_baseline_is_weakened() {
    let config =
        config_facts(&canonical_deny_toml_service().replace("yanked = \"warn\"", "yanked = \"allow\""));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-DENY-05"
            && result.severity == Severity::Error
            && result.title == "advisories `yanked` has wrong value"
            && result.message == "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`."
            && result.file.as_deref() == Some("deny.toml")
    }));
}
