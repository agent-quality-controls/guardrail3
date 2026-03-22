use crate::domain::report::Severity;

use super::super::test_support::{
    collected_facts, forbidden_input, nested_member_shadow_tree, same_root_conflict_input,
    same_root_conflict_tree,
};
use super::{check_forbidden, check_same_root_conflict};

#[test]
fn errors_on_nested_shadowing() {
    let facts = collected_facts(&nested_member_shadow_tree("deny.toml"));
    let input = forbidden_input(&facts, "workspace/crates/core/deny.toml");
    let mut results = Vec::new();

    check_forbidden(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-03");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "nested deny config shadows parent policy");
    assert_eq!(result.file.as_deref(), Some("workspace/crates/core/deny.toml"));
    assert_eq!(
        result.message,
        "`workspace/crates/core/deny.toml` shadows deny policy rooted at `workspace`."
    );
}

#[test]
fn errors_on_same_root_conflict() {
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
