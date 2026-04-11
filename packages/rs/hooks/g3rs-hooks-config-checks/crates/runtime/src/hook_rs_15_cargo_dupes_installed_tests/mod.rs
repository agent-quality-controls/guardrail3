use g3rs_hooks_config_checks_assertions::hook_rs_15_cargo_dupes_installed as assertions;
use g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact;

fn hook(content: &str) -> G3RsHooksSelectedHookConfigFact {
    G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        content: content.to_owned(),
    }
}

#[test]
fn stays_quiet_when_hook_does_not_require_cargo_dupes() {
    let mut results = Vec::new();
    crate::hook_rs_15_cargo_dupes_installed::check(
        &hook("cargo fmt --check\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_quiet(&results);
}

#[test]
fn reports_inventory_when_cargo_dupes_is_installed() {
    let mut results = Vec::new();
    crate::hook_rs_15_cargo_dupes_installed::check(
        &hook("cargo-dupes check --exclude-tests\n"),
        &["cargo-dupes".to_owned()],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("cargo-dupes installed"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_inventory_when_cargo_dupes_is_path_qualified() {
    let mut results = Vec::new();
    crate::hook_rs_15_cargo_dupes_installed::check(
        &hook("/usr/local/bin/cargo-dupes check --exclude-tests\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("cargo-dupes installed"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn reports_missing_cargo_dupes_when_required() {
    let mut results = Vec::new();
    crate::hook_rs_15_cargo_dupes_installed::check(
        &hook("cargo dupes check --exclude-tests\n"),
        &[],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("cargo-dupes missing"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
