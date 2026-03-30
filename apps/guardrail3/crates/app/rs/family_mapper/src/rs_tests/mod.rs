use guardrail3_app_rs_family_mapper_assertions::rs::{assert_disabled, assert_enabled};

#[test]
fn other_roots_are_excluded_when_app_scope_is_configured() {
    let config = super::app_scoped_config_test();

    assert_disabled(super::root_enabled_for_toolchain_test(
        &super::root_test("fuzz"),
        Some(&config),
    ));
}

#[test]
fn configured_app_root_stays_enabled_when_app_scope_is_configured() {
    let config = super::app_scoped_config_test();

    assert_enabled(super::root_enabled_for_toolchain_test(
        &super::root_test("apps/guardrail3"),
        Some(&config),
    ));
}

#[test]
fn other_roots_follow_global_flag_when_no_scope_is_configured() {
    let config = super::global_toolchain_enabled_config_test();

    assert_enabled(super::root_enabled_for_toolchain_test(
        &super::root_test("fuzz"),
        Some(&config),
    ));
}
