use crate::domain::report::Severity;

use super::super::test_support::{edge_facts, edge_input};
use super::check;

#[test]
fn errors_on_incompatible_local_version_req() {
    let mut facts = edge_facts();
    facts.version_req = Some("^2.0.0".to_owned());
    facts.actual_version = Some("1.2.3".to_owned());
    facts.version_satisfied = Some(false);
    let input = edge_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}
