use super::helpers::check_source;

#[test]
fn skips_test_files() {
    let content = (0..600)
        .map(|i| format!("fn f{i}() {{}}\n"))
        .collect::<String>();

    let results = check_source("tests/smoke.rs", &content, true);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn ignores_comment_only_lines() {
    let content = (0..500).map(|_| "// comment\n").collect::<String>();

    let results = check_source("src/lib.rs", &content, false);

    assert!(results.is_empty(), "{results:#?}");
}
