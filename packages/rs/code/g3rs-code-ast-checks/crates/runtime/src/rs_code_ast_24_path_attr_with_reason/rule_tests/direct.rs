use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_24_path_attr_with_reason::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_path_without_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"generated.rs\"]\nmod generated;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[path] without reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"generated.rs\"]` on `mod generated` redirects module resolution. Add a specific same-line `// reason:` comment.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_path_escape() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"../generated.rs\"]\nmod generated;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[path] escapes parent directory"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"../generated.rs\"]` on `mod generated` uses a parent-directory segment. Keep module resolution inside the normal Rust module tree.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_path_with_useful_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"generated.rs\"] // reason: generated bridge shim\nmod generated;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("#[path] with reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"generated.rs\"]` on `mod generated` reason: generated bridge shim",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_conditional_path_with_useful_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg_attr(feature = \"cli\", path = \"generated.rs\")] // reason: generated bridge shim\nmod generated;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("#[path] with reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"generated.rs\"]` on `mod generated` reason: generated bridge shim",
            ),
            line: Some(1),
        }],
    );
}
