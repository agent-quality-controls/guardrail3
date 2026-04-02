use guardrail3_app_rs_family_hooks_shared_assertions::inventories::hook_shared_12_modular_scripts_executable as assertions;

use super::check;

#[test]
fn returns_no_results_for_empty_inventory() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assertions::assert_rule_quiet(&results);
}

#[test]
fn inventories_executable_script_and_flags_non_executable_script() {
    let mut results = Vec::new();
    check(
        &[
            (".githooks/pre-commit.d/10-rust.sh".to_owned(), true),
            (".githooks/pre-commit.d/20-ts.sh".to_owned(), false),
        ],
        &mut results,
    );
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title: Some("modular hook script is executable"),
                file: Some(".githooks/pre-commit.d/10-rust.sh"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                title: Some("modular hook script is not executable"),
                file: Some(".githooks/pre-commit.d/20-ts.sh"),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}
