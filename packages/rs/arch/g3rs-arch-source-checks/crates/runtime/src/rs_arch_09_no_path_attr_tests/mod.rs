use g3rs_arch_source_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use g3rs_arch_types::G3RsArchSourceFile;
use guardrail3_check_types::G3Severity;

fn source_file(rel_path: &str, content: &str) -> G3RsArchSourceFile {
    G3RsArchSourceFile {
        rel_path: rel_path.to_owned(),
        content: content.to_owned(),
    }
}

#[test]
fn cfg_test_sidecar_path_is_allowed() {
    let file = source_file(
        "crate_a/src/run.rs",
        "#[cfg(test)]\n#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
    );
    let mut results = Vec::new();

    crate::rs_arch_09_no_path_attr::check_file(&file, &mut results);

    assert!(!has_rule(&results, "RS-ARCH-SOURCE-09"));
}

#[test]
fn non_test_path_attr_still_fires() {
    let file = source_file(
        "crate_a/src/run.rs",
        "#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
    );
    let mut results = Vec::new();

    crate::rs_arch_09_no_path_attr::check_file(&file, &mut results);

    assert_rule_results(
        &results,
        "RS-ARCH-SOURCE-09",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[path] attribute forbidden"),
            file: Some("crate_a/src/run.rs"),
            inventory: Some(false),
            message: Some("`#[path = \"run_tests/mod.rs\"]` on `mod run_tests` bypasses the module facade. Use standard module resolution with mod.rs instead. Every module directory must have a mod.rs that serves as its facade."),
        }],
    );
}
