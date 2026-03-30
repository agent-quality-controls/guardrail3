use syn::visit::Visit;

use crate::extra_visitors::IgnoreVisitor;

fn must_parse(source: &str) -> syn::File {
    syn::parse_file(source).unwrap_or_else(|e| panic!("test input should be valid Rust: {e}"))
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
