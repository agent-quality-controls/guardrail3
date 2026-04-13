use g3rs_hooks_config_checks_assertions::hook_rs_06_required_tools_installed as assertions;

fn selected_hook(content: &str) -> g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact {
    g3rs_hooks_config_checks_types::G3RsHooksSelectedHookConfigFact {
        rel_path: ".githooks/pre-commit".to_owned(),
        parsed: hook_shell_parser::parse_script(content),
    }
}

#[test]
fn reports_required_tools_as_inventory_when_installed() {
    let mut results = Vec::new();
    crate::hook_rs_06_required_tools_installed::check(
        &selected_hook("gitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n"),
        &["gitleaks".to_owned(), "cargo-deny".to_owned(), "cargo-machete".to_owned()],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                title: Some("gitleaks installed"),
                file: Some(".githooks/pre-commit"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-deny installed"),
                file: Some(".githooks/pre-commit"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-machete installed"),
                file: Some(".githooks/pre-commit"),
                inventory: Some(true),
                ..Default::default()
            },
        ],
    );
}

#[test]
fn reports_missing_tools_as_errors() {
    let mut results = Vec::new();
    crate::hook_rs_06_required_tools_installed::check(
        &selected_hook("gitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n"),
        &["gitleaks".to_owned()],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                title: Some("gitleaks installed"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-deny missing"),
                inventory: Some(false),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-machete missing"),
                inventory: Some(false),
                ..Default::default()
            },
        ],
    );
}

#[test]
fn treats_path_qualified_tools_as_installed() {
    let mut results = Vec::new();
    crate::hook_rs_06_required_tools_installed::check(
        &selected_hook(
            "/opt/bin/gitleaks protect --staged --no-banner\n/opt/bin/cargo-deny check\n/opt/bin/cargo-machete\n",
        ),
        &[],
        &mut results,
    );

    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                title: Some("gitleaks installed"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-deny installed"),
                inventory: Some(true),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                title: Some("cargo-machete installed"),
                inventory: Some(true),
                ..Default::default()
            },
        ],
    );
}
