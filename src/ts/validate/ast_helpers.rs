//! Tree-sitter-based TypeScript/TSX source analysis helpers.
//!
//! These functions parse TypeScript source into a tree-sitter CST and inspect it
//! structurally — no grep, no false positives from strings or comments.
//!
//! Code analysis helpers (process.env, any types, test method calls) are in
//! [`super::ts_code_analysis`] and re-exported here for backward compatibility.

use tree_sitter::{Parser, Tree};

// Re-export code analysis functions so existing callers don't break.
pub use super::ts_code_analysis::{find_any_types, find_process_env, find_test_method_calls};

/// A comment found in the source: 1-based line number + raw text.
pub struct CommentInfo {
    pub line: usize,
    pub text: String,
}

/// Parse TypeScript source. Returns `None` if parsing fails.
pub fn parse_typescript(source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .ok()?;
    parser.parse(source, None)
}

/// Parse TSX source. Returns `None` if parsing fails.
pub fn parse_tsx(source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
        .ok()?;
    parser.parse(source, None)
}

/// Parse a TypeScript or TSX file (chosen by extension hint).
pub fn parse_ts_file(source: &str, is_tsx: bool) -> Option<Tree> {
    if is_tsx { parse_tsx(source) } else { parse_typescript(source) }
}

/// Extract all comment nodes from a parsed tree.
///
/// Returns [`CommentInfo`] entries with 1-based line numbers and raw comment text.
/// Only actual comment nodes are returned — strings and template literals are ignored.
pub fn find_comments(tree: &Tree, source: &str) -> Vec<CommentInfo> {
    let mut out = Vec::new();
    let mut cursor = tree.walk();
    walk_comments(&mut cursor, source.as_bytes(), &mut out);
    out
}

/// Find eslint-disable directives that appear inside actual comments (not strings).
pub fn find_eslint_disables(tree: &Tree, source: &str) -> Vec<CommentInfo> {
    find_comments(tree, source)
        .into_iter()
        .filter(|c| c.text.contains("eslint-disable"))
        .collect()
}

/// Find `@ts-ignore` and `@ts-expect-error` directives inside comments.
pub fn find_ts_directives(tree: &Tree, source: &str) -> Vec<CommentInfo> {
    find_comments(tree, source)
        .into_iter()
        .filter(|c| c.text.contains("@ts-ignore") || c.text.contains("@ts-expect-error"))
        .collect()
}

// ---------------------------------------------------------------------------
// Internal — recursive tree walkers
// ---------------------------------------------------------------------------

/// Recursive tree walk collecting only `comment` nodes.
fn walk_comments(
    cursor: &mut tree_sitter::TreeCursor<'_>,
    source: &[u8],
    out: &mut Vec<CommentInfo>,
) {
    loop {
        let node = cursor.node();
        if node.kind() == "comment" {
            let text = node_text(&node, source).to_owned();
            let line = node.start_position().row.saturating_add(1);
            out.push(CommentInfo { line, text });
        }

        // Recurse into children
        if cursor.goto_first_child() {
            walk_comments(cursor, source, out);
            let _moved = cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn node_text<'a>(node: &tree_sitter::Node<'_>, source: &'a [u8]) -> &'a str {
    let start = node.start_byte();
    let end = node.end_byte();
    source
        .get(start..end)
        .and_then(|b| std::str::from_utf8(b).ok())
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must_parse(source: &str) -> Tree {
        #[allow(clippy::expect_used)] // reason: test helper — panic on bad input is correct
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
}
