use super::helpers::check_source;

use g3rs_code_ast_checks_assertions::rs_code_24_path_attr_with_reason::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn skips_canonical_test_sidecar() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg(test)]\n#[path = \"rs_code_ast_24_path_attr_with_reason_tests/mod.rs\"]\nmod rs_code_ast_24_path_attr_with_reason_tests;\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn errors_on_weak_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"generated.rs\"] // reason: temp\nmod generated;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[path] reason too weak"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"generated.rs\"]` on `mod generated` needs a specific same-line `// reason:` comment. Weak reason `temp` found.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn skips_known_false_cfg_attr_path() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg_attr(any(), path = \"generated.rs\")]\nmod generated;\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
