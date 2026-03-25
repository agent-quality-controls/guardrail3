use super::check;

#[test]
fn inventories_clean_trust_state() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assert!(results[0].inventory);
}

#[test]
fn warns_when_competing_hook_systems_exist() {
    let mut results = Vec::new();
    check(
        &[
            ".husky/pre-commit".to_owned(),
            ".git/hooks/pre-commit".to_owned(),
        ],
        &mut results,
    );
    assert!(!results[0].inventory);
    assert!(results[0].message.contains(".husky/pre-commit"));
}
