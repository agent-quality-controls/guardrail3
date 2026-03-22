use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::config_facts;

#[test]
fn warns_on_deprecated_advisory_fields() {
    let deny = config_facts(
        &super::super::test_support::canonical_deny_toml_service()
            .replace("[advisories]\n", "[advisories]\nvulnerability = \"deny\"\n"),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-04");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "deprecated advisory field `vulnerability`");
    assert_eq!(
        result.message,
        "`deny.toml` uses deprecated `[advisories].vulnerability`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
