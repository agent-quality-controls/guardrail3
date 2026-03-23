use crate::domain::report::Severity;

use super::super::test_support::{edge_facts, edge_input};
use super::check;

#[test]
fn errors_on_target_specific_path_dep_to_unpublishable_crate() {
    let mut facts = edge_facts();
    facts.dep_publishable = false;
    facts.target_label = Some("cfg(unix)".to_owned());
    let input = edge_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}
