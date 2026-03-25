//! Tests extracted from `app::ts::validate::ts_code_analysis`
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

use guardrail3_app_ts::validate::ast_helpers::parse_typescript;
use guardrail3_app_ts::validate::ts_code_analysis::{
    find_any_types, find_process_env, find_test_method_calls,
};
use tree_sitter::Tree;

fn must_parse(source: &str) -> Tree {
    parse_typescript(source).expect("test input should be valid TypeScript")
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
