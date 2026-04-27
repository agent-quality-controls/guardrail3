use g3rs_code_source_checks_assertions::fs_glob_import::rule::{
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
fn errors_on_forward_std_alias_glob_import() {
    let content = "use s::fs::*;\nuse std as s;\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

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
fn errors_on_std_fs_alias_glob_import() {
    let content = "use std::fs as fs2;\nuse fs2::*;\nfn main() {}";
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
fn does_not_leak_std_fs_alias_across_sibling_functions_for_glob_import() {
    let content =
        "fn define_alias() {\n    use std::fs as fs2;\n}\nfn probe() {\n    use fs2::*;\n}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(&results, &[]);
}

#[test]
fn does_not_leak_std_fs_alias_from_inner_block_scope_for_glob_import() {
    let content = "fn probe() {\n    {\n        use std::fs as fs2;\n    }\n    use fs2::*;\n}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(&results, &[]);
}

#[test]
fn errors_on_grouped_std_fs_star_import() {
    let results =
        super::super::check_source("src/foo.rs", "use std::fs::{*};\nfn main() {}", false);

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
fn errors_on_alias_backed_grouped_std_fs_glob_import() {
    let content = "use std as s;\nuse s::fs::{*};\nfn main() {}";
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

#[test]
fn errors_on_mixed_cfg_any_std_fs_glob_import() {
    let content = "#[cfg(any(test, unix))]\nuse std::fs::*;\nfn main() {}";
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
