//! Tree-sitter-based TypeScript/TSX source analysis helpers.
//!
//! These functions parse TypeScript source into a tree-sitter CST and inspect it
//! structurally — no grep, no false positives from strings or comments.

use tree_sitter::{Node, Parser, Tree};

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

/// Find `process.env` member-expression nodes (not inside strings or comments).
///
/// Matches `process.env.X` and `process.env["X"]`.
/// Returns 1-based line numbers.
pub fn find_process_env(tree: &Tree, source: &str) -> Vec<usize> {
    let mut out = Vec::new();
    collect_process_env(&tree.root_node(), source.as_bytes(), &mut out);
    out
}

/// Find type annotations using `any` (not inside strings or comments).
///
/// Detects both `: any` type annotations and `as any` expressions.
/// Returns 1-based line numbers.
pub fn find_any_types(tree: &Tree, source: &str) -> Vec<usize> {
    let mut out = Vec::new();
    collect_any_types(&tree.root_node(), source.as_bytes(), &mut out);
    out
}

/// Find test method calls like `describe.skip(`, `it.only(`, etc.
///
/// Matches call expressions where the callee is a member expression with
/// property matching `method` (e.g., "skip" or "only") on objects named
/// "test", "describe", "it", "beforeEach", or "afterEach".
/// Returns 1-based line numbers. Ignores occurrences inside strings/comments.
pub fn find_test_method_calls(tree: &Tree, source: &str, method: &str) -> Vec<usize> {
    let mut out = Vec::new();
    collect_test_method_calls(&tree.root_node(), source.as_bytes(), method, &mut out);
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

fn collect_process_env(node: &Node<'_>, source: &[u8], out: &mut Vec<usize>) {
    if (node.kind() == "member_expression" || node.kind() == "subscript_expression")
        && is_process_env_access(node, source)
    {
        let line = node.start_position().row.saturating_add(1);
        // Don't double-report: only report the outermost process.env access.
        if !out.contains(&line) {
            out.push(line);
        }
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_process_env(&child, source, out);
    }
}

/// Check if a node represents `process.env` access (possibly with further property access).
fn is_process_env_access(node: &Node<'_>, source: &[u8]) -> bool {
    let Some(object) = node.child_by_field_name("object") else {
        return false;
    };

    // Case 1: the object itself is `process.env`
    if is_process_dot_env(&object, source) {
        return true;
    }

    // Case 2: this node IS `process.env`
    if is_process_dot_env(node, source) {
        return true;
    }

    // Case 3: deeper nesting like `process.env.FOO.BAR`
    if object.kind() == "member_expression" || object.kind() == "subscript_expression" {
        return is_process_env_access(&object, source);
    }

    false
}

/// Check if a node is exactly `process.env`.
fn is_process_dot_env(node: &Node<'_>, source: &[u8]) -> bool {
    if node.kind() != "member_expression" {
        return false;
    }
    let Some(obj) = node.child_by_field_name("object") else {
        return false;
    };
    let Some(prop) = node.child_by_field_name("property") else {
        return false;
    };
    node_text(&obj, source) == "process" && node_text(&prop, source) == "env"
}

fn collect_any_types(node: &Node<'_>, source: &[u8], out: &mut Vec<usize>) {
    match node.kind() {
        // `: any` in type annotations
        "type_annotation" => {
            if has_predefined_any_child(node, source) {
                let line = node.start_position().row.saturating_add(1);
                out.push(line);
            }
        }
        // `as any` expressions
        "as_expression" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "predefined_type" && node_text(&child, source) == "any" {
                    let line = node.start_position().row.saturating_add(1);
                    out.push(line);
                    break;
                }
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_any_types(&child, source, out);
    }
}

/// Check if a node contains a `predefined_type` descendant with text `any`.
fn has_predefined_any_child(node: &Node<'_>, source: &[u8]) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "predefined_type" && node_text(&child, source) == "any" {
            return true;
        }
        if has_predefined_any_child(&child, source) {
            return true;
        }
    }
    false
}

/// Test runner object names that can have `.skip()` / `.only()`.
const TEST_RUNNER_OBJECTS: &[&str] = &["test", "describe", "it", "beforeEach", "afterEach"];

