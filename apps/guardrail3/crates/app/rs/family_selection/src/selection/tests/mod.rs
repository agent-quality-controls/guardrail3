use guardrail3_app_rs_family_selection_assertions::selection as assertions;

use crate::selection::resolve_for_tests;

#[test]
fn explicit_family_request_bypasses_disabled_config_filter() {
    let tree = assertions::minimal_tree();
    let config = assertions::config_for_explicit_topology_request();
    let requested = assertions::explicit_topology_request();

    let selection = resolve_for_tests(&tree, Some(&config), &requested);

    assertions::assert_explicit_request_bypasses_disabled_config_filter(&selection);
}

#[test]
fn empty_request_uses_enabled_family_filtering() {
    let tree = assertions::minimal_tree();
    let config = assertions::config_for_enabled_family_filtering();

    let selection = resolve_for_tests(&tree, Some(&config), &[]);

    assertions::assert_enabled_family_filtering(&selection);
}
