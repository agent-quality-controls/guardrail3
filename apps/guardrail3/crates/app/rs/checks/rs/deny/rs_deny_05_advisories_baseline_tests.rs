use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::check;

#[test]
fn errors_when_advisories_baseline_is_weakened() {
    let config = config_facts(
        &canonical_deny_toml_service().replace("yanked = \"warn\"", "yanked = \"allow\""),
    );
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-05");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "advisories `yanked` has wrong value");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[advisories].yanked = \"warn\"`, found `allow`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
