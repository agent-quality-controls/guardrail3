pub fn assert_root_profile_name(profile_name: Option<&str>, expected: &str) {
    assert_eq!(profile_name, Some(expected));
}

pub fn assert_standalone_app_root_uses_rust_apps_profile_policy(
    profile_name: Option<&str>,
) {
    assert_eq!(profile_name, Some("library"));
}

pub fn assert_malformed_guardrail_policy_is_recorded_as_policy_context_error(
    parse_error_present: bool,
    all_allowed_configs_marked_invalid: bool,
) {
    assert!(parse_error_present, "expected malformed guardrail3.toml to be recorded");
    assert!(
        all_allowed_configs_marked_invalid,
        "expected malformed policy context to propagate to routed deny configs"
    );
}

pub fn assert_guardrail_policy_parse_error(
    parse_error: Option<&str>,
    all_allowed_configs_marked_invalid: bool,
    expected_fragment: &str,
) {
    let parse_error = parse_error.expect("expected deny policy-context parse error");
    assert!(
        parse_error.contains(expected_fragment),
        "expected parse error to contain `{expected_fragment}`, got `{parse_error}`"
    );
    assert!(
        all_allowed_configs_marked_invalid,
        "expected malformed guardrail policy to disable profile-sensitive config evaluation"
    );
}
