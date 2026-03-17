use guardrail3::app::rs::validate::source_scan::{filter_non_comment_lines, strip_string_literals};

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn filter_preserves_lines_with_string_containing_block_comment() {
    let input = r#"let x = remaining.find("/*");"#;
    let result = filter_non_comment_lines(input);
    assert_eq!(
        result.len(),
        1,
        "Line with /* in string should not be dropped"
    );
    // Assert FULL line preserved — a partial match like contains("remaining.find")
    // would pass even if the line was truncated at the string-embedded "/*"
    assert_eq!(
        result[0].1, r#"let x = remaining.find("/*");"#,
        "Full original line must be preserved exactly including the string literal"
    );
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn filter_removes_actual_block_comments() {
    let input = "code\n/* comment */\nmore code";
    let result = filter_non_comment_lines(input);
    assert_eq!(result.len(), 2);
    assert!(result[0].1.contains("code"));
    assert!(result[1].1.contains("more code"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn filter_handles_multiline_block_comment() {
    let input = "before\n/* start\nmiddle\nend */\nafter";
    let result = filter_non_comment_lines(input);
    assert_eq!(result.len(), 2);
    assert!(result[0].1.contains("before"));
    assert!(result[1].1.contains("after"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn filter_handles_block_comment_end_with_code() {
    let input = "/* comment */ let x = 1;";
    let result = filter_non_comment_lines(input);
    assert_eq!(result.len(), 1);
    assert!(result[0].1.contains("let x = 1"));
}

#[test]
#[allow(clippy::indexing_slicing)] // reason: test assertion indexes into results
fn filter_skips_line_comments() {
    let input = "code\n// comment\nmore";
    let result = filter_non_comment_lines(input);
    assert_eq!(result.len(), 2);
    assert!(result[0].1.contains("code"));
    assert!(result[1].1.contains("more"));
}

#[test]
fn filter_empty_input() {
    let result = filter_non_comment_lines("");
    assert!(result.is_empty());
}

#[test]
fn filter_only_comments() {
    let input = "// line comment\n/* block */\n/// doc comment";
    let result = filter_non_comment_lines(input);
    assert!(result.is_empty(), "Only comments should produce no output");
}

#[test]
fn filter_only_code() {
    let input = "let x = 1;\nlet y = 2;";
    let result = filter_non_comment_lines(input);
    assert_eq!(result.len(), 2);
}

#[test]
fn filter_nested_block_comment_delimiters_in_string() {
    let input = r#"let re = Regex::new("/* unterminated */");"#;
    let result = filter_non_comment_lines(input);
    assert_eq!(
        result.len(),
        1,
        "/* */ inside string should not affect comment parsing"
    );
}

#[test]
fn strip_string_literals_removes_content() {
    let result = strip_string_literals(r#"let x = "hello /* world */";"#);
    assert!(!result.contains("hello"));
    assert!(
        !result.contains("/*"),
        "Comment delimiters inside string should be stripped"
    );
}

#[test]
fn strip_string_literals_handles_escaped_quotes() {
    let result = strip_string_literals(r#"let x = "she said \"hi\"";"#);
    assert!(!result.contains("she"));
}

#[test]
fn strip_string_literals_preserves_non_string_content() {
    let result = strip_string_literals("let x = 42; /* comment */");
    assert!(result.contains("let x = 42"));
    assert!(result.contains("/* comment */"));
}

#[test]
fn strip_string_literals_handles_raw_strings() {
    // Raw string r#"..."# should have its content stripped
    let result = strip_string_literals(r##"let x = r#"hello /* world */"#;"##);
    assert!(
        !result.contains("hello"),
        "Raw string content should be stripped, got: {result}"
    );
    assert!(
        !result.contains("/*"),
        "Comment delimiters inside raw string should be stripped, got: {result}"
    );
    assert!(
        result.contains("let x = "),
        "Code outside raw string should be preserved, got: {result}"
    );

    // Simple raw string r"..." should also work
    let result2 = strip_string_literals(r#"let y = r"some content";"#);
    assert!(
        !result2.contains("some content"),
        "Simple raw string content should be stripped, got: {result2}"
    );
}

#[test]
fn strip_string_literals_empty_string() {
    let result = strip_string_literals(r#"let x = "";"#);
    assert!(result.contains("let x = "));
}
