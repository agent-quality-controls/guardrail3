use guardrail3_domain_modules::clippy::AVOID_BREAKING_EXPORTED_API;
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_baseline_contains_expected_avoid_breaking_exported_api_value() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    assert_eq!(
        parsed
            .get("avoid-breaking-exported-api")
            .and_then(toml::Value::as_bool),
        Some(AVOID_BREAKING_EXPORTED_API)
    );
}
