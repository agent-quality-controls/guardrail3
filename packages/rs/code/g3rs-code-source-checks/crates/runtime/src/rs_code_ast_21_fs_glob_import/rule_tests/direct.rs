use g3rs_code_source_checks_assertions::rs_code_ast_21_fs_glob_import::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_direct_std_fs_glob() {
    let results = super::super::check_source("src/foo.rs", "use std::fs::*;\nfn main() {}", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("std::fs glob import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `use std::fs::*` glob import bypasses clippy method bans."),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_std_alias_glob_import() {
    let content = "use std as s;\nuse s::fs::*;\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("std::fs glob import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `use std::fs::*` glob import bypasses clippy method bans."),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_grouped_std_fs_glob_import() {
    let results =
        super::super::check_source("src/foo.rs", "use std::{fs::*, io};\nfn main() {}", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("std::fs glob import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some("Direct `use std::fs::*` glob import bypasses clippy method bans."),
            line: Some(1),
        }],
    );
}
