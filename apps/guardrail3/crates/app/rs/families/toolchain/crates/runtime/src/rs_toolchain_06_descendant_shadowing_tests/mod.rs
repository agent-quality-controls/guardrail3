use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_06_descendant_shadowing::{
    ExpectedRuleResult, Severity, assert_rule_results,
};

use super::{check, descendant_legacy, descendant_modern, test_input};

#[test]
fn emits_no_result_for_workspace_root_without_descendant_toolchains() {
    let input = test_input(Vec::new());
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(&results, &[]);
}

#[test]
fn errors_when_descendant_modern_toolchain_exists_beneath_workspace_root() {
    let input = test_input(vec![descendant_modern("crates/member/rust-toolchain.toml")]);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "descendant toolchain shadows workspace policy",
            message: "Descendant `rust-toolchain.toml` at `crates/member/rust-toolchain.toml` can override the workspace-root toolchain contract. Keep toolchain policy at the workspace root only.",
            file: Some("crates/member/rust-toolchain.toml"),
        }],
    );
}

#[test]
fn errors_when_descendant_legacy_toolchain_exists_beneath_workspace_root() {
    let input = test_input(vec![descendant_legacy("crates/member/rust-toolchain")]);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "descendant legacy toolchain shadows workspace policy",
            message: "Descendant `rust-toolchain` at `crates/member/rust-toolchain` can override the workspace-root toolchain contract. Keep toolchain policy at the workspace root only.",
            file: Some("crates/member/rust-toolchain"),
        }],
    );
}
