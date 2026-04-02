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
#[derive(Debug)]
pub struct CommentInfo {
    line: usize,
    text: String,
}

impl CommentInfo {
    #[must_use]
    pub fn new(line: usize, text: String) -> Self {
        Self { line, text }
    }

    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
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
    if is_tsx {
        parse_tsx(source)
    } else {
        parse_typescript(source)
    }
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
            out.push(CommentInfo::new(line, text));
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
