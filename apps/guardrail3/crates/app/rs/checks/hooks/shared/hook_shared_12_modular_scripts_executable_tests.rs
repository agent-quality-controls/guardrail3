use super::check;

#[test]
fn returns_no_results_for_empty_inventory() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assert!(results.is_empty());
}

#[test]
fn inventories_executable_script_and_flags_non_executable_script() {
    let mut results = Vec::new();
    check(
        &[
            (".githooks/pre-commit.d/10-rust.sh".to_owned(), true),
            (".githooks/pre-commit.d/20-ts.sh".to_owned(), false),
        ],
        &mut results,
    );
    assert_eq!(results.len(), 2);
    assert!(results[0].inventory);
    assert!(!results[1].inventory);
}
