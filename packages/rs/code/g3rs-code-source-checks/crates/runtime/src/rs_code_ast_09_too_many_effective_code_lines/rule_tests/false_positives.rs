
#[test]
fn skips_test_files() {
    let content = (0..600)
        .map(|i| format!("fn f{i}() {{}}\n"))
        .collect::<String>();

    let results = super::super::check_source("tests/smoke.rs", &content, true);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn ignores_comment_only_lines() {
    let content = (0..500).map(|_| "// comment\n").collect::<String>();

    let results = super::super::check_source("src/lib.rs", &content, false);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn ignores_raw_string_payload_only_lines() {
    let payload = (0..600)
        .map(|i| format!("payload-{i}\n"))
        .collect::<String>();
    let content = format!("const BIG: &str = r#\"\n{payload}\"#;\n");

    let results = super::super::check_source("src/lib.rs", &content, false);

    assert!(results.is_empty(), "{results:#?}");
}
