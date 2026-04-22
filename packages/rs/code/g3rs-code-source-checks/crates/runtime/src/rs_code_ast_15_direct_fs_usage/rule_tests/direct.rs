use g3rs_code_source_checks_assertions::rs_code_ast_15_direct_fs_usage::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_std_fs_import() {
    let content = "use std::fs;\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use std::fs;`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_inline_std_fs_call() {
    let content = "fn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn still_errors_inside_allow_scoped_std_fs_usage() {
    let content = "#[allow(clippy::disallowed_methods)]\nfn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_std_alias_fs_call() {
    let content = "use std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_chained_std_alias_fs_call() {
    let content = "use std as s;\nuse s as t;\nfn main() { let _ = t::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = t::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(3),
        }],
    );
}

#[test]
fn errors_on_std_alias_then_fs_import() {
    let content = "use std as s;\nuse s::fs;\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use s::fs;`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_forward_std_alias_fs_import() {
    let content = "use s::fs;\nuse std as s;\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use s::fs;`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_forward_std_alias_fs_call_inside_function_scope() {
    let content =
        "fn main() {\n    let _ = s::fs::read_to_string(\"foo\");\n    use std as s;\n}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `let _ = s::fs::read_to_string(\"foo\");`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn does_not_leak_std_alias_across_sibling_functions_for_call() {
    let content = "fn define_alias() {\n    use std as s;\n}\nfn probe() {\n    let _ = s::fs::read_to_string(\"foo\");\n}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(&results, &[]);
}

#[test]
fn errors_on_grouped_std_fs_self_import() {
    let content = "use std::fs::{self, *};\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use std::fs::{self, *};`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_alias_backed_grouped_std_fs_self_import() {
    let content = "use std as s;\nuse s::fs::{self, *};\nfn main() {}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use s::fs::{self, *};`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn does_not_leak_std_alias_across_sibling_functions_for_import() {
    let content = "fn define_alias() {\n    use std as s;\n}\nfn probe() {\n    use s::fs;\n}";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(&results, &[]);
}

#[test]
fn errors_on_extern_crate_std_alias_fs_call() {
    let content = "extern crate std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_chained_extern_crate_std_alias_fs_call() {
    let content =
        "extern crate std as s;\nuse s as t;\nfn main() { let _ = t::fs::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs call"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `std::fs::*` call found: `fn main() { let _ = t::fs::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(3),
        }],
    );
}

#[test]
fn errors_on_std_fs_alias_call() {
    let content = "use std::fs as fs2;\nfn main() { let _ = fs2::read_to_string(\"foo\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("direct std::fs import"),
                file: Some("src/foo.rs"),
                inventory: Some(false),
                message: Some(
                    "Direct `use std::fs` import found: `use std::fs as fs2;`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
                ),
                line: Some(1),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("direct std::fs call"),
                file: Some("src/foo.rs"),
                inventory: Some(false),
                message: Some(
                    "Direct `std::fs::*` call found: `fn main() { let _ = fs2::read_to_string(\"foo\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
                ),
                line: Some(2),
            },
        ],
    );
}

#[test]
fn prefers_import_hit_when_import_and_call_share_one_line() {
    let content =
        "use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }";
    let results = super::super::check_source("src/foo.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("direct std::fs import"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "Direct `use std::fs` import found: `use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
            ),
            line: Some(1),
        }],
    );
}
