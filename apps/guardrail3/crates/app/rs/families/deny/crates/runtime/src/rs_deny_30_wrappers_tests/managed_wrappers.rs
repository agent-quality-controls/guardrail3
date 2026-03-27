use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_deny_ban_wrappers};

#[test]
fn errors_when_canonical_non_empty_wrapper_policy_changes() {
    let results = super::super::run_check(&set_deny_ban_wrappers(
        &build_fixture_deny_toml("service"),
        "regex",
        &["tree-sitter"],
    ));

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-30");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "managed ban wrappers changed");
    assert_eq!(
        result.message,
        "`deny.toml` ban `regex` must keep wrappers `globset, ignore, tree-sitter`."
    );
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
    assert!(!result.inventory);
}
