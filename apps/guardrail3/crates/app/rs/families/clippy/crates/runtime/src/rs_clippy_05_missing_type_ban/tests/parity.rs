use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_type_ban_list_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .collect::<Vec<_>>();
    let expected = assertions::service_type_bans();

    assert_eq!(actual, expected);
}

#[test]
fn generated_library_type_ban_list_matches_rule_baseline() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("library", false, true, "", ""))
            .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .collect::<Vec<_>>();
    let expected = assertions::library_type_bans();

    assert_eq!(actual, expected);
}
