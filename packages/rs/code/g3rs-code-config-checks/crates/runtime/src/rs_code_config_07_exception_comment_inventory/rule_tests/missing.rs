use super::helpers::{run_check, text_file};

#[test]
fn stays_clean_without_exception_comments() {
    let results = run_check(vec![text_file("deny.toml", "# note only\nvalue = 1\n")]);
    assert!(results.is_empty(), "unexpected results: {results:#?}");
}
