use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_25_allow_override_channel as assertions;

use super::super::build_fixture_deny_toml;

#[test]
fn errors_when_bans_allow_is_not_an_array() {
    let mut parsed =
        toml::from_str::<toml::Value>(&build_fixture_deny_toml("service")).expect("valid deny");
    let bans = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [bans] table");
    let _ = bans.insert(
        "allow".to_owned(),
        toml::Value::String("lazy_static".to_owned()),
    );

    let results = super::super::run_check(&toml::to_string(&parsed).expect("serialize deny"));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list malformed",
            "`deny.toml` must keep `[bans].allow` as an array of crate entries.",
            "deny.toml",
            false,
        )],
    );
}

#[test]
fn errors_when_bans_allow_entries_have_no_matchable_crate_name() {
    let mut parsed =
        toml::from_str::<toml::Value>(&build_fixture_deny_toml("service")).expect("valid deny");
    let bans = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [bans] table");
    let _ = bans.insert(
        "allow".to_owned(),
        toml::Value::Array(vec![toml::Value::Table(toml::map::Map::from_iter([(
            "reason".to_owned(),
            toml::Value::String("temporary".to_owned()),
        )]))]),
    );

    let results = super::super::run_check(&toml::to_string(&parsed).expect("serialize deny"));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list malformed",
            "`deny.toml` has malformed `[bans].allow` entries that cannot be matched to crate names.",
            "deny.toml",
            false,
        )],
    );
}
