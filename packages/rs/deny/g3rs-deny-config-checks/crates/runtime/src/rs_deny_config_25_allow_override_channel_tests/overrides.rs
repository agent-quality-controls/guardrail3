use g3rs_deny_config_checks_assertions::rs_deny_config_25_allow_override_channel as assertions;

use crate::test_support::{canonical_bans_toml, run};

#[test]
fn errors_on_non_empty_allow_list_and_deny_overrides() {
    let mut parsed =
        toml::from_str::<toml::Value>(&canonical_bans_toml("service")).expect("valid deny fixture");
    let bans = parsed
        .get_mut("bans")
        .and_then(toml::Value::as_table_mut)
        .expect("expected [bans] table");
    let _ = bans.insert(
        "allow".to_owned(),
        toml::Value::Array(vec![
            toml::Value::String("json5".to_owned()),
            toml::Value::String("lazy_static".to_owned()),
        ]),
    );
    let deny_toml = toml::to_string(&parsed).expect("serialize deny");

    let results = run(
        &deny_toml,
        Some("service"),
        true,
        crate::rs_deny_config_25_allow_override_channel::check,
    );

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
