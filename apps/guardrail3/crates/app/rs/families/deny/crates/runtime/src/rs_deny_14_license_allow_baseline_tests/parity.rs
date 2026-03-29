use super::super::build_fixture_deny_toml;
use super::super::expected_licenses_for_test;

#[test]
fn generated_license_baseline_contains_exact_expected_allow_list_and_private_ignore() {
    let parsed = toml::from_str::<toml::Value>(&build_fixture_deny_toml("service"))
        .expect("valid deny TOML");
    let licenses = parsed.get("licenses").expect("licenses section");

    let actual_allow = licenses
        .get("allow")
        .and_then(toml::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();

    assert_eq!(actual_allow, expected_licenses_for_test());
    assert_eq!(
        licenses
            .get("private")
            .and_then(|value| value.get("ignore"))
            .and_then(toml::Value::as_bool),
        Some(true)
    );
}
