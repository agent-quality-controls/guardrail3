use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn inventories_stricter_advisory_settings() {
    let deny = config_facts(
        &canonical_deny_toml_service().replace("yanked = \"warn\"", "yanked = \"deny\""),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-06");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "advisories `yanked` stricter than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `[advisories].yanked = \"deny\"`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(result.inventory);
}
