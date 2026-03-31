use guardrail3_app_rs_family_deny_assertions::rs_deny_25_allow_override_channel as assertions;

use super::super::{build_fixture_deny_toml, set_bans_allow_entries};

#[test]
fn standalone_app_root_uses_rust_apps_library_profile_for_allow_override_checks() {
    let results = crate::run_config_rule_for_test(
        &set_bans_allow_entries(
            &build_fixture_deny_toml("library"),
            vec![toml::Value::String("axum".to_owned())],
        ),
        Some("library"),
        super::super::check,
    );

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "allow-list overrides deny-list",
                "`deny.toml` allows `axum` even though it is banned.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "bans allow-list present",
                "`deny.toml` has non-empty `[bans].allow`: axum.",
                "deny.toml",
                false,
            ),
        ],
    );
}
