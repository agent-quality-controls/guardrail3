use g3rs_hooks_file_tree_checks_assertions::hook_shared_02_hooks_path_configured as assertions;

#[test]
fn reports_missing_hooks_path() {
    let mut results = Vec::new();
    crate::hook_shared_02_hooks_path_configured::check(None, &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("core.hooksPath not configured"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_expected_hooks_path() {
    let mut results = Vec::new();
    crate::hook_shared_02_hooks_path_configured::check(Some(".githooks"), &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("core.hooksPath configured"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn treats_empty_hooks_path_as_wrong_value() {
    let mut results = Vec::new();
    crate::hook_shared_02_hooks_path_configured::check(Some(""), &mut results);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            title: Some("core.hooksPath has wrong value"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
