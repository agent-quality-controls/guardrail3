use syn::visit::Visit;

use crate::ast_helpers::parse_file;
use crate::extra_visitors::IgnoreVisitor;

#[allow(clippy::expect_used)] // reason: test helper
fn must_parse(source: &str) -> syn::File {
    parse_file(source).expect("test input should be valid Rust")
}

#[test]
fn ignore_with_name_value_reason_not_flagged() {
    let src = "#[test]\n#[ignore = \"requires network\"]\nfn slow_test() {}";
    let file = must_parse(src);
    let mut v = IgnoreVisitor {
        lines: src.lines().collect(),
        violations: Vec::new(),
    };
    v.visit_file(&file);
    assert!(
        v.violations.is_empty(),
        "ignore with = reason should not be flagged"
    );
}
