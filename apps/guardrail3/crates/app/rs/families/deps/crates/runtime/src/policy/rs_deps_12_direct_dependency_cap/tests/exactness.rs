use super::{direct_dependency_cap_facts, run_with_facts};
use guardrail3_app_rs_family_deps_assertions::rs_deps_12_direct_dependency_cap as assertions;

#[test]
fn stays_quiet_at_exactly_twenty_five_unique_direct_dependencies() {
    let facts = direct_dependency_cap_facts("api", "apps/api/Cargo.toml", 25);
    let results = run_with_facts(&facts);

    assertions::assert_rule_quiet(&results);
}

#[test]
fn errors_above_twenty_five_unique_direct_dependencies() {
    let facts = direct_dependency_cap_facts("api", "apps/api/Cargo.toml", 26);
    let results = run_with_facts(&facts);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("too many direct dependencies"),
            file: Some("apps/api/Cargo.toml"),
            message: Some("Crate `api` has 26 unique direct dependencies (max 25)."),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
