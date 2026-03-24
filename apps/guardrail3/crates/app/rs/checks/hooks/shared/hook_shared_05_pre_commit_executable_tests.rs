use super::check;

#[test]
fn inventories_executable_dispatcher() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", Some(true), &mut results);
    assert!(results[0].inventory);
}

#[test]
fn errors_when_dispatcher_not_executable() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", Some(false), &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn errors_when_permissions_unavailable() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", None, &mut results);
    assert_eq!(results[0].title, "pre-commit hook permissions unavailable");
    assert!(!results[0].inventory);
}
