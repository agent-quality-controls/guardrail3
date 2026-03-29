use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_baseline_keeps_test_relaxation_policy_exact() {
    let parsed =
        toml::from_str::<toml::Value>(&build_fixture_clippy_toml("service", false, true, "", ""))
            .expect("valid clippy TOML");

    for key in [
        "allow-dbg-in-tests",
        "allow-expect-in-tests",
        "allow-panic-in-tests",
        "allow-print-in-tests",
        "allow-unwrap-in-tests",
    ] {
        assert_eq!(
            parsed.get(key).and_then(toml::Value::as_bool),
            Some(expected_bool_value(key)),
            "unexpected canonical value for {key}"
        );
    }
}

fn expected_bool_value(key: &str) -> bool {
    match key {
        "allow-dbg-in-tests" => false,
        "allow-expect-in-tests" => true,
        "allow-panic-in-tests" => false,
        "allow-print-in-tests" => false,
        "allow-unwrap-in-tests" => false,
        _ => unreachable!("unsupported key"),
    }
}
