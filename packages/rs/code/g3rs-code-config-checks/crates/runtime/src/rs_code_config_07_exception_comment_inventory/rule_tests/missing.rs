use super::helpers::run_check;

#[test]
fn stays_clean_without_exception_comments() {
    let results = run_check(Vec::new());
    assert!(results.is_empty(), "unexpected results: {results:#?}");
}
