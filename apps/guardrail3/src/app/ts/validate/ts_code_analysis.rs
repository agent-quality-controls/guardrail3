//! Tree-sitter-based TypeScript code analysis helpers.
//!
//! Structural analysis of TypeScript source: `process.env` access detection,
//! `any` type usage, and test method call discovery (`.skip()`, `.only()`).

use tree_sitter::{Node, Tree};

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

fn collect_test_method_calls(node: &Node<'_>, source: &[u8], method: &str, out: &mut Vec<usize>) {
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
                    if prop_text == method && TEST_RUNNER_OBJECTS.contains(&obj_text) {
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

