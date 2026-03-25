use super::check;

#[test]
fn reports_pre_commit_file_size() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", "abcd", &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].message, "4 bytes");
}
