use super::check;

#[test]
fn inventories_modular_directory_when_present() {
    let mut results = Vec::new();
    check(true, &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "pre-commit.d directory exists");
}

#[test]
fn inventories_monolithic_mode_when_missing() {
    let mut results = Vec::new();
    check(false, &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "pre-commit.d directory missing");
}
