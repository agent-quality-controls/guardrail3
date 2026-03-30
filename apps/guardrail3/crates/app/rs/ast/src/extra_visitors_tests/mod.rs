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
        findings: Vec::new(),
    };
    v.visit_file(&file);
    assert!(
        v.findings.is_empty(),
        "ignore with = reason should not be flagged"
    );
}

#[test]
fn cfg_attr_ignore_without_reason_is_flagged() {
    let src = "#[test]\n#[cfg_attr(test, ignore)]\nfn slow_test() {}";
    let file = must_parse(src);
    let mut v = IgnoreVisitor {
        lines: src.lines().collect(),
        findings: Vec::new(),
    };
    v.visit_file(&file);
    assert_eq!(v.findings.len(), 1);
    assert_eq!(v.findings[0].line, 2);
    assert_eq!(v.findings[0].reason, None);
}

#[test]
fn cfg_attr_ignore_with_inline_reason_not_flagged() {
    let src = "#[test]\n#[cfg_attr(test, ignore)] // reason: requires network\nfn slow_test() {}";
    let file = must_parse(src);
    let mut v = IgnoreVisitor {
        lines: src.lines().collect(),
        findings: Vec::new(),
    };
    v.visit_file(&file);
    assert_eq!(v.findings.len(), 1);
    assert_eq!(v.findings[0].line, 2);
    assert_eq!(v.findings[0].reason.as_deref(), Some("requires network"));
}
