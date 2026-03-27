use guardrail3_domain_report::Severity;

use super::super::{build_fixture_deny_toml, set_deny_ban_wrappers};

#[test]
fn errors_when_canonical_managed_wrappers_change() {
    let results = super::super::run_check(&set_deny_ban_wrappers(
        &build_fixture_deny_toml("service"),
        "regex",
        &["tree-sitter"],
    ));

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
