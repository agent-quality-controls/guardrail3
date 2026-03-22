use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn warns_when_wildcards_differs() {
    let deny = config_facts(
        &canonical_deny_toml_service().replace("wildcards = \"allow\"", "wildcards = \"deny\""),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-13");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "wildcards differs from baseline");
    assert_eq!(
        result.message,
        "`deny.toml` sets `[bans].wildcards = deny`."
    );
    assert!(!result.inventory);
}
