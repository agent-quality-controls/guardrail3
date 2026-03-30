use guardrail3_domain_modules::clippy::{library_profile_type_paths, service_profile_type_paths};
use test_support::build_fixture_clippy_toml;

#[test]
fn generated_service_types_do_not_contain_project_specific_extras() {
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
    let expected = service_profile_type_paths();

    assert_eq!(actual, expected);
}

#[test]
fn generated_library_types_do_not_misclassify_managed_global_state_entries_as_extra() {
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
    let expected = library_profile_type_paths();

    assert_eq!(actual, expected);
}
