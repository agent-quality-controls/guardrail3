use super::check;

#[test]
fn errors_when_hooks_path_missing() {
    let mut results = Vec::new();
    check(None, &mut results);
    assert!(!results[0].inventory);
}

#[test]
fn errors_when_hooks_path_is_wrong() {
    let mut results = Vec::new();
    check(Some(".git/hooks"), &mut results);
    assert_eq!(results[0].title, "core.hooksPath has wrong value");
    assert!(!results[0].inventory);
}

#[test]
fn inventories_expected_hooks_path() {
    let mut results = Vec::new();
    check(Some(".githooks"), &mut results);
    assert!(results[0].inventory);
}
