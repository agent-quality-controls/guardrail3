//! Tests extracted from `app::ts::validate::ast_helpers`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use guardrail3::app::ts::validate::ast_helpers::{
    find_comments, find_eslint_disables, find_ts_directives, parse_tsx, parse_typescript,
};
use tree_sitter::Tree;

fn must_parse(source: &str) -> Tree {
    parse_typescript(source).expect("test input should be valid TypeScript")
}

// -----------------------------------------------------------------------
// parse_typescript / parse_tsx
// -----------------------------------------------------------------------

#[test]
fn parse_valid_typescript() {
    assert!(
        parse_typescript("const x: number = 42;").is_some(),
        "should parse valid TypeScript"
    );
}

#[test]
fn parse_invalid_still_returns_tree() {
    assert!(
        parse_typescript("const = = = ;;;").is_some(),
        "tree-sitter should still produce a tree for invalid source"
    );
}

#[test]
fn parse_tsx_with_jsx() {
    assert!(
        parse_tsx("const el = <div>hello</div>;").is_some(),
        "should parse TSX"
    );
}

// -----------------------------------------------------------------------
// find_comments
// -----------------------------------------------------------------------

#[test]
fn finds_line_comment() {
    let src = "// hello world\nconst x = 1;";
    let tree = must_parse(src);
    let comments = find_comments(&tree, src);
    assert_eq!(comments.len(), 1, "should find one comment");
    assert_eq!(
        comments.first().map(|c| c.line),
        Some(1),
        "comment is on line 1"
    );
    assert!(
        comments
            .first()
            .is_some_and(|c| c.text.contains("hello world")),
        "comment text should contain 'hello world'"
    );
}

#[test]
fn finds_block_comment() {
    let src = "/* block */\nconst x = 1;";
    let tree = must_parse(src);
    let comments = find_comments(&tree, src);
    assert_eq!(comments.len(), 1, "should find one block comment");
    assert!(
        comments.first().is_some_and(|c| c.text.contains("block")),
        "comment text should contain 'block'"
    );
}

#[test]
fn string_literal_not_a_comment() {
    let src = "const s = \"// not a comment\";\nexport default s;";
    let tree = must_parse(src);
    let comments = find_comments(&tree, src);
    assert!(
        comments.is_empty(),
        "string containing // should not be found as comment"
    );
}

#[test]
fn template_literal_not_a_comment() {
    let src = "const s = `// not a comment`;\nexport default s;";
    let tree = must_parse(src);
    let comments = find_comments(&tree, src);
    assert!(
        comments.is_empty(),
        "template literal containing // should not be found as comment"
    );
}

// -----------------------------------------------------------------------
// find_eslint_disables
// -----------------------------------------------------------------------

#[test]
fn eslint_disable_in_line_comment() {
    let src = "// eslint-disable-next-line no-console\nconsole.log('hi');";
    let tree = must_parse(src);
    let disables = find_eslint_disables(&tree, src);
    assert_eq!(disables.len(), 1, "should find one eslint-disable");
    assert_eq!(
        disables.first().map(|c| c.line),
        Some(1),
        "should be on line 1"
    );
}

#[test]
fn eslint_disable_in_block_comment() {
    let src = "/* eslint-disable @typescript-eslint/no-explicit-any */\nconst x: any = 1;";
    let tree = must_parse(src);
    let disables = find_eslint_disables(&tree, src);
    assert_eq!(
        disables.len(),
        1,
        "should find block-comment eslint-disable"
    );
}

#[test]
fn eslint_disable_in_string_not_found() {
    let src = "const s = \"eslint-disable-next-line\";\nexport default s;";
    let tree = must_parse(src);
    let disables = find_eslint_disables(&tree, src);
    assert!(
        disables.is_empty(),
        "eslint-disable inside string literal should not be detected"
    );
}

#[test]
fn eslint_disable_in_template_not_found() {
    let src = "const s = `eslint-disable`;\nexport default s;";
    let tree = must_parse(src);
    let disables = find_eslint_disables(&tree, src);
    assert!(
        disables.is_empty(),
        "eslint-disable inside template string should not be detected"
    );
}

// -----------------------------------------------------------------------
// find_ts_directives
// -----------------------------------------------------------------------

#[test]
fn ts_ignore_in_comment() {
    let src = "// @ts-ignore\nconst x: any = 1;";
    let tree = must_parse(src);
    let directives = find_ts_directives(&tree, src);
    assert_eq!(directives.len(), 1, "should find @ts-ignore");
    assert!(
        directives
            .first()
            .is_some_and(|c| c.text.contains("@ts-ignore")),
        "should contain directive text"
    );
}

#[test]
fn ts_expect_error_in_comment() {
    let src = "// @ts-expect-error intentional\nconst x = 1;";
    let tree = must_parse(src);
    let directives = find_ts_directives(&tree, src);
    assert_eq!(directives.len(), 1, "should find @ts-expect-error");
}

#[test]
fn ts_ignore_in_string_not_found() {
    let src = "const s = \"@ts-ignore is a directive\";\nexport default s;";
    let tree = must_parse(src);
    let directives = find_ts_directives(&tree, src);
    assert!(
        directives.is_empty(),
        "@ts-ignore inside string literal should not be detected"
    );
}

#[test]
fn ts_expect_error_in_template_not_found() {
    let src = "const s = `@ts-expect-error`;\nexport default s;";
    let tree = must_parse(src);
    let directives = find_ts_directives(&tree, src);
    assert!(
        directives.is_empty(),
        "@ts-expect-error inside template string should not be detected"
    );
}
