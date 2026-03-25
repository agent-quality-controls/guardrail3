use super::super::super::deny_support::expected_licenses;
use super::super::super::test_support::canonical_deny_toml_service;

#[test]
fn generated_license_baseline_contains_exact_expected_allow_list_and_private_ignore() {
    let parsed =
        toml::from_str::<toml::Value>(&canonical_deny_toml_service()).expect("valid deny TOML");
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

    assert_eq!(actual_allow, expected_licenses());
    assert_eq!(
        licenses
            .get("private")
            .and_then(|value| value.get("ignore"))
            .and_then(toml::Value::as_bool),
        Some(true)
    );
}
