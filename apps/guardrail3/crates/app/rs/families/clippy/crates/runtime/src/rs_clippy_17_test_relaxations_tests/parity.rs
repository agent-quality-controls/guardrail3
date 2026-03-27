use super::super::super::clippy_support::expected_bool_value;
use super::super::super::test_support::canonical_clippy_toml;

#[test]
fn generated_service_baseline_keeps_test_relaxation_policy_exact() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_clippy_toml()).expect("valid clippy TOML");

    for key in [
        "allow-dbg-in-tests",
        "allow-expect-in-tests",
        "allow-print-in-tests",
        "allow-unwrap-in-tests",
    ] {
        assert_eq!(
            parsed.get(key).and_then(toml::Value::as_bool),
            expected_bool_value(key),
            "unexpected canonical value for {key}"
        );
    }
}
