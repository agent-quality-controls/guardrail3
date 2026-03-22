use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_when_multiple_versions_is_weaker() {
    let deny = config_facts(&canonical_deny_toml_service().replace(
        "multiple-versions = \"deny\"",
        "multiple-versions = \"warn\"",
    ));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-10");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "multiple-versions weaker than baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `[bans].multiple-versions = \"warn\"`."
    );
}

#[test]
fn warns_when_multiple_versions_is_missing() {
    let deny =
        config_facts(&canonical_deny_toml_service().replace("multiple-versions = \"deny\"\n", ""));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-10");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "multiple-versions missing");
    assert_eq!(
        result.message,
        "`deny.toml` does not set `[bans].multiple-versions`."
    );
}
