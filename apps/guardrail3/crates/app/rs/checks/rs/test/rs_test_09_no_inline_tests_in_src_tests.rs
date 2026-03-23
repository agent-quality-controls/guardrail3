use crate::domain::report::Severity;

use super::super::parse::{analyze, parse_rust_file};
use super::super::test_support::file_input;
use super::check;

#[test]
fn errors_on_inline_cfg_test_module() {
    let content =
        "pub fn run() {}\n#[cfg(test)]\nmod tests { #[test] fn long_named_test_case() {} }";
    let ast = parse_rust_file(content).expect("parse");
    let parsed = analyze(&ast, content);
    let input = file_input("src/lib.rs", true, false, false, content, parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(
        results
            .iter()
            .any(|result| { result.id == "RS-TEST-09" && result.severity == Severity::Error })
    );
}

#[test]
fn allows_sidecar_test_file_in_src() {
    let content = "#[test]\nfn long_named_test_case() {}";
    let ast = parse_rust_file(content).expect("parse");
    let parsed = analyze(&ast, content);
    let input = file_input("src/lib_tests.rs", true, false, true, content, parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}
