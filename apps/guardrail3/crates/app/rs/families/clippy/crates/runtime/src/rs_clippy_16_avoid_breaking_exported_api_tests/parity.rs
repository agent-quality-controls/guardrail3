use super::super::super::clippy_support::expected_bool_value;
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_service_baseline_contains_expected_avoid_breaking_exported_api_value() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    assert_eq!(
        parsed
            .get("avoid-breaking-exported-api")
            .and_then(toml::Value::as_bool),
        expected_bool_value("avoid-breaking-exported-api")
    );
}
