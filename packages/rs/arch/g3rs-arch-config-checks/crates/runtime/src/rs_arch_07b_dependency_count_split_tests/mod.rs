use g3rs_arch_config_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use guardrail3_check_types::G3Severity;

use crate::test_support::{config_crate, input};

#[test]
fn exact_dependency_threshold_stays_quiet() {
    let mut node = config_crate("crate_a");
    node.dependency_count = 12;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-07"));
}

#[test]
fn dependency_threshold_over_limit_fires_config_rule() {
    let mut node = config_crate("crate_a");
    node.dependency_count = 13;

    let results = crate::check(&input(vec![node], Vec::new()));

    assert_rule_results(
        &results,
        "RS-ARCH-CONFIG-07",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate has too many direct dependencies, must split"),
            file: Some("crate_a/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}
