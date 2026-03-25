use guardrail3_domain_report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{
    canonical_deny_toml_service, config_facts, set_deny_ban_wrappers,
};
use super::super::check;

#[test]
fn errors_when_canonical_managed_wrappers_change() {
    let config = config_facts(&set_deny_ban_wrappers(
        &canonical_deny_toml_service(),
        "regex",
        &["tree-sitter"],
    ));
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "managed ban wrappers changed");
    assert_eq!(
        result.message,
        "`deny.toml` ban `regex` no longer matches the canonical managed entry."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}
