use crate::domain::report::Severity;

use super::super::test_support::{collected_facts, forbidden_input, nested_member_shadow_tree};
use super::check;

#[test]
fn errors_on_forbidden_deny_location() {
    let facts = collected_facts(&nested_member_shadow_tree("deny.toml"));
    let input = forbidden_input(&facts, "workspace/crates/core/deny.toml");
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "deny config at forbidden location");
    assert_eq!(result.file.as_deref(), Some("workspace/crates/core/deny.toml"));
    assert_eq!(
        result.message,
        "`workspace/crates/core/deny.toml` (deny.toml) is at `workspace/crates/core` which is not an allowed deny policy root."
    );
}
