use g3rs_code_source_checks_assertions::path_attr_with_reason::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn skips_canonical_test_sidecar() {
    let results = super::super::check_source(
        "src/run.rs",
        "#[cfg(test)]\n#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn wrong_test_module_name_is_not_exempt() {
    let results = super::super::check_source(
        "src/rule.rs",
        "#[cfg(test)]\n#[path = \"rule_tests/mod.rs\"] // reason: owned sidecar tests for file module\nmod tests;\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("#[path] with reason"),
            file: Some("src/rule.rs"),
            inventory: Some(false),
            message: Some(
                "`#[path = \"rule_tests/mod.rs\"]` on `mod tests` reason: owned sidecar tests for file module",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_weak_reason() {
    let results = super::super::check_source(
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
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(any(), path = \"generated.rs\")]\nmod generated;\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
