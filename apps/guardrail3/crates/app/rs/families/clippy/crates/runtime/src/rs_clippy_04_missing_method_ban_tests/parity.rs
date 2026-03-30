use guardrail3_domain_modules::clippy::SERVICE_METHOD_PATHS;
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_method_ban_set_matches_rule_baseline() {
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
    let expected = SERVICE_METHOD_PATHS.to_vec();

    assert_eq!(actual, expected);
}
