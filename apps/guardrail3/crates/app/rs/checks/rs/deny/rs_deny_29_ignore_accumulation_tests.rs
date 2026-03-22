use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_when_ignore_list_is_large() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [\"A\", \"B\", \"C\", \"D\", \"E\", \"F\"]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-29");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "advisory ignore list is large");
    assert_eq!(
        result.message,
        "`deny.toml` has 6 `[advisories].ignore` entries (threshold: 5)."
    );
}

#[test]
fn does_not_warn_at_threshold() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "ignore = []",
        "ignore = [\"A\", \"B\", \"C\", \"D\", \"E\"]",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert!(results.is_empty());
}
