use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_15_direct_fs_usage::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_std_fs_import() {
    let content = "use std::fs;\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `use std::fs` import found: `use std::fs;`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_inline_std_fs_call() {
    let content = "fn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(1),
        }],
    );
}

#[test]
fn still_errors_inside_allow_scoped_std_fs_usage() {
    let content =
        "#[allow(clippy::disallowed_methods)]\nfn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_std_alias_fs_call() {
    let content = "use std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_extern_crate_std_alias_fs_call() {
    let content = "extern crate std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(2),
        }],
    );
}

#[test]
fn prefers_import_hit_when_import_and_call_share_one_line() {
    let content =
        "use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `use std::fs` import found: `use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly."),
            line: Some(1),
        }],
    );
}
