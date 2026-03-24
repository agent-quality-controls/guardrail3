use super::check;

#[test]
fn reports_line_and_byte_counts() {
    let mut results = Vec::new();
    check(".githooks/pre-commit", "line1\nline2\n", &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].message, "2 lines, 12 bytes");
}
