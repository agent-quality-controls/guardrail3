use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_25_allow_override_channel as assertions;

use super::helpers::{build_fixture_deny_toml, set_bans_allow_entries};

#[test]
fn errors_on_non_empty_allow_list_even_when_it_does_not_override_a_ban() {
    let allow = vec![toml::Value::String("totally-custom-crate".to_owned())];
    let results = super::helpers::run_check(&set_bans_allow_entries(
        &build_fixture_deny_toml("service"),
        allow,
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list present",
            "`deny.toml` has non-empty `[bans].allow`: totally-custom-crate.",
            "deny.toml",
            false,
        )],
    );
}
