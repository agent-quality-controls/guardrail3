use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_25_allow_override_channel as assertions;

use super::super::{build_fixture_deny_toml, set_bans_allow_entries};

#[test]
fn local_allow_list_presence_only_errors_for_the_owned_local_root() {
    let results = super::super::run_check(&set_bans_allow_entries(
        &build_fixture_deny_toml("service"),
        vec![toml::Value::String("totally-custom-crate".to_owned())],
    ));
    assert!(!results.is_empty());
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
