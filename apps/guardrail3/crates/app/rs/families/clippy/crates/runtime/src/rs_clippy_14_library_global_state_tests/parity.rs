use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::library_profile_type_paths;

#[test]
fn generated_library_profile_contains_exact_managed_global_state_type_set() {
    let parsed = toml::from_str::<toml::Value>(&test_support::build_fixture_clippy_toml(
        "library", false, true, "", "",
    ))
    .expect("valid clippy TOML");
    let actual = parsed
        .get("disallowed-types")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("path").and_then(toml::Value::as_str))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let expected = library_profile_type_paths()
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
}
