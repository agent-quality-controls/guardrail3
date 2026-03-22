use crate::domain::report::Severity;

use super::super::inputs::ConfigDenyInput;
use super::super::test_support::{canonical_deny_toml_service, config_facts};

#[test]
fn errors_when_no_default_features_is_true() {
    let deny = config_facts(
        &canonical_deny_toml_service()
            .replace("no-default-features = false", "no-default-features = true"),
    );
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-08");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "graph no-default-features must be false");
    assert_eq!(
        result.message,
        "`deny.toml` must set `[graph].no-default-features = false`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}

#[test]
fn errors_when_graph_section_is_missing() {
    let deny = config_facts(&canonical_deny_toml_service().replace("[graph]\n", "[removed]\n"));
    let input = ConfigDenyInput { config: &deny };
    let mut results = Vec::new();

    super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-08");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "[graph] section missing");
    assert_eq!(
        result.message,
        "`deny.toml` must contain `[graph]` coverage settings."
    );
}
