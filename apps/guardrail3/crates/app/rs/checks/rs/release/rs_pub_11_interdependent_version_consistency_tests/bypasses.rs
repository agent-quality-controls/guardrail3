use super::super::super::test_support::{edge_facts, edge_input};
use super::super::check;

#[test]
fn does_not_error_when_local_publishable_version_is_compatible() {
    let facts = edge_facts();
    let input = edge_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

#[test]
fn does_not_error_for_non_publishable_or_non_path_edges() {
    let mut non_publishable = edge_facts();
    non_publishable.dep_publishable = false;
    let non_publishable_input = edge_input(&non_publishable);
    let mut non_publishable_results = Vec::new();

    check(&non_publishable_input, &mut non_publishable_results);

    assert!(non_publishable_results.is_empty());

    let mut non_path = edge_facts();
    non_path.has_path = false;
    non_path.version_satisfied = Some(false);
    let non_path_input = edge_input(&non_path);
    let mut non_path_results = Vec::new();

    check(&non_path_input, &mut non_path_results);

    assert!(non_path_results.is_empty());
}
