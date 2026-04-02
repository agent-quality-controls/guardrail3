use guardrail3_app_rs_family_clippy_assertions::rs_clippy_06_extra_method_ban as assertions;
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_methods_do_not_contain_project_specific_extras() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-methods")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .collect::<Vec<_>>();
    let expected = assertions::service_method_bans();

    assert_eq!(actual, expected);
}
