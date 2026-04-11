use g3rs_hooks_config_checks_assertions::hook_rs_14_guardrail_binary_available as assertions;
use g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact;

fn hook(content: &str) -> G3RsHooksSelectedHookConfigFact {
    G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        content: content.to_owned(),
    }
}

#[test]
fn stays_quiet_when_hook_does_not_require_g3rs() {
    let mut results = Vec::new();
    crate::hook_rs_14_guardrail_binary_available::check(
        &hook("cargo fmt --check\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn reports_inventory_when_g3rs_is_installed() {
    let mut results = Vec::new();
    crate::hook_rs_14_guardrail_binary_available::check(
        &hook("g3rs rs validate --staged .\n"),
        &["g3rs".to_owned()],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("g3rs binary available"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_inventory_when_g3rs_is_path_qualified() {
    let mut results = Vec::new();
    crate::hook_rs_14_guardrail_binary_available::check(
        &hook("/usr/local/bin/g3rs rs validate --staged .\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("g3rs binary available"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_missing_g3rs_when_validation_is_required() {
    let mut results = Vec::new();
    crate::hook_rs_14_guardrail_binary_available::check(
        &hook("g3rs rs validate --staged .\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("g3rs binary missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