fn collect_test_method_calls(
    node: &Node<'_>,
    source: &[u8],
    method: &str,
    out: &mut Vec<usize>,
) {
    // call_expression → function: member_expression(object, property)
    if node.kind() == "call_expression" {
        if let Some(callee) = node.child_by_field_name("function") {
            if callee.kind() == "member_expression" {
                if let (Some(obj), Some(prop)) = (
                    callee.child_by_field_name("object"),
                    callee.child_by_field_name("property"),
                ) {
                    let obj_text = node_text(&obj, source);
                    let prop_text = node_text(&prop, source);
                    if prop_text == method
                        && TEST_RUNNER_OBJECTS.contains(&obj_text)
                    {
                        let line = node.start_position().row.saturating_add(1);
                        if !out.contains(&line) {
                            out.push(line);
                        }
                    }
                }
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_test_method_calls(&child, source, method, out);
    }
}

fn node_text<'a>(node: &Node<'_>, source: &'a [u8]) -> &'a str {
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

    // -----------------------------------------------------------------------
    // find_process_env
    // -----------------------------------------------------------------------

    #[test]
    fn process_env_dot_access() {
        let src = "const x = process.env.FOO;";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert_eq!(hits.len(), 1, "should find process.env.FOO");
        assert_eq!(hits.first().copied(), Some(1));
    }

    #[test]
    fn process_env_bracket_access() {
        let src = "const x = process.env[\"FOO\"];";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert_eq!(hits.len(), 1, "should find process.env[\"FOO\"]");
    }

    #[test]
    fn process_env_in_string_not_found() {
        let src = "const x = \"process.env.FOO\";";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside string literal");
    }

    #[test]
    fn process_env_in_comment_not_found() {
        let src = "// process.env.FOO\nconst x = 1;";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside comment");
    }

    #[test]
    fn process_env_in_template_not_found() {
        let src = "const x = `process.env.FOO`;";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside template literal");
    }

    #[test]
    fn process_env_multiple_lines() {
        let src = "const a = process.env.A;\nconst b = process.env.B;";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert_eq!(hits.len(), 2, "should find both");
    }

    #[test]
    fn process_env_no_double_count() {
        let src = "const val = process.env.NODE_ENV;";
        let tree = must_parse(src);
        let hits = find_process_env(&tree, src);
        assert_eq!(hits.len(), 1, "should count once, not twice");
    }

    // -----------------------------------------------------------------------
    // find_any_types
    // -----------------------------------------------------------------------

    #[test]
    fn any_type_annotation() {
        let src = "const x: any = 5;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert_eq!(hits.len(), 1, "should find : any");
    }

    #[test]
    fn any_as_expression() {
        let src = "const x = foo as any;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert_eq!(hits.len(), 1, "should find as any");
    }

    #[test]
    fn any_parameter_type() {
        let src = "function foo(a: any): void {}";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert_eq!(hits.len(), 1, "should find parameter : any");
    }

    #[test]
    fn any_return_type() {
        let src = "function foo(): any { return 1; }";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert_eq!(hits.len(), 1, "should find return : any");
    }

    #[test]
    fn any_in_string_not_found() {
        let src = "const x = \": any\";\nexport default x;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside string");
    }

    #[test]
    fn any_in_comment_not_found() {
        let src = "// const x: any = 5;\nconst y = 1;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside comment");
    }

    #[test]
    fn any_in_block_comment_not_found() {
        let src = "/* as any */ const y = 1;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert!(hits.is_empty(), "should NOT match inside block comment");
    }

    #[test]
    fn any_as_variable_name_not_found() {
        let src = "const any = 5;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert!(
            hits.is_empty(),
            "`any` as variable name should not be detected as type"
        );
    }

    #[test]
    fn other_predefined_types_not_matched() {
        let src = "const x: string = \"hello\"; const y: number = 5;";
        let tree = must_parse(src);
        let hits = find_any_types(&tree, src);
        assert!(hits.is_empty(), "should NOT match string or number types");
    }

    // -----------------------------------------------------------------------
    // find_test_method_calls
    // -----------------------------------------------------------------------

    #[test]
    fn test_skip_calls_found() {
        let src = "describe.skip(\"test\", () => {});";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "skip");
        assert_eq!(hits.len(), 1, "should find describe.skip call");
        assert_eq!(hits.first().copied(), Some(1));
    }

    #[test]
    fn test_skip_in_string_not_found() {
        let src = "const s = \"describe.skip()\";\nexport default s;";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "skip");
        assert!(
            hits.is_empty(),
            "describe.skip inside string literal should not be detected"
        );
    }

    #[test]
    fn test_only_calls_found() {
        let src = "it.only(\"test\", () => {});";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "only");
        assert_eq!(hits.len(), 1, "should find it.only call");
        assert_eq!(hits.first().copied(), Some(1));
    }

    #[test]
    fn test_only_in_string_not_found() {
        let src = "const s = \"it.only()\";\nexport default s;";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "only");
        assert!(
            hits.is_empty(),
            "it.only inside string literal should not be detected"
        );
    }

    #[test]
    fn test_skip_in_comment_not_found() {
        let src = "// test.skip(\"broken\", () => {});\nconst x = 1;";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "skip");
        assert!(
            hits.is_empty(),
            "test.skip inside comment should not be detected"
        );
    }

    #[test]
    fn test_only_multiple_lines() {
        let src = "it.only(\"a\", () => {});\ndescribe.only(\"b\", () => {});";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "only");
        assert_eq!(hits.len(), 2, "should find both .only calls");
    }

    #[test]
    fn test_before_each_skip() {
        let src = "beforeEach.skip(() => {});";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "skip");
        assert_eq!(hits.len(), 1, "should find beforeEach.skip");
    }

    #[test]
    fn test_non_test_object_not_found() {
        let src = "foo.skip(\"test\");";
        let tree = must_parse(src);
        let hits = find_test_method_calls(&tree, src, "skip");
        assert!(
            hits.is_empty(),
            "foo.skip should not match — foo is not a test runner object"
        );
    }
}
