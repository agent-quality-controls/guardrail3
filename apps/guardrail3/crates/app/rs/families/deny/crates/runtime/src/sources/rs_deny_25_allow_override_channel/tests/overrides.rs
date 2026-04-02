use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_25_allow_override_channel as assertions;

use super::super::{build_fixture_deny_toml, set_bans_allow_entries};

#[test]
fn errors_on_non_empty_allow_list_and_deny_overrides() {
    let allow = vec![
        toml::Value::String("lazy_static".to_owned()),
        toml::Value::String("json5".to_owned()),
    ];
    let results = super::super::run_check(&set_bans_allow_entries(
        &build_fixture_deny_toml("service"),
        allow,
    ));

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "allow-list overrides deny-list",
                "`deny.toml` allows `json5` even though it is banned.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "allow-list overrides deny-list",
                "`deny.toml` allows `lazy_static` even though it is banned.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "bans allow-list present",
                "`deny.toml` has non-empty `[bans].allow`: json5, lazy_static.",
                "deny.toml",
                false,
            ),
        ],
    );
}
