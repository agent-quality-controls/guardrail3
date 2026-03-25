use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    collected_facts, same_root_conflict_input, same_root_conflict_tree,
};
use super::super::check_same_root_conflict;

#[test]
fn errors_on_same_root_precedence_conflict() {
    let facts = collected_facts(&same_root_conflict_tree());
    let input = same_root_conflict_input(&facts, "");
    let mut results = Vec::new();

    check_same_root_conflict(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-03");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "multiple deny configs at one policy root");
    assert_eq!(result.file.as_deref(), Some(".cargo/deny.toml"));
    assert_eq!(
        result.message,
        "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml."
    );
}
