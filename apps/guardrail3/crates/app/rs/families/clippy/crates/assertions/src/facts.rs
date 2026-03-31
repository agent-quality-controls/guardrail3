pub fn assert_root_config_uses_packages_profile_when_packages_policy_exists(
    profile_name: Option<&str>,
    garde_enabled: bool,
) {
    assert_eq!(profile_name, Some("library"));
    assert!(!garde_enabled);
}

pub fn assert_workspace_local_app_root_uses_rust_apps_profile_policy(
    profile_name: Option<&str>,
    garde_enabled: bool,
) {
    assert_eq!(profile_name, Some("library"));
    assert!(!garde_enabled);
}

pub fn assert_malformed_guardrail_policy_is_recorded_as_policy_context_error(
    parse_error_present: bool,
    all_allowed_configs_have_parse_error: bool,
) {
    assert!(
        parse_error_present,
        "expected malformed guardrail3.toml to be recorded"
    );
    assert!(
        all_allowed_configs_have_parse_error,
        "expected malformed policy context to propagate to routed clippy configs"
    );
}

pub fn assert_package_workspace_root_uses_rust_packages_profile_policy(
    profile_name: Option<&str>,
    garde_enabled: bool,
) {
    assert_eq!(profile_name, Some("library"));
    assert!(!garde_enabled);
}
